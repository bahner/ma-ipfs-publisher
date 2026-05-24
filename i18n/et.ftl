# ma-runtime – Eesti
lang-name = Eesti

own-did-published = Oma DID dokument avaldatud IPNS-is
own-did-publish-failed = Oma DID dokumendi avaldamine ebaõnnestus
own-did-publish-timeout = Oma DID dokumendi avaldamine aegus 2 minuti pärast
started = ma runtime käivitatud
shutdown-requested = Seiskamine nõutud
closing-endpoint = iroh lõpp-punkti sulgemine...
shutdown-complete = Seiskamine lõpetatud
status-listening = Olekuserver kuulab
rpc-message-received = RPC sõnum saadud
rpc-message-rejected = RPC sõnum tagasi lükatud
ipfs-message-rejected = IPFS sõnum tagasi lükatud
ctrlc-handler-failed = Ctrl-C käsitleja ebaõnnestus
node-connected = Sõlm ühendatud protokolliga
received-encrypted-ma-msg = Krüptitud ma sõnum saadud /ma/ipfs/0.0.1 kaudu
unknown-rpc-atom = Tundmatu RPC aatom, ignoreerimine
rpc-not-text-atom = RPC andmed ei ole tekstiatoom
rpc-unknown-verb = Tundmatu RPC verb
rpc-reply-sent = RPC vastus saadetud
ping-received = :ping saadud, saadan :pong
did-publish-request-received = DID dokumendi avaldamise päring saadud
document-published = Dokument avaldatud
did-publish-cid-reply-sent = CID vastus saadetud DID avaldamise jaoks
did-publish-resolve-failed = Saatjat ei õnnestunud lahendada ipfs-publish vastuse edastamiseks
ipfs-store-request-received = IPFS salvestamise päring saadud
ipfs-stored = Sisu salvestatud IPFS-i
ipfs-store-cid-reply-sent = CID vastus saadetud
ipfs-store-resolve-failed = Saatjat ei õnnestunud lahendada ipfs-store vastuse edastamiseks

# Olemite saatmine
bootstrap-complete = Bootstrap lõpetatud
entity-loaded = Olemite plugin laaditud
entity-load-failed = Olemite plugini laadimine ebaõnnestus
entity-not-found = Olemit ei leitud, RPC ignoreerimine
entity-dispatched = RPC edastatud olemile
entity-replied = Olem saatis RPC vastuse
root-create-entity = #root: loo olem
root-list-entities = #root: olemite loend
root-delete-entity = #root: kustuta olem
root-entity-updated = Runtime manifest uuendatud
entity-created = Olem loodud
entity-deleted = Olem kustutatud
entity-states-saving = Olemite olekute salvestamine IPFS-i
entity-state-saving = Olemi oleku salvestamine
entity-state-saved = Olemi olek salvestatud
entity-state-empty = Plugin tagastas tühja oleku, salvestamine vahele jäetud
entity-states-saved = Olemite olekud salvestatud
link-set = Link seatud
ftl-loaded = Keeleteated laaditud IPFS-ist

# Esimene käivitus / auto-init
no-config-found = Konfiguratsiooni ei leitud.
initialising-new-identity = Uue runtime identiteedi initsialiseerimine.
generated-headless-config = Peavaba konfiguratsioon genereeritud.

# Omandiõigus
runtime-claimed = Runtime registreeritud.

# Kaitstud juureelemendid
refuse-delete-root = Keeldun kategooriliselt nõutava juureelemendi kustutamisest
no-root-acl = Juur-ACL pole konfigureeritud — runtime töötab ilma juurdepääsukontrollita
namespace-not-found = Nimeruumi ei leitud
no-ns-gate-acl = Selle nimeruumi jaoks pole gate-ACL konfigureeritud
runtime-claim-persisted = Omanik kirjutatud konfiguratsiooni.
runtime-already-claimed = Runtime on juba registreeritud.


# Namespace creation (:create)
acl-missing-owners = Juur-ACL peab sisaldama +owners kirjet
namespace-created = Nimeruum loodud
namespace-already-exists = Nimeruum juba eksisteerib
namespace-name-reserved = Nimeruumi nimi on reserveeritud
namespace-create-denied = Nimeruumi loomine: juurdepääs keelatud
namespace-create-usage = Kasutus: :create <nimi>
crud-message-received = CRUD-sõnum vastu võetud
crud-acl-updated = Juurtranspordi ACL uuendati
