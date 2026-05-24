# ma-runtime – বাংলা
lang-name = বাংলা

own-did-published = নিজের DID দলিল IPNS-এ প্রকাশিত হয়েছে
own-did-publish-failed = নিজের DID দলিল প্রকাশ করতে ব্যর্থ
own-did-publish-timeout = নিজের DID দলিল প্রকাশ ২ মিনিট পরে টাইমআউট হয়েছে
started = ma runtime শুরু হয়েছে
shutdown-requested = বন্ধ করার অনুরোধ করা হয়েছে
closing-endpoint = iroh এন্ডপয়েন্ট বন্ধ হচ্ছে...
shutdown-complete = বন্ধ করা সম্পন্ন হয়েছে
status-listening = স্ট্যাটাস সার্ভার শুনছে
rpc-message-received = RPC বার্তা প্রাপ্ত হয়েছে
rpc-message-rejected = RPC বার্তা প্রত্যাখ্যাত হয়েছে
ipfs-message-rejected = IPFS বার্তা প্রত্যাখ্যাত হয়েছে
ctrlc-handler-failed = Ctrl-C হ্যান্ডলার ব্যর্থ হয়েছে
node-connected = নোড প্রোটোকলে সংযুক্ত হয়েছে
received-encrypted-ma-msg = /ma/ipfs/0.0.1-এ এনক্রিপ্টেড ma বার্তা প্রাপ্ত হয়েছে
unknown-rpc-atom = অজানা RPC অ্যাটম, উপেক্ষা করুন
rpc-not-text-atom = RPC পেলোড একটি টেক্সট অ্যাটম নয়
rpc-unknown-verb = অজানা RPC ক্রিয়া
rpc-reply-sent = RPC উত্তর পাঠানো হয়েছে
ping-received = :ping প্রাপ্ত হয়েছে, :pong পাঠানো হচ্ছে
did-publish-request-received = DID দলিল প্রকাশের অনুরোধ প্রাপ্ত হয়েছে
document-published = দলিল প্রকাশিত হয়েছে
did-publish-cid-reply-sent = DID প্রকাশের জন্য CID উত্তর পাঠানো হয়েছে
did-publish-resolve-failed = ipfs-publish উত্তর দিতে প্রেরক সমাধান করতে ব্যর্থ
ipfs-store-request-received = IPFS স্টোরেজ অনুরোধ প্রাপ্ত হয়েছে
ipfs-stored = কন্টেন্ট IPFS-এ সংরক্ষিত হয়েছে
ipfs-store-cid-reply-sent = CID উত্তর পাঠানো হয়েছে
ipfs-store-resolve-failed = ipfs-store উত্তর দিতে প্রেরক সমাধান করতে ব্যর্থ

# এন্টিটি ডিসপ্যাচ
bootstrap-complete = Bootstrap সম্পন্ন হয়েছে
entity-loaded = এন্টিটি প্লাগইন লোড হয়েছে
entity-load-failed = এন্টিটি প্লাগইন লোড করতে ব্যর্থ
entity-not-found = এন্টিটি পাওয়া যায়নি, RPC উপেক্ষা করুন
entity-dispatched = RPC এন্টিটিতে পাঠানো হয়েছে
entity-replied = এন্টিটি RPC উত্তর পাঠিয়েছে
root-create-entity = #root: এন্টিটি তৈরি করুন
root-list-entities = #root: এন্টিটি তালিকা
root-delete-entity = #root: এন্টিটি মুছুন
root-entity-updated = Runtime ম্যানিফেস্ট আপডেট হয়েছে
entity-created = এন্টিটি তৈরি হয়েছে
entity-deleted = এন্টিটি মুছে গেছে
entity-states-saving = IPFS-এ এন্টিটি অবস্থা সংরক্ষণ হচ্ছে
entity-state-saving = এন্টিটি অবস্থা সংরক্ষণ হচ্ছে
entity-state-saved = এন্টিটি অবস্থা সংরক্ষিত হয়েছে
entity-state-empty = প্লাগইন খালি অবস্থা ফেরত দিয়েছে, সংরক্ষণ এড়িয়ে গেছে
entity-states-saved = এন্টিটি অবস্থাসমূহ সংরক্ষিত হয়েছে
link-set = লিঙ্ক সেট হয়েছে
ftl-loaded = IPFS থেকে ভাষার বার্তা লোড হয়েছে

# প্রথম স্টার্টআপ / স্বয়ংক্রিয়-ইনিট
no-config-found = কনফিগারেশন পাওয়া যায়নি।
initialising-new-identity = নতুন runtime পরিচয় শুরু হচ্ছে।
generated-headless-config = হেডলেস কনফিগারেশন তৈরি হয়েছে।

# মালিকানা
runtime-claimed = Runtime নিবন্ধিত হয়েছে।

# সুরক্ষিত রুট উপাদান
refuse-delete-root = প্রয়োজনীয় রুট উপাদান মুছতে দৃঢ়ভাবে অস্বীকার করুন
no-root-acl = রুট ACL কনফিগার করা নেই — runtime অ্যাক্সেস নিয়ন্ত্রণ ছাড়া চলছে
namespace-not-found = নেমস্পেস পাওয়া যায়নি
no-ns-gate-acl = এই নেমস্পেসের জন্য গেট ACL কনফিগার করা নেই
runtime-claim-persisted = মালিক কনফিগারেশনে লেখা হয়েছে।
runtime-already-claimed = Runtime ইতিমধ্যে নিবন্ধিত।


# Namespace creation (:create)
acl-missing-owners = রুট ACL-এ +owners এন্ট্রি থাকতে হবে
namespace-created = নেমস্পেস তৈরি হয়েছে
namespace-already-exists = নেমস্পেস ইতিমধ্যে বিদ্যমান
namespace-name-reserved = নেমস্পেসের নাম সংরক্ষিত
namespace-create-denied = নেমস্পেস তৈরি: অ্যাক্সেস অস্বীকৃত
namespace-create-usage = ব্যবহার: :create <নাম>
crud-message-received = CRUD বার্তা পাওয়া গেছে
crud-acl-updated = রুট ট্রান্সপোর্ট ACL আপডেট হয়েছে
