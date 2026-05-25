# ma-runtime – Suomi
lang-name = Suomi

own-did-published = Oma DID-asiakirja julkaistu IPNS:ään
own-did-publish-failed = Oman DID-asiakirjan julkaisu epäonnistui
own-did-publish-timeout = Oman DID-asiakirjan julkaisu aikakatkaistiin 2 min jälkeen
started = ma runtime käynnistetty
shutdown-requested = Sammutus pyydetty
closing-endpoint = Suljetaan iroh-päätepistettä...
shutdown-complete = Sammutus valmis
status-listening = Tilapalvelin kuuntelee
rpc-message-received = RPC-viesti vastaanotettu
rpc-message-rejected = RPC-viesti hylätty
ipfs-message-rejected = IPFS-viesti hylätty
ctrlc-handler-failed = Ctrl-C-käsittelijä epäonnistui
node-connected = Solmu yhdistetty protokollaan
received-encrypted-ma-msg = Salattu ma-viesti vastaanotettu osoitteessa /ma/ipfs/0.0.1
unknown-rpc-atom = Tuntematon RPC-atomi, ohitetaan
rpc-not-text-atom = RPC-sanoma ei ole tekstiatomi
rpc-unknown-verb = Tuntematon RPC-verbi
rpc-reply-sent = RPC-vastaus lähetetty
ping-received = :ping vastaanotettu, lähetetään :pong
did-publish-request-received = DID-asiakirjan julkaisupyyntö vastaanotettu
document-published = Asiakirja julkaistu
did-publish-cid-reply-sent = CID-vastaus lähetetty DID-julkaisua varten
did-publish-resolve-failed = Lähettäjän selvittäminen epäonnistui ipfs-publish-vastauksen toimittamiseksi
ipfs-store-request-received = IPFS-tallennuspyyntö vastaanotettu
ipfs-stored = Sisältö tallennettu IPFS:ään
ipfs-store-cid-reply-sent = CID-vastaus lähetetty
ipfs-store-resolve-failed = Lähettäjän selvittäminen epäonnistui ipfs-store-vastauksen toimittamiseksi

# Entiteettien välitys
bootstrap-complete = Bootstrap valmis
entity-loaded = Entiteetin laajennus ladattu
entity-load-failed = Entiteetin laajennuksen lataus epäonnistui
entity-not-found = Entiteettiä ei löydy, RPC ohitetaan
entity-dispatched = RPC välitetty entiteetille
entity-replied = Entiteetti lähetti RPC-vastauksen
root-create-entity = #root: luo entiteetti
root-list-entities = #root: entiteettien luettelo
root-delete-entity = #root: poista entiteetti
root-entity-updated = Runtime-manifesti päivitetty
entity-created = Entiteetti luotu
entity-deleted = Entiteetti poistettu
entity-states-saving = Tallennetaan entiteettien tiloja IPFS:ään
entity-state-saving = Tallennetaan entiteetin tilaa
entity-state-saved = Entiteetin tila tallennettu
entity-state-empty = Laajennus palautti tyhjän tilan, tallennus ohitettu
entity-states-saved = Entiteettien tilat tallennettu
link-set = Linkki asetettu
ftl-loaded = Kielitiedotteet ladattu IPFS:stä

# Ensimmäinen käynnistys / auto-init
no-config-found = Konfiguraatiota ei löydy.
initialising-new-identity = Alustetaan uutta runtime-identiteettiä.
generated-headless-config = Päätön konfiguraatio luotu.

# Omistajuus
runtime-claimed = Runtime rekisteröity.

# Suojatut juurielementit
refuse-delete-root = Kieltäydyn ehdottomasti poistamasta vaadittua juurielementtiä
no-root-acl = Juuri-ACL ei ole määritetty — runtime toimii ilman pääsynhallintaa
acl-owners-access = Kutsujalle myönnettiin pääsy +owners-ryhmän jäsenenä
namespace-not-found = Nimiavaruutta ei löydy
no-ns-gate-acl = Tälle nimiavaruudelle ei ole määritetty gate-ACL:ää
runtime-claim-persisted = Omistaja kirjoitettu konfiguraatioon.
runtime-already-claimed = Runtime on jo rekisteröity.


# Namespace creation (:create)
namespace-created = Nimiavaruus luotu
namespace-already-exists = Nimiavaruus on jo olemassa
namespace-name-reserved = Nimiavaruuden nimi on varattu
namespace-create-denied = Nimiavaruuden luonti: pääsy kielletty
namespace-create-usage = Käyttö: :create <nimi>
crud-message-received = CRUD-viesti vastaanotettu
crud-acl-updated = Juuri-kuljetuksen ACL päivitetty

# CRUD validation errors
blob-value-ipfs-path = blob-arvon on oltava IPFS-polku (/ipfs/, /ipns/ tai /ipld/)
acl-value-ipfs-path = ACL-arvon on oltava IPFS-polku (/ipfs/, /ipns/ tai /ipld/)
kind-value-ipfs-path = kind-arvon on oltava IPFS-polku (/ipfs/, /ipns/ tai /ipld/)
kind-not-found = Tyyppiä ei löydy
cidv1-required = arvo täytyy olla puhdas CIDv1 (alkaa 'b':llä; CIDv0 'Qm…' ei hyväksytä)
config-key-protected = config-avain '%key%' on suojattu
config-key-no-delete = daemon-config-avainta '%key%' ei voi poistaa
config-key-not-manifest = config-avain '%key%' ei ole tunnettu manifest config -avain
wrong-crud-protocol = väärä CRUD-protokolla: %type%
entity-name-invalid = entity-nimen täytyy olla tulostettavaa UTF-8
reserved-entity-name = entity-nimi '%name%' on varattu
