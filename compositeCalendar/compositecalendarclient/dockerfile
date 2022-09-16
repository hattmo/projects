FROM node:12
WORKDIR /app/
COPY ./tsconfig.json ./
COPY ./webpack.config.ts ./
COPY ./package.json ./
COPY ./package-lock.json ./
RUN npm ci --ignore-scripts
COPY ./src ./src
RUN npx webpack --mode production

FROM nginx:stable
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=0 /app/dist/ /usr/share/nginx/html
