CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE links (
    -- Change BIGSERIAL to UUID and use a generation function
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    short_code VARCHAR(10) NOT NULL UNIQUE,
    long_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delete_key TEXT NOT NULL
);
