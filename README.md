# k-in-a-row game

[![main fe](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-fe.yml)
[![main be](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-be.yml/badge.svg)](https://github.com/elmomoilanen/k-in-a-row/actions/workflows/tests-be.yml)

Play the k-in-a-row game against a bot player. You can choose from various board sizes, winning conditions, and game levels.

## Quick start

Play locally on your machine by following these instructions using `Docker`.

Run the following command to start the backend and frontend services

```bash
docker compose up
```

Once it's ready, open your web browser and navigate to `localhost:5173` to start playing.

When you are finished with playing, run the following command to stop and remove used containers and the network

```bash
docker compose down
```

For more thorough clean up, running following might help

```bash
docker compose down --rmi local
docker system prune
```

## Development

Refer to the README files in the backend and frontend subdirectories for more information.

## Production

Google's Firebase is used to host the frontend code, and Cloud Run is used to run the backend service. For more details, read the README files in the subdirectories and review the deployment workflows.
