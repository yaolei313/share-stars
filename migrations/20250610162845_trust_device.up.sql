-- 创建 trusted_device 表
CREATE TABLE trusted_device
(
    id              bigint GENERATED ALWAYS AS IDENTITY,
    user_id         bigint       NOT NULL,
    device_fp_hash  VARCHAR(255) NOT NULL,
    device_name     VARCHAR(255) NULL,
    device_platform int NULL,
    os_family       VARCHAR(20) NULL,
    os_version      VARCHAR(20) NULL,
    browser_family  VARCHAR(20) NULL,
    browser_version VARCHAR(20) NULL,
    app_family      VARCHAR(20) null,
    app_version     VARCHAR(20) null,
    last_login_ip   INET NULL,
    last_login_at   TIMESTAMPTZ  NOT NULL,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    expires_at      TIMESTAMPTZ NULL,
    is_active       boolean      NOT NULL DEFAULT TRUE,

    primary key (id),
    -- 复合唯一约束，确保同一个用户不会有重复的设备指纹记录
    CONSTRAINT unq_user_device_fingerprint UNIQUE (device_fp_hash, user_id)
);

-- 为 user_id 列创建索引
CREATE INDEX idx_trusted_device_user_id ON trusted_device (user_id);

-- 添加表注释
COMMENT
ON TABLE trusted_device IS '存储用户已标记为可信的设备信息';
COMMENT
ON COLUMN trusted_device.device_fp_hash IS '设备的唯一指纹的哈希值';
COMMENT
ON COLUMN trusted_device.device_name IS '用户为设备设置的名称（例如：“我的 MacBook Pro”，“家里的台式机”）';
COMMENT
ON COLUMN trusted_device.device_platform IS '设备平台（例如：Web, Android, Ios, Pc）';
COMMENT
ON COLUMN trusted_device.os_family IS '操作系统家族（例如：Windows, macOS, iOS, Android, Linux）';
COMMENT
ON COLUMN trusted_device.os_version IS '操作系统版本';
COMMENT
ON COLUMN trusted_device.browser_family IS '浏览器家族（例如：Chrome, Firefox, Safari, Edge）';
COMMENT
ON COLUMN trusted_device.browser_version IS '浏览器版本';
COMMENT
ON COLUMN trusted_device.app_family IS 'app家族';
COMMENT
ON COLUMN trusted_device.app_version IS 'app版本';
COMMENT
ON COLUMN trusted_device.last_login_ip IS '最后一次登录时使用的IP地址';
COMMENT
ON COLUMN trusted_device.last_login_at IS '最后一次从此设备成功登录的时间';
COMMENT
ON COLUMN trusted_device.created_at IS '标记为可信设备的时间';
COMMENT
ON COLUMN trusted_device.expires_at IS '可信状态的过期时间（例如：90 天后失效），NULL 表示永不失效（不推荐）';
COMMENT
ON COLUMN trusted_device.is_active IS '设备是否可信（手动撤销或系统自动撤销）';



