# README

### postgres

```shell
docker run -itd -e POSTGRES_USER=stars -e POSTGRES_PASSWORD=stars123 -p 5432:5432 --name share-stars-postgres postgres
```

## rsa key

```shell
openssl genrsa -out private_key1.pem 2048
openssl rsa -in private_key1.pem -pubout -out public_key1.pem
openssl genrsa -out private_key2.pem 2048
openssl rsa -in private_key2.pem -pubout -out public_key2.pem
```

## sqlx

* sqlx database create
* sqlx migrate add -r users
* sqlx migrate run

## design

### 用户行为分析（User Behavior Analytics）或用户增长（User Growth）模块

1. 设备指纹组件 (Device Fingerprinting)  
   这是核心组件，负责：
    * 指纹生成: 通常是客户端（Web JavaScript SDK, Mobile SDK）的任务，但也可以指这个服务提供生成指纹的算法或库
    * 交换设备id：检查本地是否有设备id，若不存在，则使用设备指纹请求后端获取设备ID及设备id token
    * 设备ID存储: 将后端生成的设备ID存储，供客户端后续使用（通常存储在 LocalStorage, Cookie 或应用本地存储）

2. 访客识别/匿名用户管理 (Visitor Identification / Anonymous User Management)  
   这个模块负责：
    * 指纹收集与清洗: 接收来自客户端的原始指纹数据，进行标准化和初步清洗。
    * 设备ID生成与管理:
      基于指纹数据生成稳定、唯一的设备ID。这可能涉及复杂的算法，以应对指纹微小变化或对抗指纹欺骗。它会维护一个指纹-设备ID的映射关系，确保同一个设备的指纹（即使略有变化）也能对应到同一个设备ID。
    * 持久化: 将生成的设备ID及其相关指纹信息存储在数据库中（例如，我们之前讨论的 devices 表和 lookup_device 表）。
    * 设备ID下发: 将后端生成的设备ID返回给客户端，供客户端后续使用（通常存储在 LocalStorage, Cookie 或应用本地存储）。
    * 匿名会话关联: 将当前用户的匿名会话（Session）与获取到的设备ID关联起来，后续所有匿名行为数据都与这个设备ID绑定。
    * 匿名数据收集: 接收并存储这些匿名行为数据。

3. 用户行为分析 (User Behavior Analytics) / 产品分析 (Product Analytics)  
   这是最终消费设备ID和匿名行为数据的模块
    * 数据存储与处理: 存储与设备ID关联的匿名行为数据（例如，页面浏览、点击、停留时间、漏斗分析等）。这通常会用到数据仓库、数据湖或专业的分析平台。
    * 行为模式识别: 通过分析这些匿名行为，识别用户的兴趣、偏好、使用习惯、流失风险等。
    * 漏斗分析与归因: 用于分析用户在产品中的转化路径、发现瓶颈，并进行初步的渠道归因。
    * 个性化推荐: 尽管是匿名用户，也可以基于其匿名行为进行基础的个性化内容推荐或广告展示。
