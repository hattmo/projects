FROM golang:1.15.5
ENV GOARCH=386
ENV GOOS=linux
WORKDIR /go
COPY inject.go .
RUN go get golang.org/x/net/html
RUN go build -o injector


FROM nginx:1.19.4
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY index.html /usr/share/nginx/html/
COPY docker-entrypoint.sh /
COPY --from=0 /go/injector /