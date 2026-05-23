# ma-runtime – Slovenščina
lang-name = Slovenščina

own-did-published = Lastni DID dokument objavljen na IPNS
own-did-publish-failed = Objava lastnega DID dokumenta ni uspela
own-did-publish-timeout = Objava lastnega DID dokumenta je potekla po 2 minutah
started = ma runtime zagnan
shutdown-requested = Zahtevano zaustavitev
closing-endpoint = Zapiranje iroh končne točke...
shutdown-complete = Zaustavitev dokončana
status-listening = Strežnik statusa posluša
rpc-message-received = Prejeto RPC sporočilo
rpc-message-rejected = RPC sporočilo zavrnjeno
ipfs-message-rejected = IPFS sporočilo zavrnjeno
ctrlc-handler-failed = Upravljalnik Ctrl-C je odpovedal
node-connected = Vozlišče povezano s protokolom
received-encrypted-ma-msg = Prejeto šifrirano ma sporočilo na /ma/ipfs/0.0.1
unknown-rpc-atom = Neznani RPC atom, prezrto
rpc-reply-sent = RPC odgovor poslan
ping-received = Prejet :ping, pošiljam :pong
did-publish-request-received = Prejeta zahteva za objavo DID dokumenta
document-published = Dokument objavljen
did-publish-cid-reply-sent = Poslan CID odgovor za objavo DID
did-publish-resolve-failed = Ni mogoče razrešiti pošiljatelja za dostavo odgovora ipfs-publish
ipfs-store-request-received = Prejeta zahteva za shranjevanje IPFS
ipfs-stored = Vsebina shranjena na IPFS
ipfs-store-cid-reply-sent = CID odgovor poslan
ipfs-store-resolve-failed = Ni mogoče razrešiti pošiljatelja za dostavo odgovora ipfs-store

# Razpošiljanje entitet
bootstrap-complete = Bootstrap dokončan
entity-loaded = Vtičnik entitete naložen
entity-load-failed = Nalaganje vtičnika entitete ni uspelo
entity-not-found = Entiteta ni najdena, RPC prezrto
entity-dispatched = RPC posredovano entiteti
entity-replied = Entiteta je poslala RPC odgovor
root-create-entity = #root: ustvari entiteto
root-list-entities = #root: seznam entitet
root-delete-entity = #root: izbriši entiteto
root-entity-updated = Runtime manifest posodobljen
entity-created = Entiteta ustvarjena
entity-deleted = Entiteta izbrisana
entity-states-saving = Shranjevanje stanj entitet v IPFS
entity-state-saving = Shranjevanje stanja entitete
entity-state-saved = Stanje entitete shranjeno
entity-state-empty = Vtičnik je vrnil prazno stanje, shranjevanje preskočeno
entity-states-saved = Stanja entitet shranjena
link-set = Povezava nastavljena
ftl-loaded = Jezikovne sporočila naložena iz IPFS

# Prvi zagon / auto-init
no-config-found = Konfiguracija ni najdena.
initialising-new-identity = Inicializacija nove runtime identitete.
generated-headless-config = Headless konfiguracija ustvarjena.

# Lastništvo
runtime-claimed = Runtime registriran.

# Zaščiteni korenski elementi
refuse-delete-root = Odločno zavračam brisanje zahtevanega korenskega elementa
no-root-acl = Ni konfiguriranega korenskega ACL — runtime deluje brez nadzora dostopa
namespace-not-found = Imenski prostor ni najden
no-ns-gate-acl = Za ta imenski prostor ni konfiguriran gate ACL
runtime-claim-persisted = Lastnik zapisan v konfiguracijo.
runtime-already-claimed = Runtime je že registriran.


# Namespace creation (:create)
acl-missing-owners = Korenski ACL mora vsebovati vnos +owners
namespace-created = Imenski prostor ustvarjen
namespace-already-exists = Imenski prostor že obstaja
namespace-name-reserved = Ime imenskega prostora je rezervirano
namespace-create-denied = Ustvarjanje imenskega prostora: dostop zavrnjen
namespace-create-usage = Uporaba: :create <ime>
