# ma-runtime – हिन्दी
lang-name = हिन्दी

own-did-published = स्वयं का DID दस्तावेज़ IPNS पर प्रकाशित किया गया
own-did-publish-failed = स्वयं का DID दस्तावेज़ प्रकाशित करने में विफल
own-did-publish-timeout = स्वयं के DID दस्तावेज़ का प्रकाशन 2 मिनट बाद टाइम आउट हुआ
started = ma runtime शुरू हुआ
shutdown-requested = बंद करने का अनुरोध किया गया
closing-endpoint = iroh एंडपॉइंट बंद किया जा रहा है...
shutdown-complete = बंद करना पूर्ण हुआ
status-listening = स्टेटस सर्वर सुन रहा है
rpc-message-received = RPC संदेश प्राप्त हुआ
rpc-message-rejected = RPC संदेश अस्वीकार किया गया
ipfs-message-rejected = IPFS संदेश अस्वीकार किया गया
ctrlc-handler-failed = Ctrl-C हैंडलर विफल रहा
node-connected = नोड प्रोटोकॉल से जुड़ा
received-encrypted-ma-msg = /ma/ipfs/0.0.1 पर एन्क्रिप्टेड ma संदेश प्राप्त हुआ
unknown-rpc-atom = अज्ञात RPC एटम, अनदेखा करें
rpc-not-text-atom = RPC सामग्री एक पाठ परमाणु नहीं है
rpc-unknown-verb = अज्ञात RPC क्रिया
rpc-reply-sent = RPC उत्तर भेजा गया
ping-received = :ping प्राप्त हुआ, :pong भेजा जा रहा है
did-publish-request-received = DID दस्तावेज़ प्रकाशन अनुरोध प्राप्त हुआ
document-published = दस्तावेज़ प्रकाशित किया गया
did-publish-cid-reply-sent = DID प्रकाशन के लिए CID उत्तर भेजा गया
did-publish-resolve-failed = ipfs-publish उत्तर देने के लिए प्रेषक को हल करने में विफल
ipfs-store-request-received = IPFS भंडारण अनुरोध प्राप्त हुआ
ipfs-stored = सामग्री IPFS पर संग्रहीत की गई
ipfs-store-cid-reply-sent = CID उत्तर भेजा गया
ipfs-store-resolve-failed = ipfs-store उत्तर देने के लिए प्रेषक को हल करने में विफल

# एंटिटी डिस्पैच
bootstrap-complete = Bootstrap पूर्ण हुआ
entity-loaded = एंटिटी प्लगइन लोड किया गया
entity-load-failed = एंटिटी प्लगइन लोड करने में विफल
entity-not-found = एंटिटी नहीं मिली, RPC अनदेखा करें
entity-dispatched = RPC एंटिटी को भेजा गया
entity-replied = एंटिटी ने RPC उत्तर भेजा
root-create-entity = #root: एंटिटी बनाएं
root-list-entities = #root: एंटिटी सूची
root-delete-entity = #root: एंटिटी हटाएं
root-entity-updated = Runtime मेनिफ़ेस्ट अपडेट हुआ
entity-created = एंटिटी बनाई गई
entity-deleted = एंटिटी हटाई गई
entity-states-saving = एंटिटी स्थितियां IPFS पर सहेजी जा रही हैं
entity-state-saving = एंटिटी स्थिति सहेजी जा रही है
entity-state-saved = एंटिटी स्थिति सहेजी गई
entity-state-empty = प्लगइन ने खाली स्थिति लौटाई, सहेजना छोड़ा गया
entity-states-saved = एंटिटी स्थितियां सहेजी गईं
link-set = लिंक सेट किया गया
ftl-loaded = IPFS से भाषा संदेश लोड किए गए

# पहला प्रारंभ / स्वतः-प्रारंभीकरण
no-config-found = कॉन्फ़िगरेशन नहीं मिला।
initialising-new-identity = नई runtime पहचान प्रारंभ की जा रही है।
generated-headless-config = हेडलेस कॉन्फ़िगरेशन उत्पन्न किया गया।

# स्वामित्व
runtime-claimed = Runtime पंजीकृत हुआ।

# संरक्षित रूट तत्व
refuse-delete-root = आवश्यक रूट तत्व को हटाने से दृढ़तापूर्वक इनकार
no-root-acl = रूट ACL कॉन्फ़िगर नहीं है — runtime बिना एक्सेस नियंत्रण के चल रहा है
namespace-not-found = नेमस्पेस नहीं मिला
no-ns-gate-acl = इस नेमस्पेस के लिए गेट ACL कॉन्फ़िगर नहीं है
runtime-claim-persisted = स्वामी कॉन्फ़िगरेशन में लिखा गया।
runtime-already-claimed = Runtime पहले से पंजीकृत है।


# Namespace creation (:create)
acl-missing-owners = रूट ACL में +owners प्रविष्टि होनी चाहिए
namespace-created = नेमस्पेस बनाया गया
namespace-already-exists = नेमस्पेस पहले से मौजूद है
namespace-name-reserved = नेमस्पेस का नाम आरक्षित है
namespace-create-denied = नेमस्पेस बनाना: पहुंच अस्वीकृत
namespace-create-usage = उपयोग: :create <नाम>
crud-message-received = CRUD संदेश प्राप्त
crud-acl-updated = रूट ट्रान्सपोर्ट ACL अपडेट हुआ
