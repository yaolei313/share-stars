create table "account"
(
    user_id       bigint      not null primary key,
    salt          char(16)    not null default '',
    password_hash varchar(64) not null default '',
    closed        boolean     not null default false, -- 软删除
    closed_at     timestamptz,                        -- 软删除时间
    disabled      boolean     not null default false, -- 临时禁用
    disabled_at   timestamptz,                        -- 临时禁用时间
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now()
);
comment on table account is '按userId分表';
create unique index unq_user_id on account (user_id);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON account
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

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
create unique index unq_user_id_provider on account_identity (user_id, provider);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON account_identity
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE "account_device"
(
    id                bigint GENERATED ALWAYS AS IDENTITY,
    user_id           bigint       NOT NULL,
    device_id         varchar(32)  NOT NULL,
    last_login_ip     INET         NULL,     -- 上一次登陆ip
    last_ip_location  varchar(32)  null,     -- 上一次登陆ip位置
    last_login_method int          not null, -- 上一次登陆方式
    last_login_at     TIMESTAMPTZ  NOT NULL, -- 上一次登陆时间
    nickname          VARCHAR(100) NULL,     -- 自定义设备名称
    trusted_at        TIMESTAMPTZ  NULL,     -- 添加为可信设备的时间
    expires_at        TIMESTAMPTZ  NULL,     -- 可信设备的过期时间
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    primary key (id)
);
comment on table account_device is '按userId分表';
CREATE UNIQUE INDEX unq_user_id_device_id on account_device (user_id, device_id);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON account_device
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

create table "lookup_account"
(
    id         bigint GENERATED ALWAYS AS IDENTITY,
    identifier varchar(255) not null, -- 邮箱地址, 手机号, OpenID
    provider   int          not null, -- email, phone, google
    user_id    bigint       not null,
    primary key (id)
);
comment on table lookup_account is '按identifier分表';
create unique index unq_identifier_provider on lookup_account (identifier, provider);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON lookup_account
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();