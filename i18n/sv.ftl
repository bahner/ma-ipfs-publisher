# ma-runtime – Svenska
lang-name = Svenska

own-did-published = Eget DID-dokument publicerat på IPNS
own-did-publish-failed = Misslyckades med att publicera eget DID-dokument
own-did-publish-timeout = Publicering av eget DID-dokument tog för lång tid efter 2 minuter
started = ma runtime startad
shutdown-requested = Avstängning begärd
closing-endpoint = Stänger iroh-slutpunkt...
shutdown-complete = Avstängning slutförd
status-listening = Statusserver lyssnar
rpc-message-received = RPC-meddelande mottaget
rpc-message-rejected = RPC-meddelande avvisat
ipfs-message-rejected = IPFS-meddelande avvisat
ctrlc-handler-failed = Ctrl-C-hanterare misslyckades
node-connected = Nod ansluten till protokoll
received-encrypted-ma-msg = Krypterat ma-meddelande mottaget på /ma/ipfs/0.0.1
unknown-rpc-atom = Okänd RPC-atom, ignoreras
rpc-reply-sent = RPC-svar skickat
ping-received = :ping mottaget, skickar :pong
did-publish-request-received = Begäran om publicering av DID-dokument mottagen
document-published = Dokument publicerat
did-publish-cid-reply-sent = CID-svar skickat för DID-publicering
did-publish-resolve-failed = Kunde inte lösa upp avsändare för att leverera ipfs-publish-svar
ipfs-store-request-received = IPFS-lagringsbegäran mottagen
ipfs-stored = Innehåll lagrat på IPFS
ipfs-store-cid-reply-sent = CID-svar skickat
ipfs-store-resolve-failed = Kunde inte lösa upp avsändare för att leverera ipfs-store-svar

# Entitetsutsändning
bootstrap-complete = Bootstrap slutförd
entity-loaded = Entitetsplugin laddad
entity-load-failed = Misslyckades med att ladda entitetsplugin
entity-not-found = Entitet ej hittad, ignorerar RPC
entity-dispatched = RPC vidarebefordrad till entitet
entity-replied = Entitet skickade RPC-svar
root-create-entity = #root: skapa entitet
root-list-entities = #root: lista entiteter
root-delete-entity = #root: ta bort entitet
root-entity-updated = Runtime-manifest uppdaterat
entity-created = Entitet skapad
entity-deleted = Entitet borttagen
entity-states-saving = Sparar entitetstillstånd till IPFS
entity-state-saving = Sparar entitetstillstånd
entity-state-saved = Entitetstillstånd sparat
entity-state-empty = Plugin returnerade tomt tillstånd, hoppar över sparning
entity-states-saved = Entitetstillstånd sparade
link-set = Länk inställd
ftl-loaded = Språkmeddelanden laddade från IPFS

# Första start / auto-init
no-config-found = Ingen konfiguration hittad.
initialising-new-identity = Initierar ny runtime-identitet.
generated-headless-config = Headless-konfiguration genererad.

# Äganderätt
runtime-claimed = Runtime registrerad.

# Skyddade rotelement
refuse-delete-root = Vägrar bestämt att ta bort ett obligatoriskt rotelement
no-root-acl = Ingen rot-ACL konfigurerad — runtime körs utan åtkomstkontroll
namespace-not-found = Namnrymd ej hittad
no-ns-gate-acl = Ingen gate-ACL konfigurerad för denna namnrymd
runtime-claim-persisted = Ägare skriven till konfiguration.
runtime-already-claimed = Runtime är redan registrerad.


# Namespace creation (:create)
acl-missing-owners = Rot-ACL måste innehålla en +owners-post
namespace-created = Namnrymd skapad
namespace-already-exists = Namnrymd finns redan
namespace-name-reserved = Namnrymdsnamnet är reserverat
namespace-create-denied = Skapa namnrymd: åtkomst nekad
namespace-create-usage = Användning: :create <namn>
