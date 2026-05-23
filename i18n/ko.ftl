# ma-runtime – 한국어
lang-name = 한국어

own-did-published = 자신의 DID 문서가 IPNS에 게시되었습니다
own-did-publish-failed = 자신의 DID 문서 게시에 실패했습니다
own-did-publish-timeout = 자신의 DID 문서 게시가 2분 후에 시간 초과되었습니다
started = ma runtime이 시작되었습니다
shutdown-requested = 종료가 요청되었습니다
closing-endpoint = iroh 엔드포인트를 닫는 중...
shutdown-complete = 종료가 완료되었습니다
status-listening = 상태 서버가 수신 대기 중입니다
rpc-message-received = RPC 메시지를 수신했습니다
rpc-message-rejected = RPC 메시지가 거부되었습니다
ipfs-message-rejected = IPFS 메시지가 거부되었습니다
ctrlc-handler-failed = Ctrl-C 핸들러가 실패했습니다
node-connected = 노드가 프로토콜에 연결되었습니다
received-encrypted-ma-msg = /ma/ipfs/0.0.1에서 암호화된 ma 메시지를 수신했습니다
unknown-rpc-atom = 알 수 없는 RPC 원자, 무시합니다
rpc-reply-sent = RPC 응답을 전송했습니다
ping-received = :ping 수신, :pong 전송 중
did-publish-request-received = DID 문서 게시 요청을 수신했습니다
document-published = 문서가 게시되었습니다
did-publish-cid-reply-sent = DID 게시를 위한 CID 응답을 전송했습니다
did-publish-resolve-failed = ipfs-publish 응답 전달을 위한 발신자 확인에 실패했습니다
ipfs-store-request-received = IPFS 저장 요청을 수신했습니다
ipfs-stored = 콘텐츠가 IPFS에 저장되었습니다
ipfs-store-cid-reply-sent = CID 응답을 전송했습니다
ipfs-store-resolve-failed = ipfs-store 응답 전달을 위한 발신자 확인에 실패했습니다

# 엔티티 디스패치
bootstrap-complete = Bootstrap이 완료되었습니다
entity-loaded = 엔티티 플러그인이 로드되었습니다
entity-load-failed = 엔티티 플러그인 로드에 실패했습니다
entity-not-found = 엔티티를 찾을 수 없습니다, RPC 무시
entity-dispatched = RPC가 엔티티로 디스패치되었습니다
entity-replied = 엔티티가 RPC 응답을 전송했습니다
root-create-entity = #root: 엔티티 생성
root-list-entities = #root: 엔티티 목록
root-delete-entity = #root: 엔티티 삭제
root-entity-updated = Runtime 매니페스트가 업데이트되었습니다
entity-created = 엔티티가 생성되었습니다
entity-deleted = 엔티티가 삭제되었습니다
entity-states-saving = 엔티티 상태를 IPFS에 저장하는 중
entity-state-saving = 엔티티 상태를 저장하는 중
entity-state-saved = 엔티티 상태가 저장되었습니다
entity-state-empty = 플러그인이 빈 상태를 반환했습니다, 저장을 건너뜁니다
entity-states-saved = 엔티티 상태가 저장되었습니다
link-set = 링크가 설정되었습니다
ftl-loaded = IPFS에서 언어 메시지를 로드했습니다

# 첫 번째 시작 / 자동 초기화
no-config-found = 설정을 찾을 수 없습니다.
initialising-new-identity = 새 runtime 아이덴티티를 초기화하는 중입니다.
generated-headless-config = 헤드리스 설정이 생성되었습니다.

# 소유권
runtime-claimed = Runtime이 등록되었습니다.

# 보호된 루트 요소
refuse-delete-root = 필요한 루트 요소 삭제를 단호히 거부합니다
no-root-acl = 루트 ACL이 구성되지 않았습니다 — runtime이 접근 제어 없이 실행 중입니다
namespace-not-found = 네임스페이스를 찾을 수 없습니다
no-ns-gate-acl = 이 네임스페이스에 게이트 ACL이 구성되지 않았습니다
runtime-claim-persisted = 소유자가 설정에 기록되었습니다.
runtime-already-claimed = Runtime이 이미 등록되어 있습니다.


# Namespace creation (:create)
acl-missing-owners = 루트 ACL에는 +owners 항목이 있어야 합니다
namespace-created = 네임스페이스가 생성되었습니다
namespace-already-exists = 네임스페이스가 이미 존재합니다
namespace-name-reserved = 네임스페이스 이름이 예약되어 있습니다
namespace-create-denied = 네임스페이스 생성: 접근 거부됨
namespace-create-usage = 사용법: :create <이름>
