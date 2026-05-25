# ma-runtime — lang belta (Belter Creole)
lang-name = Lang Belta
# From The Expanse, constructed by linguist Nick Farmer.
# ISO 639-3: qbc (local/private use code)

own-did-published = oye DID-dok bringinowt IPNS
own-did-publish-failed = hámfalla bringinowt oye DID-dok
own-did-publish-timeout = bringinowt oye DID-dok tek-out finyish 2 minits
started = ma runtime gútegow
shutdown-requested = haaɗtugol kowtowm galifee
closing-endpoint = iroh endpoint na-du...
shutdown-complete = tek-out finyish
status-listening = kowlshowing servew dhaggeeffachaa
rpc-message-received = RPC sako bringin
rpc-message-rejected = RPC sako na owkwa
ipfs-message-rejected = IPFS sako na owkwa
ctrlc-handler-failed = Ctrl-C bossmang kása
node-connected = nod jokkondiraa wit laawol
received-encrypted-ma-msg = sikkina ma-sako bringin fo /ma/ipfs/0.0.1
unknown-rpc-atom = RPC atom na sasa, ignoring
rpc-not-text-atom = RPC koyo nating text atom
rpc-unknown-verb = RPC verb keng ando gonya
rpc-reply-sent = RPC kowtowm neldaa
ping-received = :ping bringin, neldi :pong
did-publish-request-received = DID-dok bringinowt kowtowm bringin
document-published = dok bringinowt
did-publish-cid-reply-sent = CID kowtowm fo DID bringinowt neldaa
did-publish-resolve-failed = hámfalla du fech neldowt fo ipfs-publish kowtowm
ipfs-store-request-received = IPFS hol kowtowm bringin
ipfs-stored = abubuwa hol im IPFS
ipfs-store-cid-reply-sent = CID kowtowm neldaa
ipfs-store-resolve-failed = hámfalla du fech neldowt fo ipfs-store kowtowm

# Neldi huunde
bootstrap-complete = Bootstrap finyish
entity-loaded = huunde plugin gútegow
entity-load-failed = hámfalla gútegow huunde plugin
entity-not-found = huunde na finyish wit, RPC ignoring
entity-dispatched = RPC neldaa huunde
entity-replied = huunde kowtowm RPC neldaa
root-create-entity = #root: mek huunde
root-list-entities = #root: kowl huunde
root-delete-entity = #root: na-du huunde
root-entity-updated = runtime manifest haaɗtaare
entity-created = huunde mek finyish
entity-deleted = huunde na-du finyish
entity-states-saving = hol huunde ɗeɗɗe im IPFS
entity-state-saving = hol huunde ɗeɗɗe
entity-state-saved = huunde ɗeɗɗe hol finyish
entity-state-empty = plugin yotti nating ɗeɗɗe, na hol
entity-states-saved = huunde ɗeɗɗe hol finyish
link-set = jokkol waɗaa
ftl-loaded = lang-sako bringin fo IPFS

# Wanya start / auto-init
no-config-found = nating saiti finyish wit.
initialising-new-identity = mekking nyu runtime selfmang.
generated-headless-config = headless saiti mek finyish.

# Jom selfmang
runtime-claimed = runtime faaɓinaa.

# Eegame jalte ɗe a ɗon
refuse-delete-root = na-du na-du na-du: tek-out jalte ɗe a ɗon na gonya
no-root-acl = nating root ACL — runtime gútegow beshkaise owkwa-hamma
acl-owners-access = Lo clamant a eu accès cume membre de +owners
namespace-not-found = namespace na finyish wit
no-ns-gate-acl = nating gate ACL fo dis namespace
runtime-claim-persisted = jom winndirii saiti.
runtime-already-claimed = runtime finyish faaɓinaa wanya.


# Namespace creation (:create)
namespace-created = namespace bon-kreye
namespace-already-exists = namespace jadu-la
namespace-name-reserved = dat namespace nem a gonya
namespace-create-denied = namespace tek-out: beshkaise owkwa-hamma
namespace-create-usage = koman: :create <nem>
crud-message-received = CRUD sako bringin
crud-acl-updated = root transport ACL haaɗtaare

# CRUD validation errors
blob-value-ipfs-path = di blob walowit muss bik a IPFS towchu (/ipfs/, /ipns/, o /ipld/)
acl-value-ipfs-path = di ACL walowit muss bik a IPFS towchu (/ipfs/, /ipns/, o /ipld/)
kind-value-ipfs-path = di kind walowit muss bik a IPFS towchu (/ipfs/, /ipns/, o /ipld/)
kind-not-found = Deting kind no find
cidv1-required = da felota nok kowl CIDv1 (starta fo 'b'; CIDv0 'Qm…' no bosco)
config-key-protected = config keyit '%key%' iz setanyeng
config-key-no-delete = daemon config keyit '%key%' du bik delowda
config-key-not-manifest = config keyit '%key%' nuk kowl manifest config keyit
wrong-crud-protocol = wrong CRUD protokowl: %type%
entity-name-invalid = fo name entity printable UTF-8 tenye
reserved-entity-name = fo name entity '%name%' reserved da
