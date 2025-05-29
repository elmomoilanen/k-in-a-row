# Frontend

This is the frontend service of the game, which communicates with the backend service to determine the bot player's moves on the game board.

## Development

Ensure your local `node` version matches the version specified in the frontend workflows and the Dockerfile (which is primarily for quick testing, provided `Docker` is available).

Notice that the frontend must know the backend endpoint URL `PUBLIC_API_URL`. Set it directly as an environment variable, or make a copy of the `.env.example` file by running the command `cp .env.example .env`.

First install dependencies

```bash
npm install
```

For TS to correctly read environment variables, the `svelte-kit sync` command must be executed. To do that, run for example the following command

```bash
npm run check
```

After that start the dev server

```bash
npm run dev -- --open
```

## Production

Firebase hosting is used in production.

This directory has been connected to a Firebase project by running `firebase init` command. As a result of running `firebase init`, two public Firebase configuration files were added to this directory. These files are required for the frontend deployment pipeline.

Before deploying to Firebase, the built application must be adapted for the hosting environment. SvelteKit's static adapter is used to prerender this site as a collection of static files. This is configured in `svelte.config.js` and with the prerender option placed in the root layout file `+layout.ts`.

The `deploy-fe.yml` GitHub workflow automatically deploys a new release to Firebase whenever a versioned commit is pushed to the main branch. For this pipeline, `PUBLIC_API_URL` (backend's production URL) and `FIREBASE_SERVICE_ACCOUNT_K_INAROW` (Firebase service account credentials) must have been stored as secrets in GitHub's Actions secrets. The deployment job in the workflow runs only after the lints, type checks, and tests have passed successfully.

## Update guide

- Review and update the Node and Nginx image versions in the Dockerfile, if necessary
- Use locally the same Node version as in the Dockerfile
- Preview potential dependency updates, e.g. with `npm outdated`, and apply them by running first `npm update` and then `npm install` (ensures `package-lock.json` stays in sync with `package.json`)
