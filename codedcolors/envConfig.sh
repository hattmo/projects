#!/usr/bin/env sh
set -eu

envsubst '${PORT}' < /template.conf > /etc/nginx/conf.d/default.conf

exec "$@"