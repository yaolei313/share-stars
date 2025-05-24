create table "passport"
(
    id              bigint       not null primary key default (uuid_generate_v4()),
    user_id         bigint       not null,
    username        varchar(32)  not null,
    phone           varchar(15)  not null             default '',
    email           varchar(255) not null             default '',
    salt            char(16)     not null,
    password_sha256 varchar(64)  not null,
    register_type   smallint     not null,
    register_at     timestamp    not null,
    closed          boolean      not null             default false,
    closed_at       timestamp,
    disabled        boolean      not null             default false,
    disabled_at     timestamp,
    created_at      timestamp                         default now(),
    updated_at      timestamp                         default now()
);

create index users_idx_email on users (email);-- Add up migration script here
