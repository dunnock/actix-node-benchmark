#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER sped WITH PASSWORD 'sped';
    CREATE DATABASE sped;
    GRANT ALL PRIVILEGES ON DATABASE sped TO sped;
EOSQL

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "sped" -f /docker-entrypoint-initdb.d/create