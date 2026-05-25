# ma-runtime – Kriolu
lang-name = Kriolu

own-did-published = Dokumentu DID propiu publikadu na IPNS
own-did-publish-failed = Ka konsigui publiká dokumentu DID propiu
own-did-publish-timeout = Publikason di dokumentu DID propiu xpiradu dipus di 2 minutu
started = ma runtime kumesadu
shutdown-requested = Desligamentu pedidu
closing-endpoint = Fechandu pontu di iroh...
shutdown-complete = Desligamentu konpletu
status-listening = Sèrvidor di stadu sta skuta
rpc-message-received = Mensajen RPC risibidu
rpc-message-rejected = Mensajen RPC rejitadu
ipfs-message-rejected = Mensajen IPFS rejitadu
ctrlc-handler-failed = Manijadór Ctrl-C falhadu
node-connected = Nodu ligadu a protokolu
received-encrypted-ma-msg = Risibidu ma-msg enkriptadu na /ma/ipfs/0.0.1
unknown-rpc-atom = Atom RPC deskonsidu, ignoradu
rpc-not-text-atom = Xargi RPC ka é un átumu di textu
rpc-unknown-verb = Verbu RPC deskonyisidu
rpc-reply-sent = Risposta RPC mandadu
ping-received = :ping risibidu, mandandu :pong
did-publish-request-received = Risibidu pedidu di publikason di dokumentu DID
document-published = Dokumentu publikadu
did-publish-cid-reply-sent = Risposta CID pa publikason DID mandadu
did-publish-resolve-failed = Ka konsigui rezolvi rimitentu pa entregá risposta ipfs-publish
ipfs-store-request-received = Risibidu pedidu di armazenamentu IPFS
ipfs-stored = Konteúdu armazenadu na IPFS
ipfs-store-cid-reply-sent = Risposta CID mandadu
ipfs-store-resolve-failed = Ka konsigui rezolvi rimitentu pa entregá risposta ipfs-store

# Entity dispatch
bootstrap-complete = Bootstrap konpletu
entity-loaded = Plugin di entidadi kargadu
entity-load-failed = Ka konsigui kargá plugin di entidadi
entity-not-found = Entidadi ka atxadu, ignorandu RPC
entity-dispatched = RPC dispachadú pa entidadi
entity-replied = Entidadi mandadu risposta RPC
root-create-entity = #root: kriá entidadi
root-list-entities = #root: listá entidadi
root-delete-entity = #root: eliminá entidadi
root-entity-updated = Manifèstu di runtime atualizadu
entity-created = Entidadi kriadu
entity-deleted = Entidadi eliminadu
entity-states-saving = Gravandu stadu di entidadi na IPFS
entity-state-saving = Gravandu stadu di entidadi
entity-state-saved = Stadu di entidadi gravadu
entity-state-empty = Plugin retornadu stadu vaziu, saltandu
entity-states-saved = Stadu di entidadi gravadu
link-set = Ligason definidu
ftl-loaded = Mensajen di lingua kargadu di IPFS

# First-run auto-init
no-config-found = Nenhun konfigurason atxadu.
initialising-new-identity = Inisializandu nova identidadi di runtime.
generated-headless-config = Konfigurason headless geradú.

# Ownership / claim
runtime-claimed = Runtime reklamadu.

# Protected root elements
refuse-delete-root = Resuzandu firmimenti eliminá elementu raiz obrigatóriu
no-root-acl = Nenhun ACL raiz konfiguradu — runtime ta funcionandu sen kontrolu di asesu
acl-owners-access = Chamador ten aksesu kumu membro di +owners
namespace-not-found = Spasu di nomi ka atxadu
no-ns-gate-acl = Nenhun ACL di portason konfiguradu pa esi spasu di nomi
runtime-claim-persisted = Proprietáriu skrividu na konfigurason.
runtime-already-claimed = Runtime ja reklamadu.


# Namespace creation (:create)
namespace-created = Namespace kriadu
namespace-already-exists = Namespace ja ta la
namespace-name-reserved = Nomi di namespace ta reservadu
namespace-create-denied = Kriason di namespace: azesu ngadu
namespace-create-usage = Uzu: :create <nomi>
crud-message-received = Mensajen CRUD resibidu
crud-acl-updated = ACL di transporte raiz atualizada

# CRUD validation errors
blob-value-ipfs-path = valur di blob ten di ser un kaminhu IPFS (/ipfs/, /ipns/, ou /ipld/)
acl-value-ipfs-path = valur di ACL ten di ser un kaminhu IPFS (/ipfs/, /ipns/, ou /ipld/)
kind-value-ipfs-path = valur di kind ten di ser un kaminhu IPFS (/ipfs/, /ipns/, ou /ipld/)
kind-not-found = Tépu ka atuadu
cidv1-required = valór ten k'es un CIDv1 pur (kumesa ku 'b'; CIDv0 'Qm…' ka ta aceitu)
config-key-protected = xavi di config '%key%' sta protejidu
config-key-no-delete = ka da eliminá xavi di config '%key%' di daemon
config-key-not-manifest = xavi di config '%key%' ka é un xavi di manifest config konxidu
wrong-crud-protocol = protokolu CRUD eradu: %type%
entity-name-invalid = nomi di entity ten di ser UTF-8 imprimivel
reserved-entity-name = nomi di entity '%name%' ta rezervadu
