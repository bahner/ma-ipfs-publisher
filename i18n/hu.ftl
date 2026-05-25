# ma-runtime – Magyar
lang-name = Magyar

own-did-published = Saját DID dokumentum közzétéve az IPNS-ben
own-did-publish-failed = A saját DID dokumentum közzététele sikertelen
own-did-publish-timeout = A saját DID dokumentum közzététele 2 perc után időtúllépett
started = A ma runtime elindult
shutdown-requested = Leállás kérve
closing-endpoint = Az iroh végpont bezárása...
shutdown-complete = A leállás befejeződött
status-listening = Az állapotszerver figyel
rpc-message-received = RPC üzenet érkezett
rpc-message-rejected = RPC üzenet elutasítva
ipfs-message-rejected = IPFS üzenet elutasítva
ctrlc-handler-failed = A Ctrl-C kezelő sikertelen volt
node-connected = Csomópont csatlakozott a protokollhoz
received-encrypted-ma-msg = Titkosított ma üzenet érkezett a /ma/ipfs/0.0.1 csatornán
unknown-rpc-atom = Ismeretlen RPC atom, mellőzés
rpc-not-text-atom = Az RPC tartalom nem szövegatom
rpc-unknown-verb = Ismeretlen RPC-ige
rpc-reply-sent = RPC válasz elküldve
ping-received = :ping érkezett, :pong küldése
did-publish-request-received = DID dokumentum közzétételi kérés érkezett
document-published = Dokumentum közzétéve
did-publish-cid-reply-sent = CID válasz elküldve a DID közzétételhez
did-publish-resolve-failed = A küldő feloldása sikertelen az ipfs-publish válasz kézbesítéséhez
ipfs-store-request-received = IPFS tárolási kérés érkezett
ipfs-stored = Tartalom eltárolva az IPFS-ben
ipfs-store-cid-reply-sent = CID válasz elküldve
ipfs-store-resolve-failed = A küldő feloldása sikertelen az ipfs-store válasz kézbesítéséhez

# Entitások kézbesítése
bootstrap-complete = Bootstrap kész
entity-loaded = Entitás bővítmény betöltve
entity-load-failed = Az entitás bővítmény betöltése sikertelen
entity-not-found = Az entitás nem található, RPC mellőzve
entity-dispatched = RPC kézbesítve az entitásnak
entity-replied = Az entitás RPC választ küldött
root-create-entity = #root: entitás létrehozása
root-list-entities = #root: entitások listája
root-delete-entity = #root: entitás törlése
root-entity-updated = Runtime manifest frissítve
entity-created = Entitás létrehozva
entity-deleted = Entitás törölve
entity-states-saving = Entitásállapotok mentése az IPFS-be
entity-state-saving = Entitásállapot mentése
entity-state-saved = Entitásállapot mentve
entity-state-empty = A bővítmény üres állapotot adott vissza, mentés kihagyva
entity-states-saved = Entitásállapotok mentve
link-set = Hivatkozás beállítva
ftl-loaded = Nyelvi üzenetek betöltve az IPFS-ből

# Első indítás / auto-init
no-config-found = Nem található konfiguráció.
initialising-new-identity = Új runtime identitás inicializálása.
generated-headless-config = Fejnélküli konfiguráció generálva.

# Tulajdonjog
runtime-claimed = A runtime regisztrálva.

# Védett gyökérelemek
refuse-delete-root = Határozottan megtagadom a szükséges gyökérelem törlését
no-root-acl = Nincs gyökér-ACL konfigurálva — a runtime hozzáférés-vezérlés nélkül működik
acl-owners-access = A hívónak hozzáférést kaptott +owners tagjaként
namespace-not-found = A névtér nem található
no-ns-gate-acl = Ehhez a névtérhez nincs gate-ACL konfigurálva
runtime-claim-persisted = A tulajdonos beírva a konfigurációba.
runtime-already-claimed = A runtime már regisztrálva van.


# Namespace creation (:create)
namespace-created = Névtér létrehozva
namespace-already-exists = A névtér már létezik
namespace-name-reserved = A névtér neve foglalt
namespace-create-denied = Névtér létrehozás: hozzáférés megtagadva
namespace-create-usage = Használat: :create <név>
crud-message-received = CRUD üzenet érkezett
crud-acl-updated = Gyökér-átviteli ACL frissítve

# CRUD validation errors
blob-value-ipfs-path = a blob értékének IPFS elérési útnak kell lennie (/ipfs/, /ipns/ vagy /ipld/)
acl-value-ipfs-path = az ACL értékének IPFS elérési útnak kell lennie (/ipfs/, /ipns/ vagy /ipld/)
kind-value-ipfs-path = a kind értékének IPFS elérési útnak kell lennie (/ipfs/, /ipns/ vagy /ipld/)
kind-not-found = Típus nem található
cidv1-required = az értéknek egyszerű CIDv1-nek kell lennie ('b'-vel kezdődik; CIDv0 'Qm…' nem fogadható el)
config-key-protected = a '%key%' config kulcs védett
config-key-no-delete = a '%key%' daemon config kulcs nem törölhető
config-key-not-manifest = a '%key%' config kulcs nem ismert manifest config kulcs
wrong-crud-protocol = helytelen CRUD protokoll: %type%
entity-name-invalid = az entity neve nyomtatható UTF-8 legyen
reserved-entity-name = az entity neve '%name%' foglalt
