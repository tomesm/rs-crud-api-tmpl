-- passenger 
CREATE TABLE passenger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    uid UUID NOT NULL,
    first_name STRING NOT NULL,
    last_name STRING NOT NULL,
    status STRING
);