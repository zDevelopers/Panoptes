use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Mutex, Arc};

use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::State;
use crate::config::TranslationsConfigInner;
use std::ops::Deref;


/// Stores all available Minecraft locales. Upon first use, actual data is not loaded.
#[derive(Debug, Clone)]
pub struct MinecraftLocales {
    pub locales: HashMap<String, Arc<MinecraftLocale>>,
    pub default_locale: String
}

impl MinecraftLocales {
    /// Returns an empty structure of locales, used when configuration is missing.
    /// If disabled, `display_name` keys will contain the non-prefixed Minecraft ID.
    pub fn empty() -> Self {
        MinecraftLocales {
            locales: HashMap::new(),
            default_locale: String::new()
        }
    }
}

impl From<TranslationsConfigInner> for MinecraftLocales {
    /// From the configuration, loads the translations and their metadata from the given folder.
    /// If an error occurs, returns an empty structure, effectively disabling translations support.
    fn from(config: TranslationsConfigInner) -> Self {
        match read_dir(config.directory.as_path()) {
            Ok(dir) => MinecraftLocales {
                locales: dir
                    .filter_map(|e| e.ok())
                    .filter_map(|dir_entry| {
                        match File::open(dir_entry.path()) {
                            Ok(file) => {
                                let reader = BufReader::new(file);
                                let json: serde_json::Result<HashMap<String, String>> = serde_json::from_reader(reader);
                                if let Ok(json) = json {
                                    if let Some(locale) = json.get("language.code") {
                                        return Some((locale.to_lowercase(), Arc::new(MinecraftLocale {
                                            file: dir_entry.path(),
                                            translations: Mutex::new(None),
                                        })))
                                    }
                                }
                                None
                            }
                            Err(_) => None
                        }
                    })
                    .collect(),
                default_locale: config.default_locale.to_lowercase()
            },
            Err(e) => {
                eprintln!("Unable to read translations directory. Disabling translations support.\n{}", e);
                MinecraftLocales::empty()
            }
        }
    }
}


/// Stores items translations for a locale. When the application starts, only the file path
/// is stored. Translations are lazy-loaded at first use.
/// To retrieve the correct locale for a given request, a request guard is available. See the
/// [`Locale`] guard.
#[derive(Debug)]
pub struct MinecraftLocale {
    pub file: PathBuf,
    translations: Mutex<Option<HashMap<String, String>>>,
}

impl MinecraftLocale {
    /// Loads translations from the file.
    fn load(&self) -> Result<HashMap<String, String>, String> {
        match File::open(&self.file) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let json: serde_json::Result<HashMap<String, String>> = serde_json::from_reader(reader);
                match json {
                    Ok(json) => Ok(
                        json.into_iter()
                            .filter(|(key, _)| key.starts_with("block.") || key.starts_with("item."))
                            .map(|(key, translation)| (key
                                                           .replace("block.minecraft.", "")
                                                           .replace("block.", "")
                                                           .replace("item.minecraft.", "")
                                                           .replace("item.", ""),
                                translation
                            ))
                            .collect()
                    ),
                    Err(err) => Err(format!("Unable to parse locale file {:?}: {}", self.file, err))
                }
            },
            Err(err) => Err(format!("Unable to load locale file {:?}: {}", self.file, err))
        }
    }

    pub fn translate(&self, key: String) -> String {
        // We either use the data read-only or replace it completely (if empty), so
        // it's safe to accept poisoned mutexes.
        let mut translations = match self.translations.lock() {
            Ok(translations) => translations,
            Err(poisoned) => poisoned.into_inner()
        };

        // We need to load translations
        if let None = *translations {
            *translations = Some(match self.load() {
                Ok(translations) => translations,
                Err(err) => {
                    eprintln!("Error while loading translations. {}", err);
                    HashMap::new()
                }
            })
        }

        match (*translations).as_ref() {
            Some(translations) => translations.get(&key).unwrap_or(&key).clone(),
            None => key
        }
    }

    fn empty() -> Self {
        Self {
            file: PathBuf::new(),
            translations: Mutex::new(Some(HashMap::new()))
        }
    }
}

/// A request guard to retrieve the requested locale data, or the default one if none was requested.
/// This guard dereferences to the [`MinecraftLocale`] containing the translations to use.
///
/// ```rust
/// #[get("/localized_path")]
/// fn localized_path(locale: Locale) {
///     let translated = locale.translate(String::from("purple_stained_glass_pane"));
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Locale {
    pub locale: Arc<MinecraftLocale>
}

impl Deref for Locale {
    type Target = Arc<MinecraftLocale>;

    fn deref(&self) -> &Self::Target {
        &self.locale
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for Locale {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let locales = try_outcome!(request.guard::<State<'r, MinecraftLocales>>().await);

        // The requested locale is extracted from the `?locale=` query string.
        let requested_locale = match request.get_query_value::<String>("locale") {
            Some(locale) => match locale {
                Ok(locale) => locale.clone(),
                Err(_) => locales.default_locale.clone()
            },
            None => locales.default_locale.clone()
        }.to_lowercase();

        // We try to load the requested locale, then the default locale, then if it was improperly
        // configured, an empty dummy locale structure that returns the translation keys.
        match locales.locales.get(&requested_locale) {
            Some(locale) => Outcome::Success(Locale {
                locale: Arc::clone(locale)
            }),
            None => match locales.locales.get(&locales.default_locale) {
                Some(locale) => Outcome::Success(Locale {
                    locale: Arc::clone(locale)
                }),
                None => Outcome::Success(Locale {
                    locale: Arc::new(MinecraftLocale::empty())
                })
            }
        }
    }
}
