# ma-runtime – Íslenska
lang-name = Íslenska

own-did-published = Eigið DID-skjal birt á IPNS
own-did-publish-failed = Tókst ekki að birta eigið DID-skjal
own-did-publish-timeout = Birting eigins DID-skjals rann út eftir 2 mínútur
started = ma runtime ræst
shutdown-requested = Lokun beðin
closing-endpoint = Loka iroh-endapunkti...
shutdown-complete = Lokun lokið
status-listening = Stöðuþjónn hlustandi
rpc-message-received = RPC-skilaboð móttekin
rpc-message-rejected = RPC-skilaboð hafnað
ipfs-message-rejected = IPFS-skilaboð hafnað
ctrlc-handler-failed = Ctrl-C-meðhöndlun mistókst
node-connected = Hnútur tengdur við samskiptareglu
received-encrypted-ma-msg = Dulkóðuð ma-skilaboð móttekin á /ma/ipfs/0.0.1
unknown-rpc-atom = Óþekkt RPC-atóm, hunsa
rpc-not-text-atom = RPC innihald er ekki textafrumeind
rpc-unknown-verb = Óþekkt RPC sögn
rpc-reply-sent = RPC-svar sent
ping-received = :ping móttekin, sendi :pong
did-publish-request-received = Beiðni um birtingu DID-skjals móttekin
document-published = Skjal birt
did-publish-cid-reply-sent = CID-svar sent fyrir DID-birtingu
did-publish-resolve-failed = Tókst ekki að leysa upp sendanda til að afhenda ipfs-publish-svar
ipfs-store-request-received = IPFS-geymslu beiðni móttekin
ipfs-stored = Efni geymt á IPFS
ipfs-store-cid-reply-sent = CID-svar sent
ipfs-store-resolve-failed = Tókst ekki að leysa upp sendanda til að afhenda ipfs-store-svar

# Sending eininga
bootstrap-complete = Bootstrap lokið
entity-loaded = Einingaviðbót hlaðin
entity-load-failed = Hlöðun einingaviðbótar mistókst
entity-not-found = Eining finnst ekki, RPC hunsuð
entity-dispatched = RPC sent til einingar
entity-replied = Eining sendi RPC-svar
root-create-entity = #root: búa til einingu
root-list-entities = #root: listi yfir einingar
root-delete-entity = #root: eyða einingu
root-entity-updated = Runtime-skilabréf uppfært
entity-created = Eining búin til
entity-deleted = Eining eyðilögð
entity-states-saving = Vista stöður eininga í IPFS
entity-state-saving = Vista stöðu einingar
entity-state-saved = Staða einingar vistuð
entity-state-empty = Viðbót skilaði tómri stöðu, vistun sleppt
entity-states-saved = Stöður eininga vistaðar
link-set = Hlekkur stilltur
ftl-loaded = Tungumálaskilaboð hlaðin frá IPFS

# Fyrsta keyrsla / sjálfvirk uppsetning
no-config-found = Engin stilling fundist.
initialising-new-identity = Frumstilli nýja runtime-auðkenni.
generated-headless-config = Höfuðlaus stilling mynduð.

# Eignarréttur
runtime-claimed = Runtime skráð.

# Vernduð rótareiningar
refuse-delete-root = Neitaði eindregið að eyða nauðsynlegri rótareiningu
no-root-acl = Engin rót-ACL stillt — runtime keyrir án aðgangsstýringar
acl-owners-access = Kallinn fékk aðgang sem meðlimur í +owners
namespace-not-found = Nafnasvæði finnst ekki
no-ns-gate-acl = Engin gate-ACL stillt fyrir þetta nafnasvæði
runtime-claim-persisted = Eigandi skrifaður í stillingar.
runtime-already-claimed = Runtime er þegar skráð.


# Namespace creation (:create)
namespace-created = Nafnrými búið til
namespace-already-exists = Nafnrými er þegar til
namespace-name-reserved = Nafn nafnrýmis er frátekið
namespace-create-denied = Stofnun nafnrýmis: aðgangur hafnað
namespace-create-usage = Notkun: :create <nafn>
crud-message-received = CRUD skilaboð móttekin
crud-acl-updated = Rót-flutnings-ACL uppfærð

# CRUD validation errors
blob-value-ipfs-path = blob-gildið verður að vera IPFS-slóð (/ipfs/, /ipns/ eða /ipld/)
acl-value-ipfs-path = ACL-gildið verður að vera IPFS-slóð (/ipfs/, /ipns/ eða /ipld/)
kind-value-ipfs-path = kind-gildið verður að vera IPFS-slóð (/ipfs/, /ipns/ eða /ipld/)
kind-not-found = Tegund fannst ekki
cidv1-required = gildið verður að vera hreint CIDv1 (byrjar á 'b'; CIDv0 'Qm…' er ekki samþykkt)
config-key-protected = stillingarlykillinn '%key%' er varinn
config-key-no-delete = ekki er hægt að eyða stillingarlykli '%key%' þjónustunnar
config-key-not-manifest = stillingarlykillinn '%key%' er ekki þekktur manifest-stillingarlykill
wrong-crud-protocol = rangur CRUD-samskiptaregla: %type%
entity-name-invalid = entity-nafnið verður að vera prentanlegt UTF-8
reserved-entity-name = entity-nafnið '%name%' er frátekið
