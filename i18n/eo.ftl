# ma-runtime – Esperanto
lang-name = Esperanto

own-did-published = Propra DID-dokumento publikigita al IPNS
own-did-publish-failed = Malsukcesis publikigi propran DID-dokumenton
own-did-publish-timeout = Publikigo de propra DID-dokumento eksvalidiĝis post 2 minutoj
started = ma rultempo ekfunkciis
shutdown-requested = Ĉesigo petita
closing-endpoint = Fermante iroh-finpunkton...
shutdown-complete = Ĉesigo kompleta
status-listening = Statusa servilo aŭskultas
rpc-message-received = RPC-mesaĝo ricevita
rpc-message-rejected = RPC-mesaĝo malakceptita
ipfs-message-rejected = IPFS-mesaĝo malakceptita
ctrlc-handler-failed = Ctrl-C-traktilo malsukcesis
node-connected = Nodo konektita al protokolo
received-encrypted-ma-msg = Ricevita ĉifrita ma-msg sur /ma/ipfs/0.0.1
unknown-rpc-atom = Nekonata RPC-atomo, ignorita
rpc-not-text-atom = RPC-enhavo ne estas tekstatomo
rpc-unknown-verb = Nekonata RPC-verbo
rpc-reply-sent = RPC-respondo sendita
ping-received = :ping ricevita, sendante :pong
did-publish-request-received = Ricevita peto publikigi DID-dokumenton
document-published = Dokumento publikigita
did-publish-cid-reply-sent = CID-respondo por DID-publikigo sendita
did-publish-resolve-failed = Ne eblis solvi sendinton por liveri ipfs-publish-respondon
ipfs-store-request-received = Ricevita IPFS-stoka peto
ipfs-stored = Enhavo stokita en IPFS
ipfs-store-cid-reply-sent = CID-respondo sendita
ipfs-store-resolve-failed = Ne eblis solvi sendinton por liveri ipfs-store-respondon

# Entity dispatch
bootstrap-complete = Prastarigo kompleta
entity-loaded = Entiteca kromaĵo ŝargita
entity-load-failed = Malsukcesis ŝargi entitecan kromaĵon
entity-not-found = Entiteco ne trovita, RPC ignorita
entity-dispatched = RPC ekspedita al entiteco
entity-replied = Entiteco sendis RPC-respondon
root-create-entity = #root: krei entitecon
root-list-entities = #root: listigi entitecojn
root-delete-entity = #root: forigi entitecon
root-entity-updated = Rultempo-manifesto ĝisdatigita
entity-created = Entiteco kreita
entity-deleted = Entiteco forigita
entity-states-saving = Konservante entitecajn statojn al IPFS
entity-state-saving = Konservante entitecan staton
entity-state-saved = Entiteca stato konservita
entity-state-empty = Kromaĵo resendis malplenan staton, preterpasita
entity-states-saved = Entitecaj statoj konservitaj
link-set = Ligilo agordita
ftl-loaded = Lingvaj mesaĝoj ŝargitaj el IPFS

# First-run auto-init
no-config-found = Neniu agordo trovita.
initialising-new-identity = Prastarigo de nova rultempo-identeco.
generated-headless-config = Sendkapaĵa agordo generita.

# Ownership / claim
runtime-claimed = Rultempo reklamita.

# Protected root elements
refuse-delete-root = Decideme rifuzas forigi bezonatan radikeron
no-root-acl = Neniu radika ACL agordita — rultempo funkcias sen alir-kontrolo
acl-owners-access = Alvokanto ricevis aliron kiel membro de +owners
namespace-not-found = Nomspaco ne trovita
no-ns-gate-acl = Neniu pordeja ACL agordita por ĉi tiu nomspaco
runtime-claim-persisted = Posedanto registrita en agordo.
runtime-already-claimed = Rultempo jam reklamita.


# Namespace creation (:create)
namespace-created = Nomspaco kreita
namespace-already-exists = Nomspaco jam ekzistas
namespace-name-reserved = Nomspaca nomo estas rezervita
namespace-create-denied = Kreado de nomspaco: aliro malakceptita
namespace-create-usage = Uzado: :create <nomo>
crud-message-received = CRUD-mesaĝo ricevita
crud-acl-updated = Radika transporta ACL ĝisdatigita

# CRUD validation errors
blob-value-ipfs-path = la valoro de blob devas esti IPFS-vojo (/ipfs/, /ipns/, aŭ /ipld/)
acl-value-ipfs-path = la valoro de ACL devas esti IPFS-vojo (/ipfs/, /ipns/, aŭ /ipld/)
kind-value-ipfs-path = la valoro de kind devas esti IPFS-vojo (/ipfs/, /ipns/, aŭ /ipld/)
kind-not-found = La tipo ne troviĝis
cidv1-required = la valoro devas esti nuda CIDv1 (komencas per 'b'; CIDv0 'Qm…' ne akceptata)
config-key-protected = la agorda ŝlosilo '%key%' estas protektita
config-key-no-delete = la daemon-agorda ŝlosilo '%key%' ne povas esti forigita
config-key-not-manifest = la agorda ŝlosilo '%key%' ne estas konata manifest-agorda ŝlosilo
wrong-crud-protocol = malĝusta CRUD-protokolo: %type%
entity-name-invalid = la nomo de entity devas esti presebla UTF-8
reserved-entity-name = la nomo de entity '%name%' estas rezervita
