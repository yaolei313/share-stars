create table "account"
(
    id            bigint GENERATED ALWAYS AS IDENTITY,
    user_id       bigint      not null,
    salt          char(16)    not null default '',
    password_hash varchar(64) not null default '',
    closed        boolean     not null default false, -- 软删除
    closed_at     timestamptz,                        -- 软删除时间
    disabled      boolean     not null default false, -- 临时禁用
    disabled_at   timestamptz,                        -- 临时禁用时间
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now(),
    primary key (id)
);
comment on table account is '按userId分表';
create unique index account_unq_user_id on account (user_id);

create table "account_identity"
(
    id          bigint GENERATED ALWAYS AS IDENTITY,
    user_id     bigint       not null,
    provider    int          not null, -- email, phone, google
    identifier  varchar(255) not null, -- 邮箱地址, 手机号, OpenID
    is_verified bool         not null, -- 是否已验证
    verified_at timestamptz,           -- 验证时间
    created_at  timestamptz  NOT NULL DEFAULT NOW(),
    updated_at  timestamptz  NOT NULL DEFAULT NOW(),
    primary key (id)
);
comment on table account_identity is '按userId分表';
create unique index account_identity_unq_user_id_provider on account_identity (user_id, provider);


create table "lookup_account"
(
    id         bigint GENERATED ALWAYS AS IDENTITY,
    identifier varchar(255) not null, -- 邮箱地址, 手机号, OpenID
    provider   int          not null, -- email, phone, google
    user_id    bigint       not null,
    primary key (id)
);
comment on table lookup_account is '按identifier分表';
create unique index lookup_account_unq_identifier_provider on lookup_account (identifier, provider);
