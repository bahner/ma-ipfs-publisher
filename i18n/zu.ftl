# ma-runtime – isiZulu
lang-name = isiZulu

own-did-published = Idokhumenti yami ye-DID ishicilelwe ku-IPNS
own-did-publish-failed = Ukushicilela idokhumenti ye-DID yami kwehlulekile
own-did-publish-timeout = Ukushicilela idokhumenti ye-DID yami kwaphela isikhathi ngemizuzu emi-2
started = I-ma runtime iqalile
shutdown-requested = Icelwe ukuvala
closing-endpoint = Iyavala i-iroh endpoint...
shutdown-complete = Ukuvala kuphelile
status-listening = Iseva yesimo iyasilalela
rpc-message-received = Umlayezo we-RPC wamukelwe
rpc-message-rejected = Umlayezo we-RPC wenqatshiwe
ipfs-message-rejected = Umlayezo we-IPFS wenqatshiwe
ctrlc-handler-failed = Umphathi we-Ctrl-C wehlulekile
node-connected = Inode ixhume ku-protocol
received-encrypted-ma-msg = Umlayezo we-ma obethelwe wamukelwe ku-/ma/ipfs/0.0.1
unknown-rpc-atom = I-atom ye-RPC engaziwa, iyashiywa
rpc-not-text-atom = Umthwalo we-RPC awulona igama lombhalo
rpc-unknown-verb = Igama le-RPC elingaziwa
rpc-reply-sent = Impendulo ye-RPC ithunyelwe
ping-received = I-:ping yamukelwe, ithunyelwa i-:pong
did-publish-request-received = Isicelo sokushicilela idokhumenti ye-DID samukelwe
document-published = Idokhumenti ishicilelwe
did-publish-cid-reply-sent = Impendulo ye-CID yokushicilela i-DID ithunyelwe
did-publish-resolve-failed = Yehlulekile ukuxazulula umthumeli ukuze kunikezwe impendulo ye-ipfs-publish
ipfs-store-request-received = Isicelo sokulondoloza i-IPFS samukelwe
ipfs-stored = Okuqukethwe kulondoloziwe ku-IPFS
ipfs-store-cid-reply-sent = Impendulo ye-CID ithunyelwe
ipfs-store-resolve-failed = Yehlulekile ukuxazulula umthumeli ukuze kunikezwe impendulo ye-ipfs-store

# Ukuthumela i-entity
bootstrap-complete = I-Bootstrap iphelile
entity-loaded = I-plugin ye-entity ilayishiwe
entity-load-failed = Ukulayisha i-plugin ye-entity kwehlulekile
entity-not-found = I-entity ayitholakali, i-RPC ishiywa
entity-dispatched = I-RPC ithunyelwe kwi-entity
entity-replied = I-entity ithume impendulo ye-RPC
root-create-entity = #root: dala i-entity
root-list-entities = #root: uhlu lwe-entity
root-delete-entity = #root: susa i-entity
root-entity-updated = I-manifest ye-runtime ibuyekeziwe
entity-created = I-entity idalwe
entity-deleted = I-entity isuswe
entity-states-saving = Ilondoloza izimo ze-entity ku-IPFS
entity-state-saving = Ilondoloza isimo se-entity
entity-state-saved = Isimo se-entity silondoloziwe
entity-state-empty = I-plugin ibuyise isimo esingenalutho, ukulondoloza kushiyiwe
entity-states-saved = Izimo ze-entity zilondoloziwe
link-set = Isixhumi sibekiwe
ftl-loaded = Imilayezo yolimi ilayishwe ku-IPFS

# Ukuqala okuqala / auto-init
no-config-found = Akutholakali kusetha.
initialising-new-identity = Iqalisa i-identity entsha ye-runtime.
generated-headless-config = Isisethi se-headless sidalwe.

# Ubumnini
runtime-claimed = I-runtime ibhalisiwe.

# Izinto zengcambu ezivikelekile
refuse-delete-root = Ngenqaba ngokuqinile ukususa into yengcambu efunekayo
no-root-acl = I-ACL yengcambu ayilungistiwe — i-runtime iyasebenza ngaphandle kokulawula ukungena
acl-owners-access = Umshayeli unikezwe ukufinyelela njengelungu le-+owners
namespace-not-found = I-namespace ayitholakali
no-ns-gate-acl = I-ACL yesango ayilungistiwe kulo i-namespace
runtime-claim-persisted = Umnikazi ubhaliwe ezisethingeni.
runtime-already-claimed = I-runtime isibhalisiwe.


# Namespace creation (:create)
namespace-created = I-namespace yenziwe
namespace-already-exists = I-namespace ikhona kakade
namespace-name-reserved = Igama le-namespace ligciniwe
namespace-create-denied = Ukwenza i-namespace: ukufikelela kwenqiwe
namespace-create-usage = Ukusetyenziswa: :create <igama>
crud-message-received = Umyalezo we-CRUD utholakele
crud-acl-updated = I-ACL yokuthuthelwa kwe-root ibuyekeziwe

# CRUD validation errors
blob-value-ipfs-path = inani le-blob kufanele libe indlela ye-IPFS (/ipfs/, /ipns/, noma /ipld/)
acl-value-ipfs-path = inani le-ACL kufanele libe indlela ye-IPFS (/ipfs/, /ipns/, noma /ipld/)
kind-value-ipfs-path = inani le-kind kufanele libe indlela ye-IPFS (/ipfs/, /ipns/, noma /ipld/)
kind-not-found = Uhlobo alutholakali
cidv1-required = inani kufanele libe i-CIDv1 nje ('b' iqala ngayo; CIDv0 'Qm…' ayamukeleki)
config-key-protected = ukhiye we-config '%key%' ukhuselelwe
config-key-no-delete = ukhiye we-config we-daemon '%key%' awukwazi ukususwa
config-key-not-manifest = ukhiye we-config '%key%' akuyona inqolobane ye-manifest config eyaziwa
wrong-crud-protocol = iphrothokholi ye-CRUD engalungile: %type%
entity-name-invalid = igama le-entity kufanele libe yi-UTF-8 elingashicilelwa
reserved-entity-name = igama le-entity '%name%' ligcinelwe
