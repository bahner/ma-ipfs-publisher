# ma-runtime – Papiamentu
lang-name = Papiamentu

own-did-published = Dokumentu DID propio a keda publika na IPNS
own-did-publish-failed = No a logra publica dokumentu DID propio
own-did-publish-timeout = Publikashon di dokumentu DID propio a expira despues di 2 minit
started = ma runtime a kuminsá
shutdown-requested = Apagamento a keda pidi
closing-endpoint = Serrando punto di iroh...
shutdown-complete = Apagamento kompleto
status-listening = Sèrber di status ta skiucha
rpc-message-received = Mensahe RPC a keda risibí
rpc-message-rejected = Mensahe RPC a keda rekasá
ipfs-message-rejected = Mensahe IPFS a keda rekasá
ctrlc-handler-failed = Handler Ctrl-C a fayá
node-connected = Nodo a konektá na protokol
received-encrypted-ma-msg = A risibí ma-msg enkriptá riba /ma/ipfs/0.0.1
unknown-rpc-atom = Atom RPC deskonosí, ignorando
rpc-not-text-atom = Karga RPC no ta un átomo di tekstu
rpc-unknown-verb = Verbo RPC deskonosí
rpc-reply-sent = Respuesta RPC a keda mandá
ping-received = :ping a keda risibí, mandando :pong
did-publish-request-received = A risibí petishon pa publica dokumentu DID
document-published = Dokumentu a keda publika
did-publish-cid-reply-sent = Respuesta CID pa publikashon DID a keda mandá
did-publish-resolve-failed = No por resolbe mandado pa entregá respuesta ipfs-publish
ipfs-store-request-received = A risibí petishon di almacenamentu IPFS
ipfs-stored = Kontenido a keda almacená na IPFS
ipfs-store-cid-reply-sent = Respuesta CID a keda mandá
ipfs-store-resolve-failed = No por resolbe mandado pa entregá respuesta ipfs-store

# Entity dispatch
bootstrap-complete = Bootstrap kompleto
entity-loaded = Plugin di entidad a keda kargá
entity-load-failed = No a logra karga plugin di entidad
entity-not-found = Entidad no a keda haña, ignorando RPC
entity-dispatched = RPC a keda mandá na entidad
entity-replied = Entidad a mandá respuesta RPC
root-create-entity = #root: krea entidad
root-list-entities = #root: lista entidad
root-delete-entity = #root: bora entidad
root-entity-updated = Manifest di runtime a keda aktualisa
entity-created = Entidad a keda kreá
entity-deleted = Entidad a keda borá
entity-states-saving = Guardando estadonan di entidad na IPFS
entity-state-saving = Guardando estado di entidad
entity-state-saved = Estado di entidad a keda guardá
entity-state-empty = Plugin a bolbe estado bashi, brinká
entity-states-saved = Estadonan di entidad a keda guardá
link-set = Link a keda shetá
ftl-loaded = Mensahenan di lenguahe a keda kargá for di IPFS

# First-run auto-init
no-config-found = Ningun konfigurashon a keda haña.
initialising-new-identity = Inicializando identidad nobo di runtime.
generated-headless-config = Konfigurashon headless a keda generá.

# Ownership / claim
runtime-claimed = Runtime a keda reklamá.

# Protected root elements
refuse-delete-root = Rekasando eliminá elemento raís rekerí
no-root-acl = Ningun ACL raís konfigurá — runtime ta operando sin kontrol di akseso
acl-owners-access = E yamanti a e keda akseso komo miembro di +owners
namespace-not-found = Namespace no a keda haña
no-ns-gate-acl = Ningun ACL di puerta konfigurá pa e namespace aki
runtime-claim-persisted = Dueño a keda skirbí na konfigurashon.
runtime-already-claimed = Runtime a keda reklamá ya.


# Namespace creation (:create)
namespace-created = Namespace a keda kreá
namespace-already-exists = Namespace ya ta eksistí
namespace-name-reserved = Non di namespace ta reservá
namespace-create-denied = Kreashon di namespace: akseso a nega
namespace-create-usage = Uso: :create <non>
crud-message-received = Mensahe CRUD risibí
crud-acl-updated = Root transport ACL aktualisá

# CRUD validation errors
blob-value-ipfs-path = e valor di blob mester ta un caminda IPFS (/ipfs/, /ipns/, of /ipld/)
acl-value-ipfs-path = e valor di ACL mester ta un caminda IPFS (/ipfs/, /ipns/, of /ipld/)
kind-value-ipfs-path = e valor di kind mester ta un caminda IPFS (/ipfs/, /ipns/, of /ipld/)
kind-not-found = Tipo no a haya
cidv1-required = e balor mester ta un CIDv1 puur (kuminsá ku 'b'; CIDv0 'Qm…' no ta aceptá)
config-key-protected = e yabi di config '%key%' ta protehá
config-key-no-delete = e yabi di config '%key%' di daemon no por wòrdu borá
config-key-not-manifest = e yabi di config '%key%' no ta un yabi di manifest config konosí
wrong-crud-protocol = protokòl CRUD rong: %type%
entity-name-invalid = number di entity mester ta UTF-8 printabel
reserved-entity-name = number di entity '%name%' ta reserva
