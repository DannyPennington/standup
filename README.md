# Standup
___
I'm practicing my basic rust & javascript by making a basic app for facilitating standups.
It uses a simple webserver using Rust's actix-web library and Askama for html templating. It's also set up to run in Docker, instructions to build are below.
___
#Prerequisites:
- `make` utility (available by default on unix-like operating systems)
- Docker (easily installed from https://www.docker.com/get-started or your package manager of choice)

Once those are installed all you need to do is navigate to the repo's root and run 
~~~
make
~~~
This will run the docker commands required to build the container