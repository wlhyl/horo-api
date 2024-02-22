FROM node:18.17.1 as build-stage

WORKDIR /app
COPY ./ /app/

RUN npm install @ionic/cli -g --registry https://registry.npmmirror.com
RUN npm install --registry https://registry.npmmirror.com --canvas_binary_host_mirror=https://registry.npmmirror.com/mirrors/node-canvas-prebuilt

RUN ionic build --prod

FROM nginx:1.25.4-alpine

COPY --from=build-stage /app/www/ /usr/share/nginx/html

#Copy default nginx configuration
COPY ./nginx-custom.conf /etc/nginx/conf.d/default.conf
