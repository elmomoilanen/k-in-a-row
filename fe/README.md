# Frontend

Frontend service for the game. Communicates with the backend service to determine bot player's moves in the game board.

Notice that the backend endpoint URL `API_URL` has been placed directly into the code. It must be updated if changes happen in the backend.

## Build and run

SvelteKit's static adapter is used to prerender this frontend site as a collection of static files. This is configured in `svelte.config.js` and with the prerender option placed to the root layout file `+layout.ts`. Nginx is used for serving the static files.

Build an image from the Dockerfile

```bash
docker build -t k-in-a-row/fe:latest .
```

and after that run the image

```
docker run -p 5173:80 --rm k-in-a-row/fe
```

Notice that the above port mapping should not be modified unless corresponding changes has been made in the backend (CORS settings).

Finally, open localhost:5173 via a web browser and start playing the game.

## Development

Install dependencies

```bash
npm install
```

and start the dev server

```bash
npm run dev -- --open
```
