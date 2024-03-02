FROM nginx:alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY phebe/dist /usr/share/nginx/html
