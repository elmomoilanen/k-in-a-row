# k-in-a-row game

[![main fe](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml)
[![main be](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/main-be.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/main-be.yml)

Play k-in-a-row game against a bot player.

## Quick start local

Play the game in your machine with these instructions.

Make sure that `Docker` is installed and available.

Run the following command

```bash
docker compose up
```

and when it's ready (might take few minutes in the first time) open `localhost:5173` via a web browser and start playing.

When finished with playing, press ctrl+c and remove the used containers with a command

```bash
docker compose down
```

## Development

It is also possible to play without the Docker but in that case Rust and Svelte installations must be taken care of manually.

Read more on this and development in general from README files in backend and frontend subdirectories.

## Production
