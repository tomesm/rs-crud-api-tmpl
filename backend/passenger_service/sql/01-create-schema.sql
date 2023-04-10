-- passenger 
CREATE TABLE passenger (
    id bigserial PRIMARY KEY,
    uid bigint, -- user id
    first_name STRING NOT NULL,
    last_name STRING NOT NULL,
    status STRING
);
-- ALTER SEQUENCE passenger_id_seq RESTART WITH 1000; -- start at 1000 for dev/debug