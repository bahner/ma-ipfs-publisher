# ma-runtime – Tok Pisin
lang-name = Tok Pisin

own-did-published = Dokumen bilong DID i bin planim long IPNS
own-did-publish-failed = Planim bilong dokumen DID i no wok
own-did-publish-timeout = Planim bilong dokumen DID i pinis taim bihain 2 minit
started = ma runtime i stat
shutdown-requested = Klospim i bin askim
closing-endpoint = Klospim poin bilong iroh...
shutdown-complete = Klospim i pinis
status-listening = Siva bilong stetes i harim
rpc-message-received = Mesej RPC i kamap
rpc-message-rejected = Mesej RPC i bin rausim
ipfs-message-rejected = Mesej IPFS i bin rausim
ctrlc-handler-failed = Handler bilong Ctrl-C i no wok
node-connected = Nod i konek long protokol
received-encrypted-ma-msg = Kisim mesej ma-msg i bin haitim long /ma/ipfs/0.0.1
unknown-rpc-atom = Atom RPC i no save, lusim
rpc-not-text-atom = RPC samting i no wanpela tok
rpc-unknown-verb = RPC tok i no save
rpc-reply-sent = Bekim RPC i bin salim
ping-received = :ping i kamap, salim :pong
did-publish-request-received = Kisim askim bilong planim dokumen DID
document-published = Dokumen i bin planim
did-publish-cid-reply-sent = Bekim CID bilong planim DID i bin salim
did-publish-resolve-failed = I no inap painim sende bilong givim bekim ipfs-publish
ipfs-store-request-received = Kisim askim bilong storim IPFS
ipfs-stored = Kontens i storim long IPFS
ipfs-store-cid-reply-sent = Bekim CID i bin salim
ipfs-store-resolve-failed = I no inap painim sende bilong givim bekim ipfs-store

# Entity dispatch
bootstrap-complete = Bootstrap i pinis
entity-loaded = Plugin bilong entiti i lod
entity-load-failed = Lodim plugin bilong entiti i no wok
entity-not-found = Entiti i no painim, lusim RPC
entity-dispatched = RPC i bin salim long entiti
entity-replied = Entiti i bin bekim RPC
root-create-entity = #root: wokim entiti
root-list-entities = #root: soim lis bilong entiti
root-delete-entity = #root: rausim entiti
root-entity-updated = Manifest bilong runtime i apdetim
entity-created = Entiti i wokim
entity-deleted = Entiti i rausim
entity-states-saving = Seivim stet bilong entiti long IPFS
entity-state-saving = Seivim stet bilong entiti
entity-state-saved = Stet bilong entiti i seivim
entity-state-empty = Plugin i givim stet nating, lusim
entity-states-saved = Stet bilong entiti i seivim
link-set = Link i putim
ftl-loaded = Mesej bilong lang i lod long IPFS

# First-run auto-init
no-config-found = I no gat konfigurasion i painim.
initialising-new-identity = Wokim nupela identiti bilong runtime.
generated-headless-config = Konfigurasion headless i mekim.

# Ownership / claim
runtime-claimed = Runtime i klemim.

# Protected root elements
refuse-delete-root = Tok no long rausim elementis bilong rut
no-root-acl = I no gat rut ACL i putim — runtime i wok nating long kontrolim aksess
acl-owners-access = Dispela man i kisim olgeta as memba bilong +owners
namespace-not-found = Espas nem i no painim
no-ns-gate-acl = I no gat get ACL i putim bilong dispela espas nem
runtime-claim-persisted = Ona i raitim long konfigurasion.
runtime-already-claimed = Runtime i klemim pinis.


# Namespace creation (:create)
namespace-created = Espas nem i klia pinis
namespace-already-exists = Espas nem i stap pinis
namespace-name-reserved = Nem bilong espas nem i bokisim pinis
namespace-create-denied = Mekim espas nem: no privilidzis
namespace-create-usage = Yusim: :create <nem>
crud-message-received = CRUD mesej i kam pinis
crud-acl-updated = Root transport ACL i nupela pinis

# CRUD validation errors
blob-value-ipfs-path = nemb bilong blob i mas kamap IPFS rot (/ipfs/, /ipns/, o /ipld/)
acl-value-ipfs-path = nemb bilong ACL i mas kamap IPFS rot (/ipfs/, /ipns/, o /ipld/)
kind-value-ipfs-path = nemb bilong kind i mas kamap IPFS rot (/ipfs/, /ipns/, o /ipld/)
kind-not-found = No painim dispela kain
cidv1-required = namba i mas baim CIDv1 nating ('b' i kirap long en; CIDv0 'Qm…' no orait)
config-key-protected = config ki '%key%' i gat banis
config-key-no-delete = i no inap rausim daemon config ki '%key%'
config-key-not-manifest = config ki '%key%' i no wan manifest config ki we ol save
wrong-crud-protocol = CRUD protokol i rong: %type%
entity-name-invalid = nem bilong entity i mas stap printable UTF-8
reserved-entity-name = nem bilong entity '%name%' i reserved
