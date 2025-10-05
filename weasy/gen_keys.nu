#!/bin/nu

rm -rf ./crypto
mkdir ./crypto
cd ./crypto
openssl genrsa -out ca.key 4096
openssl req -x509 -new -key ca.key -sha256 -days 3650 -out ca.crt -subj "/CN=CA"

openssl genrsa -out server.key 4096
openssl req -new -key server.key -out server.csr -subj "/CN=c2"
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 825 -sha256

openssl genrsa -out client.key 4096
openssl req -new -key client.key -out client.csr -subj "/CN=rat"
openssl x509 -req -in client.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out client.crt -days 825 -sha256

