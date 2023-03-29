# Frontend

Frontend service for the game. Communicates with the backend service to determine bot player's moves in the game board.

Notice that frontend must know the backend endpoint URL `PUBLIC_API_URL`. Either set it directly as an environment variable or make a copy of the `.env.example` file as follows `cp .env.example .env`.

## Quick build and run

SvelteKit's static adapter is used to prerender this frontend site as a collection of static files. This is configured in `svelte.config.js` and with the prerender option placed to the root layout file `+layout.ts`. Nginx is used for serving the static files.

Build an image from the Dockerfile

```bash
docker build -t k-in-a-row/fe:latest .
```

and after that run the image

```bash
docker run -p 5173:80 --rm k-in-a-row/fe
```

Notice that the above port mapping should not be modified unless corresponding changes have been made in the backend (e.g. CORS settings).

Finally, open `localhost:5173` via a web browser and start playing the game.

## Development

Install dependencies

```bash
npm install
```

and start the dev server

```bash
npm run dev -- --open
```

TypeScript support for reading environment variables requires that command `svelte-kit sync` has been run. To do this, run for example the command `npm run check` which includes this svelte-kit command.

## Production

Firebase hosting is used in production.

This directory has been connected to a Firebase project by running `firebase init` command. As a result of running the command, two public Firebase configuration files were added to this directory which are required when running the frontend deployment pipeline.

GitHub workflow `deploy-fe.yml` uploads a new release to Firebase for every versioned push event to the main branch. For this pipeline, `PUBLIC_API_URL` (backend's production URL) and `FIREBASE_SERVICE_ACCOUNT_K_INAROW` (Firebase service account credentials) must have been stored as secrets in GitHub's Actions secrets.
