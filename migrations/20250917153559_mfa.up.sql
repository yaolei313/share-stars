CREATE TABLE "account_config"
(
    -- 二次认证的方式，bitset
);


CREATE TABLE "account_secret"
(
    id         bigint GENERATED ALWAYS AS IDENTITY,
    user_id    bigint      NOT NULL,
    type       int         not null, -- 类型，TOTP(Time-based One-Time Password),Passkeys
    secret     bytea       NOT NULL, -- 密钥

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    primary key (id)
);
comment on table account_device is '按userId分表';
CREATE UNIQUE INDEX unq_user_id_type on account_secret (user_id, type);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON account_secret
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();-- Add up migration script here
