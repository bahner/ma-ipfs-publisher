# ma-runtime – isiXhosa
lang-name = isiXhosa

own-did-published = Uxwebhu lwam lwe-DID lwapapashwa kwi-IPNS
own-did-publish-failed = Ukupapasha uxwebhu lwe-DID lwam kwahlulekile
own-did-publish-timeout = Ukupapasha uxwebhu lwe-DID lwam kwaphela ixesha emva kwemizuzu emi-2
started = I-ma runtime iqalile
shutdown-requested = Icelwe ukuvala
closing-endpoint = Iyavala i-iroh endpoint...
shutdown-complete = Ukuvala kuphelile
status-listening = Iseva yemeko iyaphulaphula
rpc-message-received = Umyalezo we-RPC wamukelwe
rpc-message-rejected = Umyalezo we-RPC waliwa
ipfs-message-rejected = Umyalezo we-IPFS waliwa
ctrlc-handler-failed = Umphathi we-Ctrl-C wohlulekile
node-connected = Inodi yaxhuma kuprotocol
received-encrypted-ma-msg = Umyalezo we-ma obethelelwe wamukelwe kwi-/ma/ipfs/0.0.1
unknown-rpc-atom = I-atom ye-RPC engaziwa, iyagatywa
rpc-reply-sent = Impendulo ye-RPC ithunyelwe
ping-received = I-:ping yamukelwe, ithunyelwa i-:pong
did-publish-request-received = Isicelo sokupapasha uxwebhu lwe-DID samukelwe
document-published = Uxwebhu lupapashiwe
did-publish-cid-reply-sent = Impendulo ye-CID yokupapasha i-DID ithunyelwe
did-publish-resolve-failed = Yehlulekile ukusombulula umthumeli ukubhalela impendulo ye-ipfs-publish
ipfs-store-request-received = Isicelo sokugcina i-IPFS samukelwe
ipfs-stored = Umxholo ugcinwe kwi-IPFS
ipfs-store-cid-reply-sent = Impendulo ye-CID ithunyelwe
ipfs-store-resolve-failed = Yehlulekile ukusombulula umthumeli ukubhalela impendulo ye-ipfs-store

# Ukuthumela i-entity
bootstrap-complete = I-Bootstrap iphelile
entity-loaded = I-plugin ye-entity ilayishiwe
entity-load-failed = Ukulayisha i-plugin ye-entity kuhlulekile
entity-not-found = I-entity ayifunyanwa, i-RPC igatywa
entity-dispatched = I-RPC ithunyelwe kwi-entity
entity-replied = I-entity ithumele impendulo ye-RPC
root-create-entity = #root: yenza i-entity
root-list-entities = #root: uluhlu lwe-entity
root-delete-entity = #root: cima i-entity
root-entity-updated = I-manifest ye-runtime ihlaziyiwe
entity-created = I-entity yenziwe
entity-deleted = I-entity icinyiwe
entity-states-saving = Igcina iimeko ze-entity kwi-IPFS
entity-state-saving = Igcina imeko ye-entity
entity-state-saved = Imeko ye-entity igcinwe
entity-state-empty = I-plugin ibuyise imeko engenanto, ukugcina kwagxothwa
entity-states-saved = Iimeko ze-entity zigcinwe
link-set = Isixhomekeko sibekwe
ftl-loaded = Imiyalezo yolwimi ilayishwe kwi-IPFS

# Ukuqala kwesihlandlo sokuqala / auto-init
no-config-found = Akufumaneka malungiselelo.
initialising-new-identity = Iyaqalisa i-identity entsha ye-runtime.
generated-headless-config = Iveliswe i-configuration ye-headless.

# Ubumnini
runtime-claimed = I-runtime ibhaliswe.

# Izinto zengcambu ezikhusiweyo
refuse-delete-root = Ndiyala ngokuqinileyo ukucima into yengcambu efunekayo
no-root-acl = I-ACL yengcambu ayihonjiswa — i-runtime iyasebenza ngaphandle kolawulo lokungena
namespace-not-found = I-namespace ayifumaneki
no-ns-gate-acl = I-ACL yesango ayihonjiswa kulo i-namespace
runtime-claim-persisted = Umninimzi ubhalwe kumalungiselelo.
runtime-already-claimed = I-runtime seyibhaliswe.


# Namespace creation (:create)
acl-missing-owners = I-ACL yengcambu kufuneka ibe ne-+owners entry
namespace-created = I-namespace yenziwe
namespace-already-exists = I-namespace ikhona kakade
namespace-name-reserved = Igama le-namespace ligcinelwe
namespace-create-denied = Ukwenza i-namespace: ukufikelela kwenqiwe
namespace-create-usage = Ukusetyenziswa: :create <igama>
