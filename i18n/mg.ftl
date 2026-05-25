# ma-runtime – Malagasy
lang-name = Malagasy

own-did-published = Ny antontan-taratasy DID ahy dia navoaka tao amin'ny IPNS
own-did-publish-failed = Tsy vitanay namoaka ny antontan-taratasy DID ahy
own-did-publish-timeout = Ny famoahana antontan-taratasy DID ahy dia afaka fotoana rehefa afaka 2 minitra
started = Nanomboka ny ma runtime
shutdown-requested = Nangataka ny fitoahana
closing-endpoint = Mikatona iroh endpoint...
shutdown-complete = Vita ny fitoahana
status-listening = Mangataka ny mpandrindra fandinihana
rpc-message-received = Voaray ny hafatra RPC
rpc-message-rejected = Nandàvana ny hafatra RPC
ipfs-message-rejected = Nandàvana ny hafatra IPFS
ctrlc-handler-failed = Tsy nahomby ny mpitantana Ctrl-C
node-connected = Ny fehezan-teny dia nampifandray amin'ny drafitra
received-encrypted-ma-msg = Voaray hafatra ma voasokafana tao /ma/ipfs/0.0.1
unknown-rpc-atom = Atom RPC tsy fantatra, tsy karakaraina
rpc-not-text-atom = Ny entana RPC tsy atoma lahatsoratra
rpc-unknown-verb = Tsy fantatra ny baiko RPC
rpc-reply-sent = Nalefa ny valiny RPC
ping-received = Voaray :ping, alefa :pong
did-publish-request-received = Voaray fangatahana famoahana antontan-taratasy DID
document-published = Navoaka ny antontan-taratasy
did-publish-cid-reply-sent = Nalefa ny valiny CID ho an'ny famoahana DID
did-publish-resolve-failed = Tsy vitanay namaha ny mpandefitra mba hanondro ny valiny ipfs-publish
ipfs-store-request-received = Voaray fangatahana fitahirizana IPFS
ipfs-stored = Voatahiry tao amin'ny IPFS ny votoatiny
ipfs-store-cid-reply-sent = Nalefa ny valiny CID
ipfs-store-resolve-failed = Tsy vitanay namaha ny mpandefitra mba hanondro ny valiny ipfs-store

# Fanatiterahan'ny entity
bootstrap-complete = Vita ny Bootstrap
entity-loaded = Loaded ny plugin entity
entity-load-failed = Tsy vitanay ny nanidy ny plugin entity
entity-not-found = Tsy hita ny entity, tsy karakarainy ny RPC
entity-dispatched = Nalefa RPC ho an'ny entity
entity-replied = Niresaka ny entity tamin'ny valin'ny RPC
root-create-entity = #root: mamorona entity
root-list-entities = #root: lisitry ny entity
root-delete-entity = #root: mamafa entity
root-entity-updated = Nohavaozina ny manifesto runtime
entity-created = Noforonina ny entity
entity-deleted = Nafana ny entity
entity-states-saving = Mitahiry ny toe-javatra entity any amin'ny IPFS
entity-state-saving = Mitahiry ny toe-java-tsy-misy entity
entity-state-saved = Voatahiry ny toe-java-tsy-misy entity
entity-state-empty = Naverina tsy misy ny plugin, navela ny fitahirizana
entity-states-saved = Voatahiry ny toe-javatra entity
link-set = Voapetraka ny rohy
ftl-loaded = Loaded ny hafatra fiteny avy amin'ny IPFS

# Fiantombohana voalohany / auto-init
no-config-found = Tsy hita ny fanakianana.
initialising-new-identity = Manomboka identity runtime vaovao.
generated-headless-config = Noforonina ny fanakianana headless.

# Fananana
runtime-claimed = Voasoratra ny runtime.

# Singa fototra voaaro
refuse-delete-root = Mandà mafy ny mamafa ny singa fototra ilaina
no-root-acl = Tsy voapetraka ny ACL fototra — miasa ny runtime tsy misy fitarihan'ny fidirana
acl-owners-access = Nomena alalana ny mpiantso amin'ny maha-mpikambana +owners azy
namespace-not-found = Tsy hita ny namespace
no-ns-gate-acl = Tsy voapetraka ny ACL vavahady ho an'ity namespace ity
runtime-claim-persisted = Voasoratra tao amin'ny fanakianana ny tompon'andraikitra.
runtime-already-claimed = Efa voasoratra ny runtime.


# Namespace creation (:create)
namespace-created = Namespace noforonina
namespace-already-exists = Efa misy ny namespace
namespace-name-reserved = Ny anarana namespace voatokana
namespace-create-denied = Famoronana namespace: toherana ny fidirana
namespace-create-usage = Fampiasana: :create <anarana>
crud-message-received = Voaraisina ny hafatra CRUD
crud-acl-updated = Navao ny ACL fitaterana fototra

# CRUD validation errors
blob-value-ipfs-path = ny soatoavina blob dia tsy maintsy lalana IPFS (/ipfs/, /ipns/, na /ipld/)
acl-value-ipfs-path = ny soatoavina ACL dia tsy maintsy lalana IPFS (/ipfs/, /ipns/, na /ipld/)
kind-value-ipfs-path = ny soatoavina kind dia tsy maintsy lalana IPFS (/ipfs/, /ipns/, na /ipld/)
kind-not-found = Tsy hita ny karazana
cidv1-required = ny sanda dia tokony ho CIDv1 tsotra (manomboka amin'ny 'b'; CIDv0 'Qm…' tsy voaray)
config-key-protected = ny fanalahidin'ny config '%key%' dia voaro
config-key-no-delete = ny fanalahidin'ny daemon config '%key%' dia tsy azo esorina
config-key-not-manifest = ny fanalahidin'ny config '%key%' dia tsy fanalahidy manifest config fantatra
wrong-crud-protocol = diso ny CRUD protocol: %type%
entity-name-invalid = ny anaran'ny entity dia tsy maintsy UTF-8 azo atonta
reserved-entity-name = ny anaran'ny entity '%name%' dia voatokana
