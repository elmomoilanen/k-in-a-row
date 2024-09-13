FROM node:20-alpine AS build

WORKDIR /app
COPY . .
COPY .env.example .env
RUN npm ci && ./node_modules/.bin/svelte-kit sync
RUN npm run build

FROM nginx:1.27-alpine

WORKDIR /usr/share/nginx/html
RUN rm -rf ./*
COPY --from=build /app/build .
CMD ["nginx", "-g", "daemon off;"]
