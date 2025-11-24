-- Add up migration script here
-- ----------------------------
-- Table structure for sms_templates
-- ----------------------------
CREATE TABLE sms_template
(
    -- 基础标识
    id            bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    -- 核心查找字段：保证模板key和语言组合的唯一性
    template_key  VARCHAR(50) NOT NULL,              -- 模板唯一代码 (e.g., 'SMS_REG_CODE')
    language_code varCHAR(10) NOT NULL,              -- 语言代码 (e.g., 'zh-CN', 'en-US')

    -- 模板内容和类型
    content       TEXT        NOT NULL,              -- 短信正文内容，包含占位符
    template_type VARCHAR(20) NOT NULL,-- 业务用途分类 (e.g., 'registration', 'marketing')

    enabled       BOOLEAN     NOT NULL DEFAULT TRUE, -- 是否启用该模板
    created_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX unq_sms_template_key_lang
    ON sms_template (template_key, language_code);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON sms_template
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

CREATE TABLE email_template
(
    id            bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    template_key  VARCHAR(50)  NOT NULL,              -- 模板唯一代码 (e.g., 'EMAIL_WELCOME')
    language_code varCHAR(10)  NOT NULL,              -- 语言代码 (e.g., 'zh-CN', 'en-US')
    subject       VARCHAR(255) NOT NULL,              -- 邮件主题模板（包含占位符）
    content       TEXT         NOT NULL,              -- 邮件 HTML 正文（通常是主要内容）
    template_type VARCHAR(20)  NOT NULL,-- 业务用途分类 (e.g., 'registration', 'marketing')
    sender_name   VARCHAR(100),                       -- 默认发件人名称 (e.g., '您的服务团队')
    enabled       BOOLEAN      NOT NULL DEFAULT TRUE, -- 是否启用该模板
    created_at    timestamptz  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    timestamptz  NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX unq_email_template_key_lang
    ON email_template (template_key, language_code);
CREATE TRIGGER updated_at
    BEFORE UPDATE
    ON email_template
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();