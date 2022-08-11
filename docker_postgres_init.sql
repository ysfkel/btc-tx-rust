DROP DATABASE IF EXISTS myblocksdb;

CREATE DATABASE myblocksdb;

CREATE USER docker WITH PASSWORD 'docker';

GRANT ALL ON DATABASE myblocksdb TO docker;