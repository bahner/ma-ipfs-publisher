# ma-runtime – Français
lang-name = Français

own-did-published = Document DID propre publié sur IPNS
own-did-publish-failed = Échec de la publication du document DID propre
own-did-publish-timeout = Publication du document DID propre expirée après 2 minutes
started = ma runtime démarré
shutdown-requested = Arrêt demandé
closing-endpoint = Fermeture du point de terminaison iroh...
shutdown-complete = Arrêt terminé
status-listening = Serveur de statut en écoute
rpc-message-received = Message RPC reçu
rpc-message-rejected = Message RPC rejeté
ipfs-message-rejected = Message IPFS rejeté
ctrlc-handler-failed = Échec du gestionnaire Ctrl-C
node-connected = Nœud connecté au protocole
received-encrypted-ma-msg = Message ma chiffré reçu sur /ma/ipfs/0.0.1
unknown-rpc-atom = Atome RPC inconnu, ignoré
rpc-not-text-atom = La charge RPC n'est pas un atome texte
rpc-unknown-verb = Verbe RPC inconnu
rpc-reply-sent = Réponse RPC envoyée
ping-received = :ping reçu, envoi de :pong
did-publish-request-received = Demande de publication de document DID reçue
document-published = Document publié
did-publish-cid-reply-sent = Réponse CID envoyée pour la publication DID
did-publish-resolve-failed = Impossible de résoudre l'expéditeur pour livrer la réponse ipfs-publish
ipfs-store-request-received = Demande de stockage IPFS reçue
ipfs-stored = Contenu stocké sur IPFS
ipfs-store-cid-reply-sent = Réponse CID envoyée
ipfs-store-resolve-failed = Impossible de résoudre l'expéditeur pour livrer la réponse ipfs-store

# Distribution des entités
bootstrap-complete = Bootstrap terminé
entity-loaded = Plugin d'entité chargé
entity-load-failed = Échec du chargement du plugin d'entité
entity-not-found = Entité introuvable, RPC ignoré
entity-dispatched = RPC transmis à l'entité
entity-replied = L'entité a envoyé une réponse RPC
root-create-entity = #root : créer une entité
root-list-entities = #root : lister les entités
root-delete-entity = #root : supprimer une entité
root-entity-updated = Manifeste runtime mis à jour
entity-created = Entité créée
entity-deleted = Entité supprimée
entity-states-saving = Sauvegarde des états des entités sur IPFS
entity-state-saving = Sauvegarde de l'état de l'entité
entity-state-saved = État de l'entité sauvegardé
entity-state-empty = Le plugin a retourné un état vide, sauvegarde ignorée
entity-states-saved = États des entités sauvegardés
link-set = Lien défini
ftl-loaded = Messages de langue chargés depuis IPFS

# Premier démarrage / auto-init
no-config-found = Aucune configuration trouvée.
initialising-new-identity = Initialisation d'une nouvelle identité runtime.
generated-headless-config = Configuration headless générée.

# Propriété
runtime-claimed = Runtime enregistré.

# Éléments racine protégés
refuse-delete-root = Refuse catégoriquement de supprimer un élément racine requis
no-root-acl = Aucune ACL racine configurée — le runtime fonctionne sans contrôle d'accès
namespace-not-found = Espace de noms introuvable
no-ns-gate-acl = Aucune ACL de passerelle configurée pour cet espace de noms
runtime-claim-persisted = Propriétaire écrit dans la configuration.
runtime-already-claimed = Runtime déjà enregistré.


# Namespace creation (:create)
acl-missing-owners = L'ACL racine doit contenir une entrée +owners
namespace-created = Espace de noms créé
namespace-already-exists = L'espace de noms existe déjà
namespace-name-reserved = Nom d'espace de noms réservé
namespace-create-denied = Création d'espace de noms: accès refusé
namespace-create-usage = Utilisation: :create <nom>
crud-message-received = Message CRUD reçu
crud-acl-updated = ACL de transport racine mise à jour
