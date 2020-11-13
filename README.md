# Octopus Operating System

## Build

1. Install [Docker](https://docs.docker.com/get-docker/) (with [Docker Compose](https://docs.docker.com/compose/install/)).

2. Cd to the project directory.

3. Set `TARGER_ARCH` environment variable to `aarch64` or `x86_64`.

4. Run `docker-compose --env-file docker/$TARGER_ARCH.env run dev build.sh`.

## Run in QEMU

1. Run `docker-compose --env-file docker/$TARGER_ARCH.env run dev run.sh`.

## Blog

See my development [blog](https://octopus-os.blogspot.com/).
