FROM node:22-alpine AS build

ARG PUBLIC_API_URL_ARG
ENV PUBLIC_API_URL=$PUBLIC_API_URL_ARG

WORKDIR /app
COPY . .
RUN npm ci && ./node_modules/.bin/svelte-kit sync
RUN npm run build

FROM nginx:1.28-alpine

WORKDIR /usr/share/nginx/html
RUN rm -rf ./*
COPY --from=build /app/build .
CMD ["nginx", "-g", "daemon off;"]
