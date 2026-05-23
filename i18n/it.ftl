# ma-runtime – Italiano
lang-name = Italiano

own-did-published = Documento DID proprio pubblicato su IPNS
own-did-publish-failed = Pubblicazione del documento DID proprio fallita
own-did-publish-timeout = Pubblicazione del documento DID proprio scaduta dopo 2 minuti
started = ma runtime avviato
shutdown-requested = Spegnimento richiesto
closing-endpoint = Chiusura dell'endpoint iroh...
shutdown-complete = Spegnimento completato
status-listening = Server di stato in ascolto
rpc-message-received = Messaggio RPC ricevuto
rpc-message-rejected = Messaggio RPC rifiutato
ipfs-message-rejected = Messaggio IPFS rifiutato
ctrlc-handler-failed = Handler Ctrl-C fallito
node-connected = Nodo connesso al protocollo
received-encrypted-ma-msg = Messaggio ma cifrato ricevuto su /ma/ipfs/0.0.1
unknown-rpc-atom = Atomo RPC sconosciuto, ignorato
rpc-reply-sent = Risposta RPC inviata
ping-received = :ping ricevuto, invio :pong
did-publish-request-received = Richiesta di pubblicazione documento DID ricevuta
document-published = Documento pubblicato
did-publish-cid-reply-sent = Risposta CID inviata per la pubblicazione DID
did-publish-resolve-failed = Impossibile risolvere il mittente per consegnare la risposta ipfs-publish
ipfs-store-request-received = Richiesta di archiviazione IPFS ricevuta
ipfs-stored = Contenuto archiviato su IPFS
ipfs-store-cid-reply-sent = Risposta CID inviata
ipfs-store-resolve-failed = Impossibile risolvere il mittente per consegnare la risposta ipfs-store

# Dispatch delle entità
bootstrap-complete = Bootstrap completato
entity-loaded = Plugin entità caricato
entity-load-failed = Caricamento del plugin entità fallito
entity-not-found = Entità non trovata, RPC ignorato
entity-dispatched = RPC inviato all'entità
entity-replied = L'entità ha inviato la risposta RPC
root-create-entity = #root: crea entità
root-list-entities = #root: elenca entità
root-delete-entity = #root: elimina entità
root-entity-updated = Manifesto runtime aggiornato
entity-created = Entità creata
entity-deleted = Entità eliminata
entity-states-saving = Salvataggio degli stati delle entità su IPFS
entity-state-saving = Salvataggio dello stato dell'entità
entity-state-saved = Stato dell'entità salvato
entity-state-empty = Il plugin ha restituito uno stato vuoto, salvataggio ignorato
entity-states-saved = Stati delle entità salvati
link-set = Collegamento impostato
ftl-loaded = Messaggi lingua caricati da IPFS

# Primo avvio / auto-init
no-config-found = Nessuna configurazione trovata.
initialising-new-identity = Inizializzazione di una nuova identità runtime.
generated-headless-config = Configurazione headless generata.

# Proprietà
runtime-claimed = Runtime registrato.

# Elementi radice protetti
refuse-delete-root = Mi rifiuto categoricamente di eliminare un elemento radice richiesto
no-root-acl = Nessuna ACL radice configurata — il runtime opera senza controllo degli accessi
namespace-not-found = Namespace non trovato
no-ns-gate-acl = Nessuna ACL di gate configurata per questo namespace
runtime-claim-persisted = Proprietario scritto nella configurazione.
runtime-already-claimed = Runtime già registrato.


# Namespace creation (:create)
acl-missing-owners = L'ACL radice deve contenere una voce +owners
namespace-created = Spazio dei nomi creato
namespace-already-exists = Lo spazio dei nomi esiste già
namespace-name-reserved = Nome dello spazio dei nomi riservato
namespace-create-denied = Creazione spazio dei nomi: accesso negato
namespace-create-usage = Utilizzo: :create <nome>
