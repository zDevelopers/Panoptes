# Panoptès

_A Minecraft [Prism](https://www.spigotmc.org/resources/prism.75166/)-based tool to monitor shared in-game resources._

Panoptès (named from the many-eyed giant in the Greek mythology) is a web tool (with a Rust backend) used to monitor
players contributions to in-game shared resources, to check if everyone is playing fair.

It requires Prism to be installed in-game and connects to its MySQL database (read-only) to gather usage statistics.

## Backend

Panoptès backend is built with Rust nightly and [Rocket 5.0-dev](https://rocket.rs/master/).

### Installation

1. Install Rust nightly using [rustup](https://rustup.rs).
2. Create a file named `Panoptes.toml` at the root of the repository, following the template below. At least, add a
   database DSN.
2. ```make run-back```

The first run will be slow to start, as everything needs to be downloaded and compiled. Subsequent runs will be way
faster.

#### Configuration

To configure Panoptès, you can create a `Panoptes.toml` file with the following content (all parts are optional;
only add the ones you want to override). Default values can be found in the `back/Rocket.toml` file. You can set
in this file [any native Rocket configuration](https://rocket.rs/master/guide/configuration/#overview) too.

```toml
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

#### Documentation

API documentation is available at the `/` endpoint.
