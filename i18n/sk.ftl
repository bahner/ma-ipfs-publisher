# ma-runtime – Slovenčina
lang-name = Slovenčina

own-did-published = Vlastný DID dokument publikovaný na IPNS
own-did-publish-failed = Publikovanie vlastného DID dokumentu zlyhalo
own-did-publish-timeout = Publikovanie vlastného DID dokumentu vypršalo po 2 minútach
started = ma runtime spustený
shutdown-requested = Vypnutie požadované
closing-endpoint = Zatváranie iroh endpointu...
shutdown-complete = Vypnutie dokončené
status-listening = Stavový server počúva
rpc-message-received = Prijatá RPC správa
rpc-message-rejected = RPC správa odmietnutá
ipfs-message-rejected = IPFS správa odmietnutá
ctrlc-handler-failed = Obsluha Ctrl-C zlyhala
node-connected = Uzol pripojený k protokolu
received-encrypted-ma-msg = Prijatá šifrovaná ma správa na /ma/ipfs/0.0.1
unknown-rpc-atom = Neznámy RPC atom, ignorovanie
rpc-not-text-atom = RPC obsah nie je textovým atómom
rpc-unknown-verb = Neznámy RPC príkaz
rpc-reply-sent = RPC odpoveď odoslaná
ping-received = Prijatý :ping, odosielam :pong
did-publish-request-received = Prijatá žiadosť o publikovanie DID dokumentu
document-published = Dokument publikovaný
did-publish-cid-reply-sent = Odoslaná CID odpoveď pre publikovanie DID
did-publish-resolve-failed = Nepodarilo sa preložiť odosielateľa na doručenie odpovede ipfs-publish
ipfs-store-request-received = Prijatá žiadosť o uloženie IPFS
ipfs-stored = Obsah uložený na IPFS
ipfs-store-cid-reply-sent = CID odpoveď odoslaná
ipfs-store-resolve-failed = Nepodarilo sa preložiť odosielateľa na doručenie odpovede ipfs-store

# Odosielanie entít
bootstrap-complete = Bootstrap dokončený
entity-loaded = Plugin entity načítaný
entity-load-failed = Načítanie pluginu entity zlyhalo
entity-not-found = Entita nenájdená, RPC ignorované
entity-dispatched = RPC odovzdané entite
entity-replied = Entita odoslala RPC odpoveď
root-create-entity = #root: vytvoriť entitu
root-list-entities = #root: zoznam entít
root-delete-entity = #root: zmazať entitu
root-entity-updated = Runtime manifest aktualizovaný
entity-created = Entita vytvorená
entity-deleted = Entita zmazaná
entity-states-saving = Ukladanie stavov entít do IPFS
entity-state-saving = Ukladanie stavu entity
entity-state-saved = Stav entity uložený
entity-state-empty = Plugin vrátil prázdny stav, ukladanie preskočené
entity-states-saved = Stavy entít uložené
link-set = Odkaz nastavený
ftl-loaded = Jazykové správy načítané z IPFS

# Prvé spustenie / auto-init
no-config-found = Žiadna konfigurácia nenájdená.
initialising-new-identity = Inicializácia novej runtime identity.
generated-headless-config = Headless konfigurácia vygenerovaná.

# Vlastníctvo
runtime-claimed = Runtime registrovaný.

# Chránené koreňové prvky
refuse-delete-root = Dôrazne odmietam zmazať požadovaný koreňový prvok
no-root-acl = Žiadny koreňový ACL nie je nakonfigurovaný — runtime funguje bez riadenia prístupu
acl-owners-access = Volajúcemu bol udelený prístup ako členovi skupiny +owners
namespace-not-found = Menný priestor nenájdený
no-ns-gate-acl = Pre tento menný priestor nie je nakonfigurovaný gate ACL
runtime-claim-persisted = Vlastník zapísaný do konfigurácie.
runtime-already-claimed = Runtime je už registrovaný.


# Namespace creation (:create)
namespace-created = Priestor názvov vytvorený
namespace-already-exists = Priestor názvov už existuje
namespace-name-reserved = Názov priestoru názvov je rezervovaný
namespace-create-denied = Vytvorenie priestoru názvov: prístup zamietnutý
namespace-create-usage = Použitie: :create <názov>
crud-message-received = Prijatá správa CRUD
crud-acl-updated = Koreňový transportný ACL aktualizovaný

# CRUD validation errors
blob-value-ipfs-path = hodnota blob musí byť IPFS cesta (/ipfs/, /ipns/ alebo /ipld/)
acl-value-ipfs-path = hodnota ACL musí byť IPFS cesta (/ipfs/, /ipns/ alebo /ipld/)
kind-value-ipfs-path = hodnota kind musí byť IPFS cesta (/ipfs/, /ipns/ alebo /ipld/)
kind-not-found = Typ sa nenašiel
cidv1-required = hodnota musí byť holý CIDv1 (začína 'b'; CIDv0 'Qm…' nie je prijatý)
config-key-protected = konfiguračný kľúč '%key%' je chránený
config-key-no-delete = konfiguračný kľúč '%key%' démona nie je možné odstrániť
config-key-not-manifest = konfiguračný kľúč '%key%' nie je známym kľúčom manifest config
wrong-crud-protocol = nesprávny protokol CRUD: %type%
entity-name-invalid = názov entity musí byť tlačiteľné UTF-8
reserved-entity-name = názov entity '%name%' je rezervovaný
