use std::fmt;

use itertools::Itertools;
use rocket::http::RawStr;
use rocket::request::{FromParam, FromFormValue};
use rocket_contrib::uuid::Uuid;

use uuid::Uuid as UuidReal;
use crate::area::Areas;


/// Represents a list of areas in a query string.
/// `all` is used when the query string is missing.
pub struct AreasIds {
    areas: Vec<String>,
    all: bool
}

impl AreasIds {
    pub fn all() -> Self {
        AreasIds {
            all: true,
            areas: vec![]
        }
    }
}

impl Areas {
    /// Filters out the areas, keeping only those matching the given list.
    pub fn filter(&self, list: AreasIds) -> Self {
        Areas {
            areas: self.areas.iter()
                .filter( | (id, _) | list.all || list.areas.contains(*id))
                .map(|(id, area)| (id.clone(), area.clone()))
                .collect()
        }
    }
}

impl<'v> FromFormValue<'v> for AreasIds {
    type Error = std::convert::Infallible;

    /// Implements the conversion from a query string (or the lack of one) to a list of areas.
    #[inline(always)]
    fn from_form_value(param: &'v RawStr) -> Result<Self, Self::Error> {
        Ok(Self {
            areas: param.split(',')
                .map(|part| String::from_param(part.into()))
                .filter_map(Result::ok)
                .collect(),
            all: false
        })
    }

    #[inline(always)]
    fn default() -> Option<Self> {
        Some(Self::all())
    }
}


/// Represents a list of UUIDs in a query string.
pub struct Uuids {
    pub uuids: Vec<Uuid>
}

impl Uuids {

    /// Generates an SQL WHERE clause to filter for these UUIDs, assuming that the `prism_players`
    /// table is aliased as `p`.
    pub fn as_sql(&self) -> String {
        self.uuids.iter()
            .map(|uuid| format!("HEX(p.player_uuid) = '{}'", uuid.to_simple().encode_lower(&mut UuidReal::encode_buffer())))
            .intersperse(String::from(" OR "))
            .collect()
    }
}

impl<'v> FromFormValue<'v> for Uuids {
    type Error = uuid::Error;

    /// Implements the conversion from a query string (or the lack of one) to a list of UUIDs.
    #[inline(always)]
    fn from_form_value(param: &'v RawStr) -> Result<Self, Self::Error> {
        let mut uuids = param.split(",")
            .map(|raw_uuid| Uuid::from_param(raw_uuid.into()))
            .collect::<Result<Vec<Uuid>, uuid::Error>>()?;

        uuids.sort();
        uuids.dedup();

        Ok(Uuids { uuids })
    }
}

impl fmt::Display for Uuids {

    /// The formatted version displays simple UUIDs comma-separated, and is used as a cache key.
    /// For any identical set of UUIDs the output should be the same; that's why UUIDs simple
    /// representations are sorted.
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}",
            self.uuids.iter()
                .map(|uuid| format!("{}", uuid.to_simple().encode_lower(&mut UuidReal::encode_buffer())))
                .sorted()
                .intersperse(String::from(", "))
                .collect::<String>()
        )
    }
}
