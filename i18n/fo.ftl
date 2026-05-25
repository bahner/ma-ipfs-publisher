# ma-runtime – Føroyskt
lang-name = Føroyskt

own-did-published = Egið DID-skjal birt á IPNS
own-did-publish-failed = Fáast ikki at birta egið DID-skjal
own-did-publish-timeout = Birting av egnum DID-skjali gjørdi timeout eftir 2 minuttir
started = ma runtime startað
shutdown-requested = Stansa búgvin
closing-endpoint = Lokkar iroh-endastøð...
shutdown-complete = Stansing liðug
status-listening = Støðutenar hloyðir
rpc-message-received = RPC-boð móttikið
rpc-message-rejected = RPC-boð avvísað
ipfs-message-rejected = IPFS-boð avvísað
ctrlc-handler-failed = Ctrl-C-handsamari brast
node-connected = Knútur knýttur til protokol
received-encrypted-ma-msg = Dulkódað ma-boð móttikið á /ma/ipfs/0.0.1
unknown-rpc-atom = Ókentur RPC-atom, humsað
rpc-not-text-atom = RPC-innihald er ikki eitt tekstatom
rpc-unknown-verb = Ókent RPC-sagnorð
rpc-reply-sent = RPC-svar sent
ping-received = :ping móttikið, sendi :pong
did-publish-request-received = Bøn um birting av DID-skjali móttøkin
document-published = Skjal birt
did-publish-cid-reply-sent = CID-svar sent fyri DID-birting
did-publish-resolve-failed = Ikki mett at finna sendaran til at skaffa ipfs-publish-svar
ipfs-store-request-received = IPFS-goymslubøn móttøkin
ipfs-stored = Innihald goymt á IPFS
ipfs-store-cid-reply-sent = CID-svar sent
ipfs-store-resolve-failed = Ikki mett at finna sendaran til at skaffa ipfs-store-svar

# Eindir send
bootstrap-complete = Bootstrap liðugt
entity-loaded = Eindarplugin lesin inn
entity-load-failed = Innlesing av eindarplugin brast
entity-not-found = Eindir ikki funnin, RPC humsað
entity-dispatched = RPC sent til eindir
entity-replied = Eindir sendi RPC-svar
root-create-entity = #root: stovna eindir
root-list-entities = #root: lista eindir
root-delete-entity = #root: strika eindir
root-entity-updated = Runtime-manifest dagfest
entity-created = Eindir stovnað
entity-deleted = Eindir strikað
entity-states-saving = Goymi eindarstøður til IPFS
entity-state-saving = Goymi eindarstøðu
entity-state-saved = Eindarstøða goymd
entity-state-empty = Plugin skilaði tómari støðu, goymsl sleppt
entity-states-saved = Eindarstøður goymd
link-set = Leinkja sett
ftl-loaded = Málboð lesin inn frá IPFS

# Fyrsta keyrsla / sjálvvirk uppsetning
no-config-found = Ongi stillingar funnin.
initialising-new-identity = Byrjar nýggja runtime-samsvarsmæligu.
generated-headless-config = Høvuðsleys stillingar gjørdar.

# Eivirði
runtime-claimed = Runtime skráð.

# Vernd rótareimindir
refuse-delete-root = Neitari ákveðið at strika kravd rótareindir
no-root-acl = Ongin rót-ACL stilltur — runtime keyrir uttan atgongustýring
acl-owners-access = Kallarin fekk atgongd sum limur av +owners
namespace-not-found = Navnarúm ikki funnið
no-ns-gate-acl = Ongin gate-ACL stilltur fyri hetta navnarúmið
runtime-claim-persisted = Eigari skrivað til stillingar.
runtime-already-claimed = Runtime er longu skráð.


# Namespace creation (:create)
namespace-created = Navnaróm stovnað
namespace-already-exists = Navnaróm er longu til
namespace-name-reserved = Navnarómsnavnið er frátekt
namespace-create-denied = Stovnan navnaróm: atgongd nektað
namespace-create-usage = Nýtsla: :create <navn>
crud-message-received = CRUD-boð móttikið
crud-acl-updated = Root-transport-ACL uppfært

# CRUD validation errors
blob-value-ipfs-path = blob-virðið skal vera ein IPFS-leið (/ipfs/, /ipns/ ella /ipld/)
acl-value-ipfs-path = ACL-virðið skal vera ein IPFS-leið (/ipfs/, /ipns/ ella /ipld/)
kind-value-ipfs-path = kind-virðið skal vera ein IPFS-leið (/ipfs/, /ipns/ ella /ipld/)
kind-not-found = Slag er ikki funnið
cidv1-required = gildi skal vera eitt reint CIDv1 (byrjar við 'b'; CIDv0 'Qm…' ikki góðteke)
config-key-protected = config-lykillinn '%key%' er verndaður
config-key-no-delete = daemon-config-lykillinn '%key%' kann ikki slettast
config-key-not-manifest = config-lykillinn '%key%' er ikki ein kendur manifest-config-lykill
wrong-crud-protocol = rang CRUD-protokoll: %type%
entity-name-invalid = entity-navnið skal vera prentbært UTF-8
reserved-entity-name = entity-navnið '%name%' er fyrirvara
