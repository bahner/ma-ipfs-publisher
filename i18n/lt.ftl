# ma-runtime – Lietuvių
lang-name = Lietuvių

own-did-published = Savo DID dokumentas paskelbtas IPNS
own-did-publish-failed = Nepavyko paskelbti savo DID dokumento
own-did-publish-timeout = Savo DID dokumento skelbimas baigė laiką po 2 min.
started = ma runtime paleistas
shutdown-requested = Išjungimas paprašytas
closing-endpoint = Uždaromas iroh galinis taškas...
shutdown-complete = Išjungimas baigtas
status-listening = Būsenos serveris klauso
rpc-message-received = Gauta RPC žinutė
rpc-message-rejected = RPC žinutė atmesta
ipfs-message-rejected = IPFS žinutė atmesta
ctrlc-handler-failed = Ctrl-C tvarkytuvas nepavyko
node-connected = Mazgas prisijungė prie protokolo
received-encrypted-ma-msg = Gauta užšifruota ma žinutė /ma/ipfs/0.0.1
unknown-rpc-atom = Nežinomas RPC atomas, ignoruojama
rpc-not-text-atom = RPC duomenys nėra teksto atomas
rpc-unknown-verb = Nežinoma RPC komanda
rpc-reply-sent = RPC atsakymas išsiųstas
ping-received = Gautas :ping, siunčiu :pong
did-publish-request-received = Gauta DID dokumento skelbimo užklausa
document-published = Dokumentas paskelbtas
did-publish-cid-reply-sent = CID atsakymas išsiųstas DID skelbimui
did-publish-resolve-failed = Nepavyko išspręsti siuntėjo ipfs-publish atsakymui įteikti
ipfs-store-request-received = Gauta IPFS saugojimo užklausa
ipfs-stored = Turinys išsaugotas IPFS
ipfs-store-cid-reply-sent = CID atsakymas išsiųstas
ipfs-store-resolve-failed = Nepavyko išspręsti siuntėjo ipfs-store atsakymui įteikti

# Subjektų siuntimas
bootstrap-complete = Bootstrap baigtas
entity-loaded = Subjekto papildinys įkeltas
entity-load-failed = Nepavyko įkelti subjekto papildinio
entity-not-found = Subjektas nerastas, RPC ignoruojama
entity-dispatched = RPC perduota subjektui
entity-replied = Subjektas išsiuntė RPC atsakymą
root-create-entity = #root: sukurti subjektą
root-list-entities = #root: subjektų sąrašas
root-delete-entity = #root: ištrinti subjektą
root-entity-updated = Runtime manifestas atnaujintas
entity-created = Subjektas sukurtas
entity-deleted = Subjektas ištrintas
entity-states-saving = Subjektų būsenų išsaugojimas į IPFS
entity-state-saving = Subjekto būsenos išsaugojimas
entity-state-saved = Subjekto būsena išsaugota
entity-state-empty = Papildinys grąžino tuščią būseną, saugojimas praleistas
entity-states-saved = Subjektų būsenos išsaugotos
link-set = Nuoroda nustatyta
ftl-loaded = Kalbos žinutės įkeltos iš IPFS

# Pirmas paleidimas / auto-init
no-config-found = Konfigūracija nerasta.
initialising-new-identity = Inicijuojamas naujas runtime tapatumas.
generated-headless-config = Sugeneruota be galvos konfigūracija.

# Nuosavybė
runtime-claimed = Runtime užregistruotas.

# Apsaugoti šakniniai elementai
refuse-delete-root = Ryžtingai atsisakau ištrinti reikiamą šakninį elementą
no-root-acl = Šakninis ACL nesukonfigūruotas — runtime veikia be prieigos kontrolės
acl-owners-access = Skambinančiajam suteikta prieiga kaip +owners nario
namespace-not-found = Vardų erdvė nerasta
no-ns-gate-acl = Šiai vardų erdvei nesukonfigūruotas gate-ACL
runtime-claim-persisted = Savininkas įrašytas į konfigūraciją.
runtime-already-claimed = Runtime jau užregistruotas.


# Namespace creation (:create)
namespace-created = Vardų erdvė sukurta
namespace-already-exists = Vardų erdvė jau egzistuoja
namespace-name-reserved = Vardų erdvės pavadinimas yra rezervuotas
namespace-create-denied = Vardų erdvės sukūrimas: prieiga atmesta
namespace-create-usage = Naudojimas: :create <vardas>
crud-message-received = Gauta CRUD žinutė
crud-acl-updated = Šakninis transporto ACL atnaujintas

# CRUD validation errors
blob-value-ipfs-path = blob reikšmė turi būti IPFS kelias (/ipfs/, /ipns/ arba /ipld/)
acl-value-ipfs-path = ACL reikšmė turi būti IPFS kelias (/ipfs/, /ipns/ arba /ipld/)
kind-value-ipfs-path = kind reikšmė turi būti IPFS kelias (/ipfs/, /ipns/ arba /ipld/)
kind-not-found = Tipas nerastas
cidv1-required = reikšmė turi būti gryna CIDv1 (prasideda 'b'; CIDv0 'Qm…' nepriimamas)
config-key-protected = konfigūracijos raktas '%key%' yra apsaugotas
config-key-no-delete = daemon konfigūracijos rakto '%key%' negalima ištrinti
config-key-not-manifest = konfigūracijos raktas '%key%' nėra žinomas manifest config raktas
wrong-crud-protocol = neteisingas CRUD protokolas: %type%
entity-name-invalid = entity pavadinimas turi būti spausdinamas UTF-8
reserved-entity-name = entity pavadinimas '%name%' yra rezervuotas
