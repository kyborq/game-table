CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    login TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    token TEXT NOT NULL
);