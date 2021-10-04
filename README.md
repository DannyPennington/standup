# Standup
___
I'm practicing my rust & javascript by making a basic app for facilitating standups.
It uses a simple webserver built on Rust's actix-web library and Askama for html templating. It's also set up to run in Docker, instructions to build are below.

Names are pulled from a TOML config file in the project root (see `nw.toml` or `ta.toml` for examples)
You need to point the program to the config either via setting a `TEAM` environment variable or by providing the filename as an argument at runtime
e.g. `cargo run ta` will search the project root for `ta.toml` and try to read it
___
# Prerequisites:
- `make` utility (available by default on unix-like operating systems)
- Docker (easily installed from https://www.docker.com/get-started or your package manager of choice)

Once those are installed all you need to do is navigate to the repo's root and run 
~~~
make start
~~~
This will run the docker commands required to build the container and run the server on port 4200

The Makefile contains a couple of other targets which are all pretty self-explanatory, the names aren't perfect but hey ho.
