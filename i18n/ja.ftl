# ma-runtime – 日本語
lang-name = 日本語

own-did-published = 自身の DID ドキュメントを IPNS に公開しました
own-did-publish-failed = 自身の DID ドキュメントの公開に失敗しました
own-did-publish-timeout = 自身の DID ドキュメントの公開が 2 分後にタイムアウトしました
started = ma runtime が起動しました
shutdown-requested = シャットダウンが要求されました
closing-endpoint = iroh エンドポイントを閉じています...
shutdown-complete = シャットダウンが完了しました
status-listening = ステータスサーバーがリッスン中です
rpc-message-received = RPC メッセージを受信しました
rpc-message-rejected = RPC メッセージが拒否されました
ipfs-message-rejected = IPFS メッセージが拒否されました
ctrlc-handler-failed = Ctrl-C ハンドラーが失敗しました
node-connected = ノードがプロトコルに接続しました
received-encrypted-ma-msg = /ma/ipfs/0.0.1 で暗号化された ma メッセージを受信しました
unknown-rpc-atom = 不明な RPC アトム、無視します
rpc-not-text-atom = RPC データはテキストアトムではない
rpc-unknown-verb = 未知の RPC 動詞
rpc-reply-sent = RPC 返信を送信しました
ping-received = :ping を受信しました、:pong を送信します
did-publish-request-received = DID ドキュメントの公開リクエストを受信しました
document-published = ドキュメントを公開しました
did-publish-cid-reply-sent = DID 公開の CID 返信を送信しました
did-publish-resolve-failed = ipfs-publish 返信の配信のために送信者を解決できませんでした
ipfs-store-request-received = IPFS ストレージリクエストを受信しました
ipfs-stored = コンテンツを IPFS に保存しました
ipfs-store-cid-reply-sent = CID 返信を送信しました
ipfs-store-resolve-failed = ipfs-store 返信の配信のために送信者を解決できませんでした

# エンティティのディスパッチ
bootstrap-complete = Bootstrap が完了しました
entity-loaded = エンティティプラグインを読み込みました
entity-load-failed = エンティティプラグインの読み込みに失敗しました
entity-not-found = エンティティが見つかりません、RPC を無視します
entity-dispatched = RPC をエンティティにディスパッチしました
entity-replied = エンティティが RPC 返信を送信しました
root-create-entity = #root: エンティティを作成
root-list-entities = #root: エンティティ一覧
root-delete-entity = #root: エンティティを削除
root-entity-updated = Runtime マニフェストが更新されました
entity-created = エンティティが作成されました
entity-deleted = エンティティが削除されました
entity-states-saving = エンティティの状態を IPFS に保存しています
entity-state-saving = エンティティの状態を保存しています
entity-state-saved = エンティティの状態が保存されました
entity-state-empty = プラグインが空の状態を返しました、保存をスキップします
entity-states-saved = エンティティの状態が保存されました
link-set = リンクが設定されました
ftl-loaded = IPFS から言語メッセージを読み込みました

# 初回起動 / 自動初期化
no-config-found = 設定が見つかりません。
initialising-new-identity = 新しい runtime アイデンティティを初期化しています。
generated-headless-config = ヘッドレス設定が生成されました。

# 所有権
runtime-claimed = Runtime が登録されました。

# 保護されたルート要素
refuse-delete-root = 必要なルート要素の削除を断固として拒否します
no-root-acl = ルート ACL が設定されていません — runtime はアクセス制御なしで動作しています
acl-owners-access = 呼び出し元は +owners のメンバーとしてアクセスが許可されました
namespace-not-found = 名前空間が見つかりません
no-ns-gate-acl = この名前空間のゲート ACL が設定されていません
runtime-claim-persisted = 所有者が設定に書き込まれました。
runtime-already-claimed = Runtime はすでに登録されています。


# Namespace creation (:create)
namespace-created = ネームスペースが作成されました
namespace-already-exists = ネームスペースはすでに存在します
namespace-name-reserved = ネームスペース名は予約済みです
namespace-create-denied = ネームスペース作成: アクセスが拒否されました
namespace-create-usage = 使い方: :create <名前>
crud-message-received = CRUDメッセージを受信
crud-acl-updated = ルートトランスポートACLを更新

# CRUD validation errors
blob-value-ipfs-path = blobの値はIPFSパス（/ipfs/、/ipns/、または/ipld/）である必要があります
acl-value-ipfs-path = ACLの値はIPFSパス（/ipfs/、/ipns/、または/ipld/）である必要があります
kind-value-ipfs-path = kindの値はIPFSパス（/ipfs/、/ipns/、または/ipld/）である必要があります
kind-not-found = 種別が見つかりません
cidv1-required = 値はベアCIDv1でなければなりません（'b'で始まる; CIDv0 'Qm…'は受け付けません）
config-key-protected = configキー '%key%' は保護されています
config-key-no-delete = daemonのconfigキー '%key%' は削除できません
config-key-not-manifest = configキー '%key%' は既知のmanifest configキーではありません
wrong-crud-protocol = 不正なCRUDプロトコル: %type%
entity-name-invalid = entity名は印刷可能なUTF-8でなければなりません
reserved-entity-name = entity名 '%name%' は予約済みです
