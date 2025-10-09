#!/bin/nu

rm -rf ./crypto
mkdir ./crypto
cd ./crypto
openssl genrsa -out ca.key 4096
openssl req -x509 -new -key ca.key -sha256 -days 3650 -out ca.crt -subj "/CN=CA"

openssl req -new -nodes -out server.csr -newkey rsa:4096 -keyout server.key --config ../server.cnf
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 825 -sha256 -extensions req_ext -extfile ../server.cnf

openssl req -new -nodes -out client.csr -newkey rsa:4096 -keyout client.key -config ../client.cnf
openssl x509 -req -in client.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out client.crt -days 825 -sha256 -extensions v3_req -extfile ../client.cnf

