FROM node:12
WORKDIR /app/
COPY ./tsconfig.json ./webpack.config.js ./package.json ./package-lock.json ./
RUN npm ci --ignore-scripts
COPY ./src ./src
RUN npx webpack --mode production

FROM nginx:stable
ENV PORT=80
COPY ./nginx.conf /template.conf
COPY ./envConfig.sh .
COPY --from=0 /app/dst/ /usr/share/nginx/html
ENTRYPOINT ["/envConfig.sh"]
CMD ["nginx", "-g", "daemon off;"]