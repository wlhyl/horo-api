FROM node:18.17.1 as build-stage

WORKDIR /app
COPY ./ /app/

RUN npm install @ionic/cli -g --registry https://registry.npm.taobao.org
RUN npm install --registry https://registry.npm.taobao.org --canvas_binary_host_mirror=https://npm.taobao.org/mirrors/node-canvas-prebuilt/

RUN ionic build --prod

FROM nginx:1.25.2-alpine

COPY --from=build-stage /app/www/ /usr/share/nginx/html

#Copy default nginx configuration
COPY ./nginx-custom.conf /etc/nginx/conf.d/default.conf
