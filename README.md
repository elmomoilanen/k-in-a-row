# k-in-a-row game

[![main fe](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml)
[![main be](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-be.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-be.yml)

Play k-in-a-row game against a bot player.

## Quick start

Play locally in your machine with these instructions using `Docker`.

Run the following command to start services

```bash
docker compose up
```

and when it's ready open `localhost:5173` via a web browser and start playing.

When finished with playing, press ctrl+c and remove the used containers

```bash
docker compose down
```

## Development

See instructions on README files in backend and frontend subdirectories.

## Production

Google's Firebase is used to host the frontend code and Cloud Run to run the backend service. Read more from the subdirectory README files and deployment GitHub workflows.
