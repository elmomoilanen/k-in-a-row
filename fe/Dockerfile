FROM node:20-alpine as build

WORKDIR /app
COPY . .
RUN npm ci && npm run build

FROM nginx:1.25-alpine

WORKDIR /usr/share/nginx/html
RUN rm -rf ./*
COPY --from=build /app/build .
CMD ["nginx", "-g", "daemon off;"]
