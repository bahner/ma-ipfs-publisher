# art-x-lyaric — Dread Talk / Iyaric (Rasta)
# Rastafarian Iyaric dialect, also known as Dread Talk or Lyaric.
# BCP-47 private-use tag: art-x-lyaric
lang-name = Iyaric

own-did-published = I an I DID document publish forward to IPNS
own-did-publish-failed = Iration fail fi publish I an I DID document
own-did-publish-timeout = I an I DID document publish time out after 2 minutes
started = ma runtime a rise
shutdown-requested = Shutdown a call
closing-endpoint = Closing iroh endpoint...
shutdown-complete = Shutdown complete
status-listening = Status server a listen
rpc-message-received = RPC message receive
rpc-message-rejected = RPC message reject by Zion
ipfs-message-rejected = IPFS message reject by Zion
ctrlc-handler-failed = Ctrl-C handler fall
node-connected = I an I node connect to protocol
received-encrypted-ma-msg = Receive encrypted ma-msg pon /ma/ipfs/0.0.1
unknown-rpc-atom = Unknown RPC atom, pass over it
rpc-not-text-atom = RPC payload nuh text atom, seen
rpc-unknown-verb = Unknown RPC verb, pass over
rpc-reply-sent = RPC reply forward
ping-received = :ping receive, a send :pong
did-publish-request-received = Receive DID document publish request
document-published = Document publish forward
did-publish-cid-reply-sent = CID reply fi DID publish forward
did-publish-resolve-failed = Cyaan overstand sender fi carry ipfs-publish reply
ipfs-store-request-received = Receive IPFS store request
ipfs-stored = Content store pon IPFS in Zion
ipfs-store-cid-reply-sent = CID reply forward
ipfs-store-resolve-failed = Cyaan overstand sender fi carry ipfs-store reply

# Entity dispatch
bootstrap-complete = Bootstrap complete in Iration
entity-loaded = Entity plugin load
entity-load-failed = Iration fail fi load entity plugin
entity-not-found = Entity nuh find, pass over RPC
entity-dispatched = RPC dispatch to entity
entity-replied = Entity forward RPC reply
root-create-entity = #root: manifest entity
root-list-entities = #root: list entity dem
root-delete-entity = #root: remove entity
root-entity-updated = Runtime manifest update
entity-created = Entity manifest
entity-deleted = Entity remove
entity-states-saving = Saving entity states to IPFS
entity-state-saving = Saving entity state
entity-state-saved = Entity state save
entity-state-empty = Plugin return empty state, pass over
entity-states-saved = Entity states save
link-set = Link set forward
ftl-loaded = Lang messages load from IPFS

# First-run auto-init
no-config-found = No config nuh find.
initialising-new-identity = Rising new runtime identity.
generated-headless-config = Headless config generate.

# Ownership / claim
runtime-claimed = Runtime claim in Iration.

# Protected root elements
refuse-delete-root = Steadfastly refuse fi remove required root element
no-root-acl = No root ACL — runtime a work without Babylon control
acl-owners-access = Calla grantid access as memba of +owners
namespace-not-found = Namespace nuh find
no-ns-gate-acl = No gate ACL fi dis namespace
runtime-claim-persisted = Owner write to config in Zion.
runtime-already-claimed = Runtime already claim.


# Namespace creation (:create)
namespace-created = Namespace born inna dis livity
namespace-already-exists = Namespace already deh inna dis realm
namespace-name-reserved = Dat namespace name is irie an reserved
namespace-create-denied = Namespace creation: Babylon nah allow dis
namespace-create-usage = Riddim: :create <name>
crud-message-received = CRUD message come inna Zion
crud-acl-updated = Root transport ACL rise up new

# CRUD validation errors
blob-value-ipfs-path = di blob value mus be a IPFS path (/ipfs/, /ipns/, or /ipld/)
acl-value-ipfs-path = di ACL value mus be a IPFS path (/ipfs/, /ipns/, or /ipld/)
kind-value-ipfs-path = di kind value mus be a IPFS path (/ipfs/, /ipns/, or /ipld/)
kind-not-found = Di kind nuh sight
cidv1-required = di value mus be a bare CIDv1 (start wid 'b'; CIDv0 'Qm…' nah accepted)
config-key-protected = config key '%key%' is inna protection
config-key-no-delete = daemon config key '%key%' cyan delete
config-key-not-manifest = config key '%key%' nah known manifest config key
wrong-crud-protocol = wrong CRUD protocol: %type%
entity-name-invalid = di entity name must be clean printable UTF-8
reserved-entity-name = di entity name '%name%' ital, it reserved
