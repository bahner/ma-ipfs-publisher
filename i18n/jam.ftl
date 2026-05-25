# ma-runtime – Patwa
lang-name = Patwa

own-did-published = Owna DID dokument publish to IPNS
own-did-publish-failed = Cyaan publish owna DID dokument
own-did-publish-timeout = Owna DID dokument publish time out afta 2 minit
started = ma runtime staat
shutdown-requested = Shutdown request mek
closing-endpoint = Closing iroh endpoint...
shutdown-complete = Shutdown complete
status-listening = Status server a listen
rpc-message-received = RPC message receive
rpc-message-rejected = RPC message reject
ipfs-message-rejected = IPFS message reject
ctrlc-handler-failed = Ctrl-C handler fail
node-connected = Node connect to protocol
received-encrypted-ma-msg = Receive encrypt ma-msg pan /ma/ipfs/0.0.1
unknown-rpc-atom = Unknown RPC atom, ignore it
rpc-not-text-atom = RPC toktok no nuh text atom
rpc-unknown-verb = RPC komand no rekn
rpc-reply-sent = RPC reply send
ping-received = :ping receive, a send :pong
did-publish-request-received = Receive DID dokument publish request
document-published = Dokument publish
did-publish-cid-reply-sent = CID reply fi DID publish send
did-publish-resolve-failed = Cyaan resolve sender fi deliver ipfs-publish reply
ipfs-store-request-received = Receive IPFS store request
ipfs-stored = Content store pon IPFS
ipfs-store-cid-reply-sent = CID reply send
ipfs-store-resolve-failed = Cyaan resolve sender fi deliver ipfs-store reply

# Entity dispatch
bootstrap-complete = Bootstrap complete
entity-loaded = Entity plugin load
entity-load-failed = Cyaan load entity plugin
entity-not-found = Entity nuh find, ignore RPC
entity-dispatched = RPC dispatch to entity
entity-replied = Entity send RPC reply
root-create-entity = #root: create entity
root-list-entities = #root: list entity dem
root-delete-entity = #root: delete entity
root-entity-updated = Runtime manifest update
entity-created = Entity create
entity-deleted = Entity delete
entity-states-saving = Saving entity state dem to IPFS
entity-state-saving = Saving entity state
entity-state-saved = Entity state save
entity-state-empty = Plugin return empty state, skip it
entity-states-saved = Entity state dem save
link-set = Link set
ftl-loaded = Lang message load from IPFS

# First-run auto-init
no-config-found = No config nuh find.
initialising-new-identity = Initialising new runtime identity.
generated-headless-config = Headless config generate.

# Ownership / claim
runtime-claimed = Runtime claim.

# Protected root elements
refuse-delete-root = Steadfastly refuse fi delete required root element
no-root-acl = No root ACL configure — runtime a work without access control
acl-owners-access = Di calla get access as memba a +owners
namespace-not-found = Namespace nuh find
no-ns-gate-acl = No gate ACL configure fi dis namespace
runtime-claim-persisted = Owner write to config.
runtime-already-claimed = Runtime already claim.


# Namespace creation (:create)
namespace-created = Namespace mek
namespace-already-exists = Namespace deh deh already
namespace-name-reserved = Namespace name taken already
namespace-create-denied = Namespace create: access deny
namespace-create-usage = How fi use: :create <name>
crud-message-received = CRUD mesij receive
crud-acl-updated = Root transport ACL update

# CRUD validation errors
blob-value-ipfs-path = di blob value haffi be a IPFS path (/ipfs/, /ipns/, or /ipld/)
acl-value-ipfs-path = di ACL value haffi be a IPFS path (/ipfs/, /ipns/, or /ipld/)
kind-value-ipfs-path = di kind value haffi be a IPFS path (/ipfs/, /ipns/, or /ipld/)
kind-not-found = Di kind nuh deh deh
cidv1-required = di value haffi be a bare CIDv1 (start wid 'b'; CIDv0 'Qm…' nuh accepted)
config-key-protected = config key '%key%' protect
config-key-no-delete = daemon config key '%key%' cyaan delete
config-key-not-manifest = config key '%key%' nuh known manifest config key
wrong-crud-protocol = wrong CRUD protocol: %type%
entity-name-invalid = di entity name haffi be printable UTF-8
reserved-entity-name = di entity name '%name%' reserved
