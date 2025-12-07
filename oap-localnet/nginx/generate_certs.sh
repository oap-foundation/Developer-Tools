#!/bin/bash
mkdir -p certs
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout certs/relay.key \
    -out certs/relay.crt \
    -subj "/C=US/ST=Dev/L=Local/O=OAP/CN=*.local"
echo "Certificates generated in certs/"
