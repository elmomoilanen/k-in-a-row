FROM node:18-alpine as build

WORKDIR /app
COPY . .
RUN npm ci
RUN npm run build

FROM nginx:1.23-alpine

WORKDIR /usr/share/nginx/html
RUN rm -rf ./*
COPY --from=build /app/build .
CMD ["nginx", "-g", "daemon off;"]
