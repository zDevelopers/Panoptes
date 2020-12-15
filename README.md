# Panoptès

_A Minecraft [Prism](https://www.spigotmc.org/resources/prism.75166/)-based tool to monitor shared in-game resources._

Panoptès (named from the many-eyed giant in the Greek mythology) is a web tool (with a Rust backend) used to monitor
players contributions to in-game shared resources, to check if everyone is playing fair.

It requires Prism to be installed in-game and connects to its MySQL database (read-only) to gather usage statistics.

Panoptès backend is built with Rust nightly and [Rocket 5.0-dev](https://rocket.rs/master/). Its frontend is built
using VueJS 2 and Vuetify.

## Install

1. Install Rust nightly using [rustup](https://rustup.rs), and Node 12+.
2. Run `make install` to install front-end dependencies.
3. Create a `Panoptes.toml` file at the root of the repository, following the template below. At least, add a
   database DSN.
4. Create a `front/.env.local` file with the following content. You can also use a real environment variable.
   ```
   VUE_APP_API_URL=[backend url]
   ```
   
### Translations support

To enable translations support—allowing the `display_name` key of the `/ratios` endpoint to contain localized data
according to the `locale` query parameter—you must extract the translation files from an existing Minecraft
installation, as distributing these files is forbidden.

To do so, execute `make extract-translations`. If it does not work, e.g. because your installation is non-standard,
see options by executing the underlying script with `--help`. The default locale can be specified in the configuration
file, alongside the folder where translations are stored (default to the `translations` folder at the root of the
application).

## Start

```
make run
```

The first run will be slow to start, as everything needs to be downloaded and compiled. Subsequent runs will be way
faster. You can use `make start-back` or `make start-front` to only start one part.

## Configure

To configure Panoptès, you can create a `Panoptes.toml` file with the following content (all parts are optional;
only add the ones you want to override). Default values can be found in the `back/Rocket.toml` file. You can set
in this file [any native Rocket configuration](https://rocket.rs/master/guide/configuration/#overview) too.

```toml
[global.minecraft_translations]

directory = "../translations"
default_locale = "fr_fr"

[global.areas]

# You can add as many areas as you wish in this section
your_area = { name = "Area name", world = "world", pos1 = [0, 0, 0], pos2 = [400, 256, 800] }

[global.databases.prism]

url = "mysql://user:password@host/database"
pool_size = 2
```

The location of this file can be modified using the `PANOPTES_CONFIG` environment variable.

You can also use environment variables, the content being a TOML string, e.g. to configure the database DSN:

```env
PANOPTES_DATABASES='{prism={url="mysql://dsn"}}' 
```

## Read (the manual)

API documentation is available at the `/` endpoint of the backend server.

## Deploy

TODO, but for the front-end part:

```
npm run build
```
