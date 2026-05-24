# ma-runtime – 中文（简体）
lang-name = 中文（简体）

own-did-published = 自身 DID 文档已发布至 IPNS
own-did-publish-failed = 发布自身 DID 文档失败
own-did-publish-timeout = 自身 DID 文档发布在 2 分钟后超时
started = ma runtime 已启动
shutdown-requested = 已请求关闭
closing-endpoint = 正在关闭 iroh 端点...
shutdown-complete = 关闭已完成
status-listening = 状态服务器正在监听
rpc-message-received = 已收到 RPC 消息
rpc-message-rejected = RPC 消息已被拒绝
ipfs-message-rejected = IPFS 消息已被拒绝
ctrlc-handler-failed = Ctrl-C 处理程序失败
node-connected = 节点已连接至协议
received-encrypted-ma-msg = 在 /ma/ipfs/0.0.1 上收到加密 ma 消息
unknown-rpc-atom = 未知 RPC 原子，忽略
rpc-not-text-atom = RPC 数据不是文本原子
rpc-unknown-verb = 未知 RPC 动词
rpc-reply-sent = RPC 回复已发送
ping-received = 收到 :ping，发送 :pong
did-publish-request-received = 收到 DID 文档发布请求
document-published = 文档已发布
did-publish-cid-reply-sent = 已为 DID 发布发送 CID 回复
did-publish-resolve-failed = 无法解析发送方以传递 ipfs-publish 回复
ipfs-store-request-received = 收到 IPFS 存储请求
ipfs-stored = 内容已存储至 IPFS
ipfs-store-cid-reply-sent = CID 回复已发送
ipfs-store-resolve-failed = 无法解析发送方以传递 ipfs-store 回复

# 实体分发
bootstrap-complete = Bootstrap 已完成
entity-loaded = 实体插件已加载
entity-load-failed = 实体插件加载失败
entity-not-found = 未找到实体，忽略 RPC
entity-dispatched = RPC 已分发至实体
entity-replied = 实体发送了 RPC 回复
root-create-entity = #root：创建实体
root-list-entities = #root：列出实体
root-delete-entity = #root：删除实体
root-entity-updated = Runtime 清单已更新
entity-created = 实体已创建
entity-deleted = 实体已删除
entity-states-saving = 正在将实体状态保存至 IPFS
entity-state-saving = 正在保存实体状态
entity-state-saved = 实体状态已保存
entity-state-empty = 插件返回空状态，跳过保存
entity-states-saved = 实体状态已保存
link-set = 链接已设置
ftl-loaded = 语言消息已从 IPFS 加载

# 首次启动 / 自动初始化
no-config-found = 未找到配置。
initialising-new-identity = 正在初始化新的 runtime 身份。
generated-headless-config = 已生成无头配置。

# 所有权
runtime-claimed = Runtime 已注册。

# 受保护的根元素
refuse-delete-root = 坚决拒绝删除所需根元素
no-root-acl = 未配置根 ACL — runtime 在无访问控制的情况下运行
namespace-not-found = 未找到命名空间
no-ns-gate-acl = 此命名空间未配置网关 ACL
runtime-claim-persisted = 所有者已写入配置。
runtime-already-claimed = Runtime 已注册。


# Namespace creation (:create)
acl-missing-owners = 根 ACL 必须包含 +owners 条目
namespace-created = 命名空间已创建
namespace-already-exists = 命名空间已存在
namespace-name-reserved = 命名空间名称已被保留
namespace-create-denied = 创建命名空间：访问被拒绝
namespace-create-usage = 用法：:create <名称>
crud-message-received = 收到 CRUD 消息
crud-acl-updated = 根传输 ACL 已更新
