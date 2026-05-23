# ma-runtime – Deutsch
lang-name = Deutsch

own-did-published = Eigenes DID-Dokument auf IPNS veröffentlicht
own-did-publish-failed = Veröffentlichung des eigenen DID-Dokuments fehlgeschlagen
own-did-publish-timeout = Veröffentlichung des eigenen DID-Dokuments nach 2 Minuten abgebrochen
started = ma runtime gestartet
shutdown-requested = Herunterfahren angefordert
closing-endpoint = iroh-Endpunkt wird geschlossen...
shutdown-complete = Herunterfahren abgeschlossen
status-listening = Statusserver lauscht
rpc-message-received = RPC-Nachricht empfangen
rpc-message-rejected = RPC-Nachricht abgelehnt
ipfs-message-rejected = IPFS-Nachricht abgelehnt
ctrlc-handler-failed = Ctrl-C-Handler fehlgeschlagen
node-connected = Knoten mit Protokoll verbunden
received-encrypted-ma-msg = Verschlüsselte ma-Nachricht auf /ma/ipfs/0.0.1 empfangen
unknown-rpc-atom = Unbekanntes RPC-Atom, wird ignoriert
rpc-reply-sent = RPC-Antwort gesendet
ping-received = :ping empfangen, sende :pong
did-publish-request-received = Anfrage zur Veröffentlichung des DID-Dokuments empfangen
document-published = Dokument veröffentlicht
did-publish-cid-reply-sent = CID-Antwort für DID-Veröffentlichung gesendet
did-publish-resolve-failed = Sender konnte nicht aufgelöst werden, um ipfs-publish-Antwort zu liefern
ipfs-store-request-received = IPFS-Store-Anfrage empfangen
ipfs-stored = Inhalt auf IPFS gespeichert
ipfs-store-cid-reply-sent = CID-Antwort gesendet
ipfs-store-resolve-failed = Sender konnte nicht aufgelöst werden, um ipfs-store-Antwort zu liefern

# Entitätsutsendung
bootstrap-complete = Bootstrap abgeschlossen
entity-loaded = Entitätsplugin geladen
entity-load-failed = Laden des Entitätsplugins fehlgeschlagen
entity-not-found = Entität nicht gefunden, RPC wird ignoriert
entity-dispatched = RPC an Entität weitergeleitet
entity-replied = Entität hat RPC-Antwort gesendet
root-create-entity = #root: Entität erstellen
root-list-entities = #root: Entitäten auflisten
root-delete-entity = #root: Entität löschen
root-entity-updated = Runtime-Manifest aktualisiert
entity-created = Entität erstellt
entity-deleted = Entität gelöscht
entity-states-saving = Entitätszustände werden auf IPFS gespeichert
entity-state-saving = Entitätszustand wird gespeichert
entity-state-saved = Entitätszustand gespeichert
entity-state-empty = Plugin hat leeren Zustand zurückgegeben, Speichern wird übersprungen
entity-states-saved = Entitätszustände gespeichert
link-set = Verknüpfung gesetzt
ftl-loaded = Sprachnachrichten von IPFS geladen

# Erster Start / Auto-Init
no-config-found = Keine Konfiguration gefunden.
initialising-new-identity = Neue Runtime-Identität wird initialisiert.
generated-headless-config = Headless-Konfiguration erstellt.

# Eigentumsrecht
runtime-claimed = Runtime registriert.

# Geschützte Wurzelelemente
refuse-delete-root = Weigere mich strikt, ein erforderliches Wurzelelement zu löschen
no-root-acl = Keine Root-ACL konfiguriert — Runtime läuft ohne Zugangskontrolle
namespace-not-found = Namensraum nicht gefunden
no-ns-gate-acl = Keine Gate-ACL für diesen Namensraum konfiguriert
runtime-claim-persisted = Eigentümer in Konfiguration geschrieben.
runtime-already-claimed = Runtime wurde bereits registriert.


# Namespace creation (:create)
acl-missing-owners = Das Stamm-ACL muss einen +owners-Eintrag enthalten
namespace-created = Namensraum erstellt
namespace-already-exists = Namensraum existiert bereits
namespace-name-reserved = Namensraumname ist reserviert
namespace-create-denied = Namensraum erstellen: Zugriff verweigert
namespace-create-usage = Verwendung: :create <Name>
