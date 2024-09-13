# Frontend

This is the frontend service of the game, which communicates with the backend service to determine the bot player's moves on the game board.

Notice that the frontend must know the backend endpoint URL `PUBLIC_API_URL`. Set it directly as an environment variable, or make a copy of the `.env.example` file by running the command `cp .env.example .env`.

## Development

First install dependencies

```bash
npm install
```

and then start the dev server

```bash
npm run dev -- --open
```

If this command results an immediate error, check that `PUBLIC_API_URL` has been set correctly.

TypeScript support for reading environment variables requires that command `svelte-kit sync` has been run. Run for example the command `npm run check`, which includes that svelte-kit command.

## Production

Firebase hosting is used in production.

This directory has been connected to a Firebase project by running `firebase init` command. As a result, two public Firebase configuration files were added to this directory which are required when running the frontend deployment pipeline.

Prior to making deployment to Firebase, the built app needs to be adapted for this target. SvelteKit's static adapter is used to prerender this site as a collection of static files. This is configured in `svelte.config.js` and with the prerender option placed in the root layout file `+layout.ts`.

GitHub workflow `deploy-fe.yml` uploads a new release to Firebase for every versioned push event to the main branch. For this pipeline, `PUBLIC_API_URL` (backend's production URL) and `FIREBASE_SERVICE_ACCOUNT_K_INAROW` (Firebase service account credentials) must have been stored as secrets in GitHub's Actions secrets. Deployment job of the workflow runs conditionally after successful lints, type checks and tests.
