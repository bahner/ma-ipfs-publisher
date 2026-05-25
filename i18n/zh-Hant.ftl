# ma-runtime – 中文（繁體）
lang-name = 中文（繁體）

own-did-published = 自身 DID 文件已發布至 IPNS
own-did-publish-failed = 發布自身 DID 文件失敗
own-did-publish-timeout = 自身 DID 文件發布在 2 分鐘後逾時
started = ma runtime 已啟動
shutdown-requested = 已請求關閉
closing-endpoint = 正在關閉 iroh 端點...
shutdown-complete = 關閉已完成
status-listening = 狀態伺服器正在監聽
rpc-message-received = 已收到 RPC 訊息
rpc-message-rejected = RPC 訊息已被拒絕
ipfs-message-rejected = IPFS 訊息已被拒絕
ctrlc-handler-failed = Ctrl-C 處理程式失敗
node-connected = 節點已連線至協定
received-encrypted-ma-msg = 在 /ma/ipfs/0.0.1 上收到加密 ma 訊息
unknown-rpc-atom = 未知 RPC 原子，忽略
rpc-not-text-atom = RPC 資料不是文字原子
rpc-unknown-verb = 未知 RPC 動詞
rpc-reply-sent = RPC 回覆已發送
ping-received = 收到 :ping，發送 :pong
did-publish-request-received = 收到 DID 文件發布請求
document-published = 文件已發布
did-publish-cid-reply-sent = 已為 DID 發布發送 CID 回覆
did-publish-resolve-failed = 無法解析發送方以傳遞 ipfs-publish 回覆
ipfs-store-request-received = 收到 IPFS 儲存請求
ipfs-stored = 內容已儲存至 IPFS
ipfs-store-cid-reply-sent = CID 回覆已發送
ipfs-store-resolve-failed = 無法解析發送方以傳遞 ipfs-store 回覆

# 實體分發
bootstrap-complete = Bootstrap 已完成
entity-loaded = 實體外掛程式已載入
entity-load-failed = 實體外掛程式載入失敗
entity-not-found = 未找到實體，忽略 RPC
entity-dispatched = RPC 已分發至實體
entity-replied = 實體發送了 RPC 回覆
root-create-entity = #root：建立實體
root-list-entities = #root：列出實體
root-delete-entity = #root：刪除實體
root-entity-updated = Runtime 清單已更新
entity-created = 實體已建立
entity-deleted = 實體已刪除
entity-states-saving = 正在將實體狀態儲存至 IPFS
entity-state-saving = 正在儲存實體狀態
entity-state-saved = 實體狀態已儲存
entity-state-empty = 外掛程式傳回空狀態，略過儲存
entity-states-saved = 實體狀態已儲存
link-set = 連結已設定
ftl-loaded = 語言訊息已從 IPFS 載入

# 首次啟動 / 自動初始化
no-config-found = 未找到設定。
initialising-new-identity = 正在初始化新的 runtime 身分。
generated-headless-config = 已產生無頭設定。

# 所有權
runtime-claimed = Runtime 已註冊。

# 受保護的根元素
refuse-delete-root = 堅決拒絕刪除所需根元素
no-root-acl = 未設定根 ACL — runtime 在無存取控制的情況下執行
acl-owners-access = 呼叫方以 +owners 成員身份獲得存取權限
namespace-not-found = 未找到命名空間
no-ns-gate-acl = 此命名空間未設定閘道 ACL
runtime-claim-persisted = 擁有者已寫入設定。
runtime-already-claimed = Runtime 已註冊。


# Namespace creation (:create)
namespace-created = 命名空間已建立
namespace-already-exists = 命名空間已存在
namespace-name-reserved = 命名空間名稱已被保留
namespace-create-denied = 建立命名空間：存取被拒絕
namespace-create-usage = 用法：:create <名稱>
crud-message-received = 收到 CRUD 訊息
crud-acl-updated = 根傳輸 ACL 已更新

# CRUD validation errors
blob-value-ipfs-path = blob 值必須是 IPFS 路徑（/ipfs/、/ipns/ 或 /ipld/）
acl-value-ipfs-path = ACL 值必須是 IPFS 路徑（/ipfs/、/ipns/ 或 /ipld/）
kind-value-ipfs-path = kind 值必須是 IPFS 路徑（/ipfs/、/ipns/ 或 /ipld/）
kind-not-found = 未找到類型
cidv1-required = 值必須是裸 CIDv1（以 'b' 開頭；CIDv0 'Qm…' 不被接受）
config-key-protected = config 鍵 '%key%' 受保護
config-key-no-delete = 無法刪除 daemon config 鍵 '%key%'
config-key-not-manifest = config 鍵 '%key%' 不是已知的 manifest config 鍵
wrong-crud-protocol = 錯誤的 CRUD 協議：%type%
entity-name-invalid = entity 名稱必須是可列印的 UTF-8
reserved-entity-name = entity 名稱 '%name%' 已被保留
