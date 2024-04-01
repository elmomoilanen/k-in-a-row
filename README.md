# k-in-a-row game

[![main fe](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml)
[![main be](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-be.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-be.yml)

Play k-in-a-row game against a bot player. You can choose from various board sizes, winning conditions and game levels.

## Quick start

Play locally in your machine with these instructions using `Docker`.

Run the following command to start backend and frontend services

```bash
docker compose up
```

and when it's ready open your web browser and navigate to `localhost:5173` to start playing.

When finished with playing, press ctrl+c and remove the used containers

```bash
docker compose down
```

## Development

See the README files in the backend and frontend subdirectories.

## Production

Google's Firebase is used to host the frontend code and Cloud Run to run the backend service. Read more from the subdirectory README files and deployment workflows.
