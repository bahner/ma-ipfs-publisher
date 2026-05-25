# ma-runtime – Hrvatski
lang-name = Hrvatski

own-did-published = Vlastiti DID dokument objavljen na IPNS
own-did-publish-failed = Objavljivanje vlastitog DID dokumenta nije uspjelo
own-did-publish-timeout = Objavljivanje vlastitog DID dokumenta isteklo nakon 2 minute
started = ma runtime pokrenut
shutdown-requested = Zatvaranje zatraženo
closing-endpoint = Zatvaranje iroh krajnje točke...
shutdown-complete = Zatvaranje dovršeno
status-listening = Statusni poslužitelj sluša
rpc-message-received = Primljena RPC poruka
rpc-message-rejected = RPC poruka odbijena
ipfs-message-rejected = IPFS poruka odbijena
ctrlc-handler-failed = Upravljač Ctrl-C nije uspio
node-connected = Čvor spojen na protokol
received-encrypted-ma-msg = Primljena šifrirana ma poruka na /ma/ipfs/0.0.1
unknown-rpc-atom = Nepoznati RPC atom, ignoriranje
rpc-not-text-atom = RPC sadržaj nije tekstni atom
rpc-unknown-verb = Nepoznata RPC naredba
rpc-reply-sent = RPC odgovor poslan
ping-received = Primljen :ping, šaljem :pong
did-publish-request-received = Primljen zahtjev za objavu DID dokumenta
document-published = Dokument objavljen
did-publish-cid-reply-sent = Poslan CID odgovor za objavu DID
did-publish-resolve-failed = Nije moguće razriješiti pošiljatelja za dostavu odgovora ipfs-publish
ipfs-store-request-received = Primljen zahtjev za pohranu IPFS
ipfs-stored = Sadržaj pohranjen na IPFS
ipfs-store-cid-reply-sent = CID odgovor poslan
ipfs-store-resolve-failed = Nije moguće razriješiti pošiljatelja za dostavu odgovora ipfs-store

# Slanje entiteta
bootstrap-complete = Bootstrap dovršen
entity-loaded = Dodatak entiteta učitan
entity-load-failed = Učitavanje dodatka entiteta nije uspjelo
entity-not-found = Entitet nije pronađen, RPC se ignorira
entity-dispatched = RPC proslijeđen entitetu
entity-replied = Entitet je poslao RPC odgovor
root-create-entity = #root: stvori entitet
root-list-entities = #root: popis entiteta
root-delete-entity = #root: obriši entitet
root-entity-updated = Runtime manifest ažuriran
entity-created = Entitet stvoren
entity-deleted = Entitet obrisan
entity-states-saving = Spremanje stanja entiteta u IPFS
entity-state-saving = Spremanje stanja entiteta
entity-state-saved = Stanje entiteta spremljeno
entity-state-empty = Dodatak vratio prazno stanje, spremanje preskočeno
entity-states-saved = Stanja entiteta spremljena
link-set = Veza postavljena
ftl-loaded = Jezične poruke učitane iz IPFS

# Prvo pokretanje / auto-init
no-config-found = Konfiguracija nije pronađena.
initialising-new-identity = Inicijalizacija novog runtime identiteta.
generated-headless-config = Headless konfiguracija generirana.

# Vlasništvo
runtime-claimed = Runtime registriran.

# Zaštićeni korijenski elementi
refuse-delete-root = Odlučno odbijam brisanje obveznog korijenskog elementa
no-root-acl = Nije konfiguriran root ACL — runtime radi bez kontrole pristupa
acl-owners-access = Pozivaču je odobren pristup kao članu grupe +owners
namespace-not-found = Imenski prostor nije pronađen
no-ns-gate-acl = Za ovaj imenski prostor nije konfiguriran gate ACL
runtime-claim-persisted = Vlasnik zapisan u konfiguraciju.
runtime-already-claimed = Runtime je već registriran.


# Namespace creation (:create)
namespace-created = Prostor imena kreiran
namespace-already-exists = Prostor imena već postoji
namespace-name-reserved = Naziv prostora imena je rezerviran
namespace-create-denied = Stvaranje prostora imena: pristup odbijen
namespace-create-usage = Upotreba: :create <naziv>
crud-message-received = Primljena CRUD poruka
crud-acl-updated = Korijenski transportni ACL ažuriran

# CRUD validation errors
blob-value-ipfs-path = vrijednost blob mora biti IPFS putanja (/ipfs/, /ipns/ ili /ipld/)
acl-value-ipfs-path = vrijednost ACL mora biti IPFS putanja (/ipfs/, /ipns/ ili /ipld/)
kind-value-ipfs-path = vrijednost kind mora biti IPFS putanja (/ipfs/, /ipns/ ili /ipld/)
kind-not-found = Vrsta nije pronađena
cidv1-required = vrijednost mora biti goli CIDv1 (počinje s 'b'; CIDv0 'Qm…' nije prihvaćen)
config-key-protected = config ključ '%key%' je zaštićen
config-key-no-delete = daemon config ključ '%key%' ne može se brisati
config-key-not-manifest = config ključ '%key%' nije poznati manifest config ključ
wrong-crud-protocol = pogrešan CRUD protokol: %type%
entity-name-invalid = naziv entity mora biti ispisivi UTF-8
reserved-entity-name = naziv entity '%name%' je rezerviran
