# ma-runtime – मराठी
lang-name = मराठी

own-did-published = स्वतःचा DID दस्तऐवज IPNS वर प्रकाशित झाला
own-did-publish-failed = स्वतःचा DID दस्तऐवज प्रकाशित करण्यात अयशस्वी
own-did-publish-timeout = स्वतःच्या DID दस्तऐवज प्रकाशनाचा वेळ 2 मिनिटांनंतर संपला
started = ma runtime सुरू झाले
shutdown-requested = बंद करण्याची विनंती केली
closing-endpoint = iroh एंडपॉइंट बंद होत आहे...
shutdown-complete = बंद करणे पूर्ण झाले
status-listening = स्टेटस सर्व्हर ऐकत आहे
rpc-message-received = RPC संदेश प्राप्त झाला
rpc-message-rejected = RPC संदेश नाकारला गेला
ipfs-message-rejected = IPFS संदेश नाकारला गेला
ctrlc-handler-failed = Ctrl-C हँडलर अयशस्वी झाला
node-connected = नोड प्रोटोकॉलशी जोडले
received-encrypted-ma-msg = /ma/ipfs/0.0.1 वर एन्क्रिप्टेड ma संदेश प्राप्त झाला
unknown-rpc-atom = अज्ञात RPC अणू, दुर्लक्ष करा
rpc-reply-sent = RPC उत्तर पाठवले
ping-received = :ping प्राप्त, :pong पाठवत आहे
did-publish-request-received = DID दस्तऐवज प्रकाशन विनंती प्राप्त झाली
document-published = दस्तऐवज प्रकाशित झाला
did-publish-cid-reply-sent = DID प्रकाशनासाठी CID उत्तर पाठवले
did-publish-resolve-failed = ipfs-publish उत्तर देण्यासाठी प्रेषक सोडवण्यात अयशस्वी
ipfs-store-request-received = IPFS स्टोरेज विनंती प्राप्त झाली
ipfs-stored = सामग्री IPFS वर संग्रहित केली
ipfs-store-cid-reply-sent = CID उत्तर पाठवले
ipfs-store-resolve-failed = ipfs-store उत्तर देण्यासाठी प्रेषक सोडवण्यात अयशस्वी

# एंटिटी डिस्पॅच
bootstrap-complete = Bootstrap पूर्ण झाले
entity-loaded = एंटिटी प्लगइन लोड झाला
entity-load-failed = एंटिटी प्लगइन लोड करण्यात अयशस्वी
entity-not-found = एंटिटी सापडली नाही, RPC दुर्लक्षित
entity-dispatched = RPC एंटिटीला पाठवले
entity-replied = एंटिटीने RPC उत्तर पाठवले
root-create-entity = #root: एंटिटी तयार करा
root-list-entities = #root: एंटिटी यादी
root-delete-entity = #root: एंटिटी हटवा
root-entity-updated = Runtime मेनिफेस्ट अपडेट झाले
entity-created = एंटिटी तयार झाली
entity-deleted = एंटिटी हटवली
entity-states-saving = IPFS वर एंटिटी स्थिती जतन होत आहेत
entity-state-saving = एंटिटी स्थिती जतन होत आहे
entity-state-saved = एंटिटी स्थिती जतन झाली
entity-state-empty = प्लगइनने रिकामी स्थिती परत केली, जतन वगळले
entity-states-saved = एंटिटी स्थिती जतन झाल्या
link-set = लिंक सेट केला
ftl-loaded = IPFS वरून भाषा संदेश लोड झाले

# पहली सुरुवात / स्वयं-प्रारंभ
no-config-found = कॉन्फिगरेशन सापडले नाही.
initialising-new-identity = नवीन runtime ओळख सुरू करत आहे.
generated-headless-config = हेडलेस कॉन्फिगरेशन तयार झाले.

# मालकी
runtime-claimed = Runtime नोंदवले.

# संरक्षित मूळ घटक
refuse-delete-root = आवश्यक मूळ घटक हटवण्यास ठामपणे नकार
no-root-acl = मूळ ACL कॉन्फिगर केलेले नाही — runtime प्रवेश नियंत्रणाशिवाय चालत आहे
namespace-not-found = नेमस्पेस सापडले नाही
no-ns-gate-acl = या नेमस्पेससाठी गेट ACL कॉन्फिगर नाही
runtime-claim-persisted = मालक कॉन्फिगरेशनमध्ये लिहिला.
runtime-already-claimed = Runtime आधीच नोंदवले आहे.


# Namespace creation (:create)
acl-missing-owners = मूळ ACL मध्ये +owners प्रविष्टी असणे आवश्यक आहे
namespace-created = नेमस्पेस तयार केले
namespace-already-exists = नेमस्पेस आधीच अस्तित्वात आहे
namespace-name-reserved = नेमस्पेसचे नाव राखीव आहे
namespace-create-denied = नेमस्पेस तयार करणे: प्रवेश नाकारला
namespace-create-usage = वापर: :create <नाव>
