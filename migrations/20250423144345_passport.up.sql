create table "passport"
(
    id              bigint GENERATED ALWAYS AS IDENTITY,
    user_id         bigint       not null,
    phone           varchar(15)  not null default '',
    email           varchar(255) not null default '',
    salt            char(16)     not null default '',
    password_sha256 varchar(64)  not null default '',
    closed          boolean      not null default false,
    closed_at       timestamptz,
    disabled        boolean      not null default false,
    disabled_at     timestamptz,
    created_at      timestamptz  not null default now(),
    updated_at      timestamptz  not null default now()
);

create unique index passport_unq_user_id on passport (user_id);


create table "phone_mapping"
(
    id      bigint GENERATED ALWAYS AS IDENTITY,
    phone   varchar(15) not null,
    user_id bigint      not null
);

create unique index phone_mapping_unq_phone on phone_mapping (phone);

