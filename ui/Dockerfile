FROM node:20.14.0 as build-stage

WORKDIR /app
COPY ./ /app/

RUN npm install @ionic/cli -g --registry https://registry.npmmirror.com
RUN npm install --registry https://registry.npmmirror.com --canvas_binary_host_mirror=https://registry.npmmirror.com/mirrors/node-canvas-prebuilt

RUN ionic build --prod

RUN gzip /app/www/*js && gzip /app/www/*css #  && gzip /app/www/*html

FROM nginx:1.27.0-alpine

# RUN rm -rf  /usr/share/nginx/html/*

COPY --from=build-stage /app/www/ /usr/share/nginx/html

#Copy default nginx configuration
COPY ./nginx-custom.conf /etc/nginx/conf.d/default.conf
