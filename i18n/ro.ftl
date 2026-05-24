# ma-runtime – Română
lang-name = Română

own-did-published = Documentul DID propriu publicat pe IPNS
own-did-publish-failed = Publicarea documentului DID propriu a eșuat
own-did-publish-timeout = Publicarea documentului DID propriu a expirat după 2 minute
started = ma runtime pornit
shutdown-requested = Oprire solicitată
closing-endpoint = Închiderea endpoint-ului iroh...
shutdown-complete = Oprire finalizată
status-listening = Serverul de stare ascultă
rpc-message-received = Mesaj RPC primit
rpc-message-rejected = Mesaj RPC respins
ipfs-message-rejected = Mesaj IPFS respins
ctrlc-handler-failed = Handler-ul Ctrl-C a eșuat
node-connected = Nod conectat la protocol
received-encrypted-ma-msg = Mesaj ma criptat primit pe /ma/ipfs/0.0.1
unknown-rpc-atom = Atom RPC necunoscut, ignorat
rpc-not-text-atom = Sarcina RPC nu este un atom de text
rpc-unknown-verb = Verb RPC necunoscut
rpc-reply-sent = Răspuns RPC trimis
ping-received = :ping primit, trimit :pong
did-publish-request-received = Cerere de publicare document DID primită
document-published = Document publicat
did-publish-cid-reply-sent = Răspuns CID trimis pentru publicarea DID
did-publish-resolve-failed = Nu s-a putut rezolva expeditorul pentru livrarea răspunsului ipfs-publish
ipfs-store-request-received = Cerere de stocare IPFS primită
ipfs-stored = Conținut stocat pe IPFS
ipfs-store-cid-reply-sent = Răspuns CID trimis
ipfs-store-resolve-failed = Nu s-a putut rezolva expeditorul pentru livrarea răspunsului ipfs-store

# Dispecerizarea entităților
bootstrap-complete = Bootstrap finalizat
entity-loaded = Plugin entitate încărcat
entity-load-failed = Încărcarea plugin-ului entitate a eșuat
entity-not-found = Entitate negăsită, RPC ignorat
entity-dispatched = RPC transmis entității
entity-replied = Entitatea a trimis răspuns RPC
root-create-entity = #root: creează entitate
root-list-entities = #root: listează entitățile
root-delete-entity = #root: șterge entitate
root-entity-updated = Manifest runtime actualizat
entity-created = Entitate creată
entity-deleted = Entitate ștearsă
entity-states-saving = Salvare stări entități în IPFS
entity-state-saving = Salvare stare entitate
entity-state-saved = Stare entitate salvată
entity-state-empty = Plugin-ul a returnat stare goală, salvarea omisă
entity-states-saved = Stări entități salvate
link-set = Legătură setată
ftl-loaded = Mesaje limbă încărcate din IPFS

# Prima rulare / auto-init
no-config-found = Nu s-a găsit nicio configurație.
initialising-new-identity = Inițializare nouă identitate runtime.
generated-headless-config = Configurație headless generată.

# Proprietate
runtime-claimed = Runtime înregistrat.

# Elemente rădăcină protejate
refuse-delete-root = Refuz categoric să șterg un element rădăcină obligatoriu
no-root-acl = Nu există ACL rădăcină configurat — runtime funcționează fără control acces
namespace-not-found = Spațiu de nume negăsit
no-ns-gate-acl = Nu există ACL de poartă configurat pentru acest spațiu de nume
runtime-claim-persisted = Proprietar scris în configurație.
runtime-already-claimed = Runtime deja înregistrat.


# Namespace creation (:create)
acl-missing-owners = ACL-ul rădăcină trebuie să conțină o intrare +owners
namespace-created = Spațiu de nume creat
namespace-already-exists = Spațiul de nume există deja
namespace-name-reserved = Numele spațiului de nume este rezervat
namespace-create-denied = Creare spațiu de nume: acces refuzat
namespace-create-usage = Utilizare: :create <nume>
crud-message-received = Mesaj CRUD primit
crud-acl-updated = ACL de transport rădăcină actualizat
