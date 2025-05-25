-- Add up migration script here
create type user_role as enum('admin','user');

create extension if not exists "uuid-ossp";

create table "user" (
    id UUID not null primary key default (uuid_generate_v4()),
    name varchar(100) not null,
    email varchar(255) not null,
    password varchar(100) not null,
    verified boolean not null default false,
    verification_token varchar(255),
    token_expires_at timestamp with time zone,
    role user_role not null default 'user',
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create index user_idx_email on "user"(email);