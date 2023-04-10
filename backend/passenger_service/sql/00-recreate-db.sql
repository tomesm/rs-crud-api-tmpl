-- DEV ONLY. We re-create the DB at every save to make sure we have a clean state.
DROP DATABASE IF EXISTS passenger_service_db;
DROP USER IF EXISTS passenger_service_user;

CREATE USER passenger_service_user;
CREATE DATABASE passenger_service_db ENCODING = 'UTF-8';