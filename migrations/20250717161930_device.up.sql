-- sharding by device_id
CREATE TABLE device
(
    device_id        varchar(32) NOT NULL primary key,
    full_fingerprint jsonb       NOT NULL,
    platform_type    int         not NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON device
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- sharding by stable_fingerprint_hash
CREATE TABLE lookup_device
(
    stable_fingerprint_hash VARCHAR(64) PRIMARY KEY,
    device_id               varchar(32) NOT NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX unq_device_fingerprint ON lookup_device (stable_fingerprint_hash);


-- sharding by prefilter_hash
CREATE TABLE prefilter_device
(
    id                    BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    prefilter_hash        VARCHAR(64),

    device_model          VARCHAR(20) NOT NULL,
    platform_name         varchar(20) NOT NULL,
    os_name               VARCHAR(20) NOT NULL,
    screen_width          INT         not NULL, -- 屏幕宽度
    screen_height         INT         not NULL, -- 屏幕高度
    device_memory_gb      INT         not NULL, -- 设备内存 (GB)
    cpu_cores             INT         not NULL, -- CPU 核心数
    renderer_vendor       VARCHAR(50) not NULL, -- 渲染器供应商
    renderer_model        VARCHAR(50) not NULL, -- 渲染器型号

    browser_major_version VARCHAR(20) NULL,     -- 浏览器主要版本
    app_major_version     VARCHAR(20) NULL,     -- App 主要版本 (移动端)

    timezone_id           VARCHAR(10) NULL,     -- 时区 ID
    language_code         VARCHAR(10) NULL,     -- 语言代码
    ip_address_segment    INET        NULL,     -- IP 地址段

    device_id             varchar(32) NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_prefilter_hash ON prefilter_device (prefilter_hash);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON prefilter_device
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();