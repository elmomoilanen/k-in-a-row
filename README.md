# k-in-a-row game

[![main](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/main.yml)

Play k-in-a-row game against a bot player on your machine.

Currently available game types are 3x3 3-in-a-row, 4x4 (4), 5x5 (4) and 6x6 (5). Furthermore, for each game type there are two levels of the bot player: easy and normal. In games with level normal, the outcome is always expected to be a draw at best for the human player. In easy games, the bot player might make bad moves time to time allowing the human player to win.

## Quick start

Make sure that `Docker` and `Docker Compose` are installed and available.

Run the following command

```bash
docker-compose up
```

and when it's ready (might take few minutes in first time) open `localhost:5173` via a web browser and start playing.

When finished with playing, press ctrl+c and remove the used containers with a command

```bash
docker-compose down
```

## Dev

It is also possible to play without Docker but in this case Rust and Svelte installations must be taken care manually.

Read more on this and development in general from README files in backend and frontend subdirectories.
