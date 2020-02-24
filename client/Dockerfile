# ===================
# Build
# ===================
FROM node:alpine3.11 as build

WORKDIR /app

ENV PATH /app/node_modules/.bin:$PATH

COPY package.json /app/package.json

RUN npm install
RUN npm install -g @angular/cli

COPY . /app

RUN ng build --outputPath=dist --prod --aot

# ===================
# Nginx
# ===================
FROM nginx:1.16.0-alpine

# RUN rm /usr/etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html

CMD ["nginx", "-g", "daemon off;"]