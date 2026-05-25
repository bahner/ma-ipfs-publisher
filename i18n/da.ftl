# ma-runtime – Dansk
lang-name = Dansk

own-did-published = Eget DID-dokument publiceret på IPNS
own-did-publish-failed = Publicering af eget DID-dokument mislykkedes
own-did-publish-timeout = Publicering af eget DID-dokument udløb efter 2 minutter
started = ma runtime startet
shutdown-requested = Nedlukning anmodet
closing-endpoint = Lukker iroh-slutpunkt...
shutdown-complete = Nedlukning fuldført
status-listening = Statusserver lytter
rpc-message-received = RPC-besked modtaget
rpc-message-rejected = RPC-besked afvist
ipfs-message-rejected = IPFS-besked afvist
ctrlc-handler-failed = Ctrl-C-håndtering mislykkedes
node-connected = Node tilsluttet protokol
received-encrypted-ma-msg = Krypteret ma-besked modtaget på /ma/ipfs/0.0.1
unknown-rpc-atom = Ukendt RPC-atom, ignoreres
rpc-not-text-atom = RPC-indhold er ikke et tekstatom
rpc-unknown-verb = Ukendt RPC-verb
rpc-reply-sent = RPC-svar sendt
ping-received = :ping modtaget, sender :pong
did-publish-request-received = Anmodning om publicering af DID-dokument modtaget
document-published = Dokument publiceret
did-publish-cid-reply-sent = CID-svar sendt for DID-publicering
did-publish-resolve-failed = Kunne ikke opløse afsender for at levere ipfs-publish-svar
ipfs-store-request-received = IPFS-lagringsanmodning modtaget
ipfs-stored = Indhold gemt på IPFS
ipfs-store-cid-reply-sent = CID-svar sendt
ipfs-store-resolve-failed = Kunne ikke opløse afsender for at levere ipfs-store-svar

# Enhedsafsendelse
bootstrap-complete = Bootstrap fuldført
entity-loaded = Enhedsplugin indlæst
entity-load-failed = Indlæsning af enhedsplugin mislykkedes
entity-not-found = Enhed ikke fundet, RPC ignoreres
entity-dispatched = RPC videresendt til enhed
entity-replied = Enhed sendte RPC-svar
root-create-entity = #root: opret enhed
root-list-entities = #root: list enheder
root-delete-entity = #root: slet enhed
root-entity-updated = Runtime-manifest opdateret
entity-created = Enhed oprettet
entity-deleted = Enhed slettet
entity-states-saving = Gemmer enhedstilstande til IPFS
entity-state-saving = Gemmer enhedstilstand
entity-state-saved = Enhedstilstand gemt
entity-state-empty = Plugin returnerede tom tilstand, gemning springes over
entity-states-saved = Enhedstilstande gemt
link-set = Link sat
ftl-loaded = Sprogbeskeder indlæst fra IPFS

# Første start / auto-init
no-config-found = Ingen konfiguration fundet.
initialising-new-identity = Initialiserer ny runtime-identitet.
generated-headless-config = Headless-konfiguration genereret.

# Ejerskab
runtime-claimed = Runtime registreret.

# Beskyttede rodelementer
refuse-delete-root = Nægter bestemt at slette et påkrævet rodelement
no-root-acl = Ingen rod-ACL konfigureret — runtime kører uden adgangskontrol
acl-owners-access = Den kaldende part fik adgang som medlem af +owners
namespace-not-found = Navnerum ikke fundet
no-ns-gate-acl = Ingen gate-ACL konfigureret for dette navnerum
runtime-claim-persisted = Ejer skrevet til konfiguration.
runtime-already-claimed = Runtime er allerede registreret.


# Namespace creation (:create)
namespace-created = Navnerum oprettet
namespace-already-exists = Navnerum eksisterer allerede
namespace-name-reserved = Navnerumsnavn er reserveret
namespace-create-denied = Navnerum oprettelse: adgang nægtet
namespace-create-usage = Brug: :create <navn>
crud-message-received = CRUD-besked modtaget
crud-acl-updated = Root-transport-ACL opdateret

# CRUD validation errors
blob-value-ipfs-path = blob-værdien skal være en IPFS-sti (/ipfs/, /ipns/ eller /ipld/)
acl-value-ipfs-path = ACL-værdien skal være en IPFS-sti (/ipfs/, /ipns/ eller /ipld/)
kind-value-ipfs-path = kind-værdien skal være en IPFS-sti (/ipfs/, /ipns/ eller /ipld/)
kind-not-found = Type ikke fundet
cidv1-required = værdien skal være en ren CIDv1 (starter med 'b'; CIDv0 'Qm…' accepteres ikke)
config-key-protected = konfigurationsnøglen '%key%' er beskyttet
config-key-no-delete = daemon-konfigurationsnøglen '%key%' kan ikke slettes
config-key-not-manifest = konfigurationsnøglen '%key%' er ikke en kendt manifest config-nøgle
wrong-crud-protocol = forkert CRUD-protokoll: %type%
entity-name-invalid = entity-navn skal være udskrivbart UTF-8
reserved-entity-name = entity-navn '%name%' er reserveret
