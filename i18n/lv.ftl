# ma-runtime – Latviešu
lang-name = Latviešu

own-did-published = Savs DID dokuments publicēts IPNS
own-did-publish-failed = Neizdevās publicēt savu DID dokumentu
own-did-publish-timeout = Sava DID dokumenta publicēšana pārsniedza 2 min. taimauta
started = ma runtime palaists
shutdown-requested = Izslēgšana pieprasīta
closing-endpoint = Aizver iroh galapunktu...
shutdown-complete = Izslēgšana pabeigta
status-listening = Statusa serveris klausās
rpc-message-received = Saņemts RPC ziņojums
rpc-message-rejected = RPC ziņojums noraidīts
ipfs-message-rejected = IPFS ziņojums noraidīts
ctrlc-handler-failed = Ctrl-C apstrādātājs neizdevās
node-connected = Mezgls pievienojies protokolam
received-encrypted-ma-msg = Saņemts šifrēts ma ziņojums uz /ma/ipfs/0.0.1
unknown-rpc-atom = Nezināms RPC atoms, ignorēšana
rpc-not-text-atom = RPC krava nav teksta atoms
rpc-unknown-verb = Nezināms RPC darbības vārds
rpc-reply-sent = RPC atbilde nosūtīta
ping-received = Saņemts :ping, sūtu :pong
did-publish-request-received = Saņemts DID dokumenta publicēšanas pieprasījums
document-published = Dokuments publicēts
did-publish-cid-reply-sent = CID atbilde nosūtīta DID publicēšanai
did-publish-resolve-failed = Neizdevās atrisināt sūtītāju ipfs-publish atbildes piegādei
ipfs-store-request-received = Saņemts IPFS glabāšanas pieprasījums
ipfs-stored = Saturs saglabāts IPFS
ipfs-store-cid-reply-sent = CID atbilde nosūtīta
ipfs-store-resolve-failed = Neizdevās atrisināt sūtītāju ipfs-store atbildes piegādei

# Entitāšu nosūtīšana
bootstrap-complete = Bootstrap pabeigts
entity-loaded = Entitātes spraudnis ielādēts
entity-load-failed = Entitātes spraudņa ielāde neizdevās
entity-not-found = Entitāte nav atrasta, RPC ignorēts
entity-dispatched = RPC nosūtīts entitātei
entity-replied = Entitāte nosūtīja RPC atbildi
root-create-entity = #root: izveidot entitāti
root-list-entities = #root: entitāšu saraksts
root-delete-entity = #root: dzēst entitāti
root-entity-updated = Runtime manifests atjaunināts
entity-created = Entitāte izveidota
entity-deleted = Entitāte dzēsta
entity-states-saving = Saglabā entitāšu stāvokļus IPFS
entity-state-saving = Saglabā entitātes stāvokli
entity-state-saved = Entitātes stāvoklis saglabāts
entity-state-empty = Spraudnis atgrieza tukšu stāvokli, saglabāšana izlaista
entity-states-saved = Entitāšu stāvokļi saglabāti
link-set = Saite iestatīta
ftl-loaded = Valodas ziņojumi ielādēti no IPFS

# Pirmā palaišana / auto-init
no-config-found = Konfigurācija nav atrasta.
initialising-new-identity = Inicializē jaunu runtime identitāti.
generated-headless-config = Ģenerēta bezgalvas konfigurācija.

# Īpašumtiesības
runtime-claimed = Runtime reģistrēts.

# Aizsargātie saknes elementi
refuse-delete-root = Kategoriski atsakos dzēst nepieciešamo saknes elementu
no-root-acl = Saknes ACL nav konfigurēts — runtime darbojas bez piekļuves kontroles
acl-owners-access = Zvanītājam piešķirta piekļuve kā +owners dalībniekam
namespace-not-found = Nosaukumvieta nav atrasta
no-ns-gate-acl = Šai nosaukumvietai nav konfigurēts gate-ACL
runtime-claim-persisted = Īpašnieks ierakstīts konfigurācijā.
runtime-already-claimed = Runtime jau ir reģistrēts.


# Namespace creation (:create)
namespace-created = Nosaukumvieta izveidota
namespace-already-exists = Nosaukumvieta jau pastāv
namespace-name-reserved = Nosaukumvietas nosaukums ir rezervēts
namespace-create-denied = Nosaukumvietas izveide: piekļuve liegta
namespace-create-usage = Lietošana: :create <nosaukums>
crud-message-received = Saņemts CRUD ziņojums
crud-acl-updated = Saknes transporta ACL atjaunināts

# CRUD validation errors
blob-value-ipfs-path = blob vērtībai jābūt IPFS ceļam (/ipfs/, /ipns/ vai /ipld/)
acl-value-ipfs-path = ACL vērtībai jābūt IPFS ceļam (/ipfs/, /ipns/ vai /ipld/)
kind-value-ipfs-path = kind vērtībai jābūt IPFS ceļam (/ipfs/, /ipns/ vai /ipld/)
kind-not-found = Tips nav atrasts
cidv1-required = vērtībai jābūt neapstrādātam CIDv1 (sākas ar 'b'; CIDv0 'Qm…' netiek pieņemts)
config-key-protected = konfigurācijas atslēga '%key%' ir aizsargāta
config-key-no-delete = daemon konfigurācijas atslēgu '%key%' nevar dzēst
config-key-not-manifest = konfigurācijas atslēga '%key%' nav zināma manifest config atslēga
wrong-crud-protocol = nepareizs CRUD protokols: %type%
entity-name-invalid = entity nosaukumam jābūt drukājamam UTF-8
reserved-entity-name = entity nosaukums '%name%' ir rezervēts
