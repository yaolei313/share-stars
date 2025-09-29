use crate::biz::dto::AuthnMethod;
use crate::http::vo::{AppResult, RequestInfo};
use chrono::Utc;
use lib_core::db::models::AccountDevice;
use lib_core::db::services::AccountDbService;
use std::sync::Arc;

pub struct AccountDeviceService {
    account_db_service: Arc<AccountDbService>,
}

impl AccountDeviceService {
    pub fn new(account_db_service: Arc<AccountDbService>) -> Self {
        AccountDeviceService {
            account_db_service: account_db_service.clone(),
        }
    }

    pub async fn check_trusted_device(
        &self,
        user_id: i64,
        request_info: &RequestInfo,
    ) -> AppResult<bool> {
        let account_device = self
            .account_db_service
            .query_account_device(user_id, &request_info.device_id)
            .await?;
        Ok(self.is_trusted(account_device))
    }

    pub async fn save_new_account_device(
        &self,
        user_id: i64,
        device: &RequestInfo,
        auth_method: AuthnMethod,
    ) -> AppResult<()> {
        let db_device = AccountDevice {
            id: 0,
            user_id,
            device_id: device.device_id.clone(),
            last_login_ip: device.ip,
            last_login_method: auth_method.code(),
            last_login_at: Default::default(),
            nickname: None,
            trusted_at: None,
            expires_at: None,
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        self.account_db_service
            .add_account_device(&db_device)
            .await?;
        Ok(())
    }

    fn is_trusted(&self, device: Option<AccountDevice>) -> bool {
        let Some(ref device) = device else {
            return false;
        };
        let Some(ref trust_at) = device.trusted_at else {
            return false;
        };
        let now = Utc::now();
        if let Some(ref expired_at) = device.expires_at {
            trust_at.timestamp() <= now.timestamp() && now.timestamp() < expired_at.timestamp()
        } else {
            trust_at.timestamp() <= now.timestamp()
        }
    }
}

// 一、核心概念
// 设备指纹 (Device Fingerprinting): 通过收集浏览器和设备的各种可识别特征，组合成一个相对唯一的标识符。这些特征包括硬件信息、软件配置、网络环境等。
// 可信设备 (Trusted Device): 指经过用户明确授权或系统通过综合判断认为安全的设备。通常，在此设备上登录时可以跳过或简化额外的安全验证步骤。
// 非可信设备 (Untrusted Device): 指首次登录的设备、公共设备、或存在异常特征的设备。在此设备上登录时需要加强安全验证。
// 二、可信设备检测和匹配的策略
// 1. 初次标记为可信设备
// 用户主动授权： 在用户首次成功登录后（尤其是完成 MFA 后），在页面上提供一个选项，如“记住此设备”、“设为可信设备”、“下次登录免验证”。用户勾选后，系统会将当前设备的指纹保存到其账户信息中。
// 静默授权（低风险场景）： 对于低风险的操作，如果用户多次从同一设备成功登录，且没有异常行为，系统可以考虑在后台静默地将该设备标记为可信。
// 4. 可信设备的管理
// 用户界面： 提供一个用户友好的界面，让用户可以查看所有已标记为可信的设备列表，并可以随时撤销对某个设备的信任。
// 过期策略： 可信设备通常会设置一个有效期（例如 30 天、90 天）。过期后，即使设备相同，也需要重新进行身份验证。
// 异常行为撤销： 如果从某个可信设备检测到异常行为（如短时间内大量失败登录尝试、撞库攻击、账户被盗），系统应自动撤销该设备的可信状态。
// 三、设备指纹技术 (前端采集)
// 设备指纹的准确性和稳定性是其核心。Web 端主要通过 JavaScript 收集以下信息：
//
// 1. 硬件和系统特征
// User-Agent: 浏览器类型、版本、操作系统、CPU 架构。
// 屏幕分辨率和颜色深度: screen.width, screen.height, screen.colorDepth.
// CPU 核心数: navigator.hardwareConcurrency.
// 内存信息: navigator.deviceMemory (有限，通常只能获取到近似值)。
// 显卡信息: 通过 WebGL API 获取显卡制造商和渲染器信息 (renderer 和 vendor 字符串)。
// 2. 浏览器特征
// 浏览器插件列表: navigator.plugins (现代浏览器中限制较多，但仍有参考价值)。
// MIME 类型列表: navigator.mimeTypes。
// 字体列表: 通过 document.fonts 或 Canvas 渲染特定文本并检测像素差异来识别本地安装字体。
// Canvas 指纹: 在 <canvas> 元素上绘制特定图形、文本，然后将其转换为 Data URL 或像素数据。不同的显卡、驱动、字体、操作系统会产生微妙的像素差异，从而生成独一无二的指纹。这是非常强大的指纹识别技术。
// WebGL 指纹: 利用 WebGL 渲染能力，绘制复杂的 3D 图形，并从渲染结果中提取特征，比 Canvas 更强大。
// 音频指纹 (AudioContext Fingerprinting): 利用 Web Audio API 播放音频并分析音频渲染结果的微小差异。
// WebRTC IP 地址泄露: WebRTC 可能会暴露用户的真实 IP 地址（包括内网 IP），这可以作为指纹的一部分，但由于隐私问题，浏览器正在加强对其的限制。
// 时区和语言设置: Intl.DateTimeFormat().resolvedOptions().timeZone, navigator.language.

// 3. 网络和连接特征
// IP 地址: 后端从 HTTP 请求头获取。
// HTTP 请求头信息: 除了 User-Agent，还有 Accept-Language, Accept-Encoding, Connection 等。
// 代理/VPN 检测: 虽然难以直接从前端判断，但结合 IP 地址和用户行为模式可以辅助判断。
// 4. Cookie 和 Local Storage
// 持久化 ID: 在首次登录时设置一个长期的 Cookie 或 Local Storage 项作为设备 ID。这是最直接也是最脆弱的方式，因为它容易被用户清除。通常与其他指纹技术结合使用。
// 后端 API 开发：
// 设备指纹接收接口： 接收前端发送的设备特征数据。
// 指纹处理逻辑：
// 对接收到的原始特征进行清洗、标准化。
// 计算设备指纹（例如，将所有特征拼接后进行哈希）。
// 与数据库中存储的该用户账户的可信设备指纹进行比对。
// 根据比对结果，返回是否是可信设备的判断。
// 可信设备管理接口： 允许用户查看、添加、删除可信设备。
// 登录流程集成： 在用户登录时，调用设备指纹检测接口。根据检测结果，决定是否触发二次验证。
// 数据库设计：
// 创建一个 trusted_devices 表，存储用户ID、设备指纹、盐值（如果指纹哈希需要）、设备名称（用户可自定义）、添加时间、最后登录时间、过期时间等字段。
// 安全考虑：
// 指纹敏感性： 并非所有特征都同等重要。一些特征（如 IP 地址）经常变化，而另一些（如 Canvas/WebGL 指纹）相对稳定。
// 隐私保护： 告知用户设备指纹的使用目的，并提供管理选项。
// 对抗指纹伪造： 高级的攻击者可能尝试伪造指纹。系统需要不断更新指纹采集和识别算法，并结合行为分析（如登录地点、时间、频率）进行综合判断。
// 定期更新： 浏览器更新、操作系统更新等都可能导致设备指纹变化，需要允许一定的容错性或定期要求用户重新验证可信设备。
// 通过综合运用这些技术和策略，你可以建立一个较为完善的 Web 端可信设备检测和匹配系统，从而提升账户安全性并优化用户体验。
