# ma-runtime – தமிழ்
lang-name = தமிழ்

own-did-published = சொந்த DID ஆவணம் IPNS-ல் வெளியிடப்பட்டது
own-did-publish-failed = சொந்த DID ஆவணத்தை வெளியிட முடியவில்லை
own-did-publish-timeout = சொந்த DID ஆவண வெளியீடு 2 நிமிடங்களுக்குப் பிறகு காலாவதியானது
started = ma runtime தொடங்கியது
shutdown-requested = நிறுத்தம் கோரப்பட்டது
closing-endpoint = iroh endpoint மூடப்படுகிறது...
shutdown-complete = நிறுத்தம் நிறைவுற்றது
status-listening = நிலை சேவையகம் கேட்கிறது
rpc-message-received = RPC செய்தி பெறப்பட்டது
rpc-message-rejected = RPC செய்தி நிராகரிக்கப்பட்டது
ipfs-message-rejected = IPFS செய்தி நிராகரிக்கப்பட்டது
ctrlc-handler-failed = Ctrl-C கையாளுனர் தோல்வியுற்றது
node-connected = முனை நெறிமுறையுடன் இணைந்தது
received-encrypted-ma-msg = /ma/ipfs/0.0.1-ல் மறைகுறியாக்கப்பட்ட ma செய்தி பெறப்பட்டது
unknown-rpc-atom = தெரியாத RPC அணு, புறக்கணிக்கிறது
rpc-not-text-atom = RPC தரவு உரை அணு அல்ல
rpc-unknown-verb = தெரியாத RPC வினை
rpc-reply-sent = RPC பதில் அனுப்பப்பட்டது
ping-received = :ping பெறப்பட்டது, :pong அனுப்புகிறது
did-publish-request-received = DID ஆவண வெளியீட்டு கோரிக்கை பெறப்பட்டது
document-published = ஆவணம் வெளியிடப்பட்டது
did-publish-cid-reply-sent = DID வெளியீட்டிற்கான CID பதில் அனுப்பப்பட்டது
did-publish-resolve-failed = ipfs-publish பதில் அனுப்புவதற்கு அனுப்புனரை தீர்க்க முடியவில்லை
ipfs-store-request-received = IPFS சேமிப்புக் கோரிக்கை பெறப்பட்டது
ipfs-stored = உள்ளடக்கம் IPFS-ல் சேமிக்கப்பட்டது
ipfs-store-cid-reply-sent = CID பதில் அனுப்பப்பட்டது
ipfs-store-resolve-failed = ipfs-store பதில் அனுப்புவதற்கு அனுப்புனரை தீர்க்க முடியவில்லை

# நிறுவனம் அனுப்புதல்
bootstrap-complete = Bootstrap நிறைவுற்றது
entity-loaded = நிறுவன செருகுநிரல் ஏற்றப்பட்டது
entity-load-failed = நிறுவன செருகுநிரலை ஏற்ற முடியவில்லை
entity-not-found = நிறுவனம் கிடைக்கவில்லை, RPC புறக்கணிக்கப்படுகிறது
entity-dispatched = RPC நிறுவனத்திற்கு அனுப்பப்பட்டது
entity-replied = நிறுவனம் RPC பதில் அனுப்பியது
root-create-entity = #root: நிறுவனம் உருவாக்கு
root-list-entities = #root: நிறுவன பட்டியல்
root-delete-entity = #root: நிறுவனம் நீக்கு
root-entity-updated = Runtime manifest புதுப்பிக்கப்பட்டது
entity-created = நிறுவனம் உருவாக்கப்பட்டது
entity-deleted = நிறுவனம் நீக்கப்பட்டது
entity-states-saving = IPFS-ல் நிறுவன நிலைகள் சேமிக்கப்படுகின்றன
entity-state-saving = நிறுவன நிலை சேமிக்கப்படுகிறது
entity-state-saved = நிறுவன நிலை சேமிக்கப்பட்டது
entity-state-empty = செருகுநிரல் காலி நிலையை திருப்பியது, சேமிப்பு தவிர்க்கப்பட்டது
entity-states-saved = நிறுவன நிலைகள் சேமிக்கப்பட்டன
link-set = இணைப்பு அமைக்கப்பட்டது
ftl-loaded = IPFS-லிருந்து மொழி செய்திகள் ஏற்றப்பட்டன

# முதல் துவக்கம் / தானியங்கி தொடக்கம்
no-config-found = உள்ளமைவு கிடைக்கவில்லை.
initialising-new-identity = புதிய runtime அடையாளத்தை தொடங்குகிறது.
generated-headless-config = தலையற்ற உள்ளமைவு உருவாக்கப்பட்டது.

# உரிமை
runtime-claimed = Runtime பதிவு செய்யப்பட்டது.

# பாதுகாக்கப்பட்ட root உறுப்புகள்
refuse-delete-root = தேவையான root உறுப்பை நீக்குவதை உறுதியாக மறுக்கிறோம்
no-root-acl = Root ACL உள்ளமைக்கப்படவில்லை — runtime அணுகல் கட்டுப்பாடு இல்லாமல் இயங்குகிறது
namespace-not-found = namespace கிடைக்கவில்லை
no-ns-gate-acl = இந்த namespace-க்கு gate ACL உள்ளமைக்கப்படவில்லை
runtime-claim-persisted = உரிமையாளர் உள்ளமைவில் எழுதப்பட்டார்.
runtime-already-claimed = Runtime ஏற்கனவே பதிவு செய்யப்பட்டது.


# Namespace creation (:create)
acl-missing-owners = ரூட் ACL-ல் +owners உள்ளீடு இருக்க வேண்டும்
namespace-created = பெயர்வெளி உருவாக்கப்பட்டது
namespace-already-exists = பெயர்வெளி ஏற்கனவே உள்ளது
namespace-name-reserved = பெயர்வெளியின் பெயர் ஒதுக்கப்பட்டது
namespace-create-denied = பெயர்வெளி உருவாக்கம்: அணுகல் மறுக்கப்பட்டது
namespace-create-usage = பயன்பாடு: :create <பெயர்>
crud-message-received = CRUD செய்தி பெறப்பட்டது
crud-acl-updated = மூல போக்குவரத்து ACL புதுப்பிக்கப்பட்டது
