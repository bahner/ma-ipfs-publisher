# ma-runtime – Čeština
lang-name = Čeština

own-did-published = Vlastní DID dokument publikován na IPNS
own-did-publish-failed = Publikování vlastního DID dokumentu selhalo
own-did-publish-timeout = Publikování vlastního DID dokumentu vypršelo po 2 minutách
started = ma runtime spuštěn
shutdown-requested = Vypnutí požadováno
closing-endpoint = Uzavírání iroh endpointu...
shutdown-complete = Vypnutí dokončeno
status-listening = Stavový server naslouchá
rpc-message-received = Přijata RPC zpráva
rpc-message-rejected = RPC zpráva odmítnuta
ipfs-message-rejected = IPFS zpráva odmítnuta
ctrlc-handler-failed = Obsluha Ctrl-C selhala
node-connected = Uzel připojen k protokolu
received-encrypted-ma-msg = Přijata zašifrovaná ma zpráva na /ma/ipfs/0.0.1
unknown-rpc-atom = Neznámý RPC atom, ignorování
rpc-not-text-atom = RPC data nejsou textovým atomem
rpc-unknown-verb = Neznámý RPC příkaz
rpc-reply-sent = RPC odpověď odeslána
ping-received = Přijat :ping, odesílám :pong
did-publish-request-received = Přijat požadavek na publikování DID dokumentu
document-published = Dokument publikován
did-publish-cid-reply-sent = Odeslána CID odpověď pro publikování DID
did-publish-resolve-failed = Nelze přeložit odesílatele pro doručení odpovědi ipfs-publish
ipfs-store-request-received = Přijat požadavek na uložení IPFS
ipfs-stored = Obsah uložen na IPFS
ipfs-store-cid-reply-sent = CID odpověď odeslána
ipfs-store-resolve-failed = Nelze přeložit odesílatele pro doručení odpovědi ipfs-store

# Odeslání entit
bootstrap-complete = Bootstrap dokončen
entity-loaded = Plugin entity načten
entity-load-failed = Načtení pluginu entity selhalo
entity-not-found = Entita nenalezena, RPC ignorováno
entity-dispatched = RPC předáno entitě
entity-replied = Entita odeslala RPC odpověď
root-create-entity = #root: vytvořit entitu
root-list-entities = #root: seznam entit
root-delete-entity = #root: smazat entitu
root-entity-updated = Runtime manifest aktualizován
entity-created = Entita vytvořena
entity-deleted = Entita smazána
entity-states-saving = Ukládání stavů entit do IPFS
entity-state-saving = Ukládání stavu entity
entity-state-saved = Stav entity uložen
entity-state-empty = Plugin vrátil prázdný stav, ukládání přeskočeno
entity-states-saved = Stavy entit uloženy
link-set = Odkaz nastaven
ftl-loaded = Jazykové zprávy načteny z IPFS

# První spuštění / auto-init
no-config-found = Nenalezena žádná konfigurace.
initialising-new-identity = Inicializace nové runtime identity.
generated-headless-config = Headless konfigurace vygenerována.

# Vlastnictví
runtime-claimed = Runtime registrován.

# Chráněné kořenové prvky
refuse-delete-root = Důrazně odmítám smazat požadovaný kořenový prvek
no-root-acl = Žádný kořenový ACL není nakonfigurován — runtime funguje bez řízení přístupu
namespace-not-found = Jmenný prostor nenalezen
no-ns-gate-acl = Pro tento jmenný prostor není nakonfigurován gate ACL
runtime-claim-persisted = Vlastník zapsán do konfigurace.
runtime-already-claimed = Runtime je již registrován.


# Namespace creation (:create)
acl-missing-owners = Kořenový ACL musí obsahovat záznam +owners
namespace-created = Jmenný prostor byl vytvořen
namespace-already-exists = Jmenný prostor již existuje
namespace-name-reserved = Název jmenného prostoru je vyhrazen
namespace-create-denied = Vytvoření jmenného prostoru: přístup odepřen
namespace-create-usage = Použití: :create <název>
crud-message-received = Přijata zpráva CRUD
crud-acl-updated = Kořenový transportní ACL aktualizován
