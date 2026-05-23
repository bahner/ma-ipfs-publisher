# ma-runtime – Norsk Bokmål
lang-name = Norsk bokmål

own-did-published = Eget DID-dokument publisert til IPNS
own-did-publish-failed = Kunne ikke publisere eget DID-dokument
own-did-publish-timeout = Timeout ved publisering av eget DID-dokument (2 min)
started = ma runtime startet
shutdown-requested = Avslutning forespurt
closing-endpoint = Lukker iroh-endepunkt ...
shutdown-complete = Avslutning fullført
status-listening = Statusserver lytter
rpc-message-received = Mottok RPC-melding
rpc-message-rejected = RPC-melding avvist
ipfs-message-rejected = IPFS-melding avvist
ctrlc-handler-failed = Ctrl-C-behandler feilet
node-connected = Node koblet til protokoll
received-encrypted-ma-msg = Mottok kryptert ma-melding på /ma/ipfs/0.0.1
unknown-rpc-atom = Ukjent RPC-atom, ignorerer
rpc-reply-sent = RPC-svar sendt
ping-received = Mottok :ping, sender :pong
did-publish-request-received = Mottok forespørsel om publisering av dokument
document-published = Dokument publisert
did-publish-cid-reply-sent = Sendt CID-svar for DID-publisering
did-publish-resolve-failed = Kunne ikke løse opp avsender for ipfs-publish-svar
ipfs-store-request-received = Mottok IPFS store-forespørsel
ipfs-stored = Lagret innhold på IPFS
ipfs-store-cid-reply-sent = Sendt CID-svar
ipfs-store-resolve-failed = Kunne ikke løse opp avsender for ipfs-store-svar

# Enhetsutsending
bootstrap-complete = Bootstrap fullført
entity-loaded = Enhetsplugin lastet
entity-load-failed = Feil ved lasting av enhetsplugin
entity-not-found = Enhet ikke funnet, ignorerer RPC
entity-dispatched = RPC sendt til enhet
entity-replied = Enhet sendte RPC-svar
root-create-entity = #root: opprett enhet
root-list-entities = #root: list enheter
root-delete-entity = #root: slett enhet
root-entity-updated = Runtime-manifest oppdatert
entity-created = Enhet opprettet
entity-deleted = Enhet slettet
entity-states-saving = Lagrer enhetstilstander til IPFS
entity-state-saving = Lagrer enhetstilstand
entity-state-saved = Enhetstilstand lagret
entity-state-empty = Plugin returnerte tom tilstand, hopper over lagring
entity-states-saved = Enhetstilstander lagret
link-set = Lenke satt
ftl-loaded = Språkmeldinger lastet fra IPFS

# Første-gangs auto-oppsett
no-config-found = Ingen konfigurasjon funnet.
initialising-new-identity = Oppretter ny runtime-identitet.
generated-headless-config = Genererte hodeløs konfigurasjon.

# Eierskap / krav
runtime-claimed = Runtime registrert.

# Beskyttede rotelementer
refuse-delete-root = Nekter bestemt å slette påkrevd rotelement
no-root-acl = Ingen rot-ACL er konfigurert — kjøretiden opererer uten tilgangskontroll
namespace-not-found = Navnerommet ble ikke funnet
no-ns-gate-acl = Ingen port-ACL er konfigurert for dette navnerommet
runtime-claim-persisted = Eier skrevet til konfigurasjon.
runtime-already-claimed = Runtime er allerede registrert.


# Namespace creation (:create)
acl-missing-owners = Rot-ACL-en må inneholde en +owners-oppføring
namespace-created = Navnerom opprettet
namespace-already-exists = Navnerommet eksisterer allerede
namespace-name-reserved = Reservert navneromsnavn
namespace-create-denied = Navnerom oppretting: tilgang nektet
namespace-create-usage = Bruk: :create <navn>
