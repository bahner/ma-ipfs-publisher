# ma-runtime – Ελληνικά
lang-name = Ελληνικά

own-did-published = Το δικό μου έγγραφο DID δημοσιεύτηκε στο IPNS
own-did-publish-failed = Αποτυχία δημοσίευσης του δικού μου εγγράφου DID
own-did-publish-timeout = Η δημοσίευση του δικού μου εγγράφου DID έληξε μετά από 2 λεπτά
started = Το ma runtime εκκινήθηκε
shutdown-requested = Ζητήθηκε τερματισμός
closing-endpoint = Κλείσιμο του iroh τελικού σημείου...
shutdown-complete = Ο τερματισμός ολοκληρώθηκε
status-listening = Ο διακομιστής κατάστασης ακούει
rpc-message-received = Ελήφθη μήνυμα RPC
rpc-message-rejected = Το μήνυμα RPC απορρίφθηκε
ipfs-message-rejected = Το μήνυμα IPFS απορρίφθηκε
ctrlc-handler-failed = Αποτυχία χειριστή Ctrl-C
node-connected = Ο κόμβος συνδέθηκε στο πρωτόκολλο
received-encrypted-ma-msg = Ελήφθη κρυπτογραφημένο μήνυμα ma στο /ma/ipfs/0.0.1
unknown-rpc-atom = Άγνωστο άτομο RPC, αγνόηση
rpc-reply-sent = Εστάλη απάντηση RPC
ping-received = Ελήφθη :ping, αποστολή :pong
did-publish-request-received = Ελήφθη αίτηση δημοσίευσης εγγράφου DID
document-published = Το έγγραφο δημοσιεύτηκε
did-publish-cid-reply-sent = Εστάλη απάντηση CID για δημοσίευση DID
did-publish-resolve-failed = Αποτυχία επίλυσης αποστολέα για παράδοση απάντησης ipfs-publish
ipfs-store-request-received = Ελήφθη αίτηση αποθήκευσης IPFS
ipfs-stored = Το περιεχόμενο αποθηκεύτηκε στο IPFS
ipfs-store-cid-reply-sent = Εστάλη απάντηση CID
ipfs-store-resolve-failed = Αποτυχία επίλυσης αποστολέα για παράδοση απάντησης ipfs-store

# Αποστολή οντοτήτων
bootstrap-complete = Το Bootstrap ολοκληρώθηκε
entity-loaded = Φορτώθηκε το πρόσθετο οντότητας
entity-load-failed = Αποτυχία φόρτωσης προσθέτου οντότητας
entity-not-found = Η οντότητα δεν βρέθηκε, αγνόηση RPC
entity-dispatched = Το RPC αποστάλη στην οντότητα
entity-replied = Η οντότητα απάντησε με RPC
root-create-entity = #root: δημιουργία οντότητας
root-list-entities = #root: λίστα οντοτήτων
root-delete-entity = #root: διαγραφή οντότητας
root-entity-updated = Ενημερώθηκε το manifest runtime
entity-created = Δημιουργήθηκε οντότητα
entity-deleted = Διαγράφηκε οντότητα
entity-states-saving = Αποθήκευση καταστάσεων οντοτήτων στο IPFS
entity-state-saving = Αποθήκευση κατάστασης οντότητας
entity-state-saved = Αποθηκεύτηκε η κατάσταση οντότητας
entity-state-empty = Το πρόσθετο επέστρεψε κενή κατάσταση, παράλειψη αποθήκευσης
entity-states-saved = Αποθηκεύτηκαν καταστάσεις οντοτήτων
link-set = Ορίστηκε σύνδεσμος
ftl-loaded = Φορτώθηκαν μηνύματα γλώσσας από το IPFS

# Πρώτη εκκίνηση / αυτόματη αρχικοποίηση
no-config-found = Δεν βρέθηκε διαμόρφωση.
initialising-new-identity = Αρχικοποίηση νέας ταυτότητας runtime.
generated-headless-config = Δημιουργήθηκε διαμόρφωση χωρίς κεφαλή.

# Κυριότητα
runtime-claimed = Το runtime καταχωρήθηκε.

# Προστατευμένα στοιχεία ρίζας
refuse-delete-root = Αρνούμαι κατηγορηματικά να διαγράψω το απαιτούμενο στοιχείο ρίζας
no-root-acl = Δεν έχει ρυθμιστεί ACL ρίζας — το runtime λειτουργεί χωρίς έλεγχο πρόσβασης
namespace-not-found = Δεν βρέθηκε ο χώρος ονομάτων
no-ns-gate-acl = Δεν έχει ρυθμιστεί ACL πύλης για αυτόν τον χώρο ονομάτων
runtime-claim-persisted = Ο ιδιοκτήτης γράφτηκε στη διαμόρφωση.
runtime-already-claimed = Το runtime έχει ήδη καταχωρηθεί.


# Namespace creation (:create)
acl-missing-owners = Το ACL ρίζας πρέπει να περιέχει καταχώρηση +owners
namespace-created = Ο χώρος ονομάτων δημιουργήθηκε
namespace-already-exists = Ο χώρος ονομάτων υπάρχει ήδη
namespace-name-reserved = Το όνομα χώρου ονομάτων είναι δεσμευμένο
namespace-create-denied = Δημιουργία χώρου ονομάτων: η πρόσβαση απορρίφθηκε
namespace-create-usage = Χρήση: :create <όνομα>
