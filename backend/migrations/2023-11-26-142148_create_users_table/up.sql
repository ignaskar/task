create table if not exists users (
    id uuid primary key,
    name text not null,
    email text unique not null,
    password_hash bytea not null
);