# ma-runtime – Polski
lang-name = Polski

own-did-published = Własny dokument DID opublikowany w IPNS
own-did-publish-failed = Nie udało się opublikować własnego dokumentu DID
own-did-publish-timeout = Publikacja własnego dokumentu DID przekroczyła limit czasu po 2 minutach
started = ma runtime uruchomiony
shutdown-requested = Żądanie wyłączenia
closing-endpoint = Zamykanie punktu końcowego iroh...
shutdown-complete = Wyłączenie zakończone
status-listening = Serwer statusu nasłuchuje
rpc-message-received = Odebrano wiadomość RPC
rpc-message-rejected = Wiadomość RPC odrzucona
ipfs-message-rejected = Wiadomość IPFS odrzucona
ctrlc-handler-failed = Błąd procedury obsługi Ctrl-C
node-connected = Węzeł podłączony do protokołu
received-encrypted-ma-msg = Odebrano zaszyfrowaną wiadomość ma na /ma/ipfs/0.0.1
unknown-rpc-atom = Nieznany atom RPC, ignorowanie
rpc-not-text-atom = Ładunek RPC nie jest atomem tekstowym
rpc-unknown-verb = Nieznany czasownik RPC
rpc-reply-sent = Odpowiedź RPC wysłana
ping-received = Odebrano :ping, wysyłam :pong
did-publish-request-received = Odebrano żądanie publikacji dokumentu DID
document-published = Dokument opublikowany
did-publish-cid-reply-sent = Wysłano odpowiedź CID dla publikacji DID
did-publish-resolve-failed = Nie można rozwiązać nadawcy w celu dostarczenia odpowiedzi ipfs-publish
ipfs-store-request-received = Odebrano żądanie przechowywania IPFS
ipfs-stored = Treść zapisana w IPFS
ipfs-store-cid-reply-sent = Odpowiedź CID wysłana
ipfs-store-resolve-failed = Nie można rozwiązać nadawcy w celu dostarczenia odpowiedzi ipfs-store

# Przekazywanie encji
bootstrap-complete = Bootstrap zakończony
entity-loaded = Wtyczka encji załadowana
entity-load-failed = Nie udało się załadować wtyczki encji
entity-not-found = Encja nie znaleziona, ignorowanie RPC
entity-dispatched = RPC przekazane do encji
entity-replied = Encja wysłała odpowiedź RPC
root-create-entity = #root: utwórz encję
root-list-entities = #root: lista encji
root-delete-entity = #root: usuń encję
root-entity-updated = Manifest runtime zaktualizowany
entity-created = Encja utworzona
entity-deleted = Encja usunięta
entity-states-saving = Zapisywanie stanów encji do IPFS
entity-state-saving = Zapisywanie stanu encji
entity-state-saved = Stan encji zapisany
entity-state-empty = Wtyczka zwróciła pusty stan, pomijanie zapisu
entity-states-saved = Stany encji zapisane
link-set = Łącze ustawione
ftl-loaded = Komunikaty językowe załadowane z IPFS

# Pierwsze uruchomienie / auto-init
no-config-found = Nie znaleziono konfiguracji.
initialising-new-identity = Inicjalizacja nowej tożsamości runtime.
generated-headless-config = Wygenerowano konfigurację headless.

# Własność
runtime-claimed = Runtime zarejestrowany.

# Chronione elementy główne
refuse-delete-root = Stanowczo odmawiam usunięcia wymaganego elementu głównego
no-root-acl = Brak skonfigurowanego ACL głównego — runtime działa bez kontroli dostępu
namespace-not-found = Przestrzeń nazw nie znaleziona
no-ns-gate-acl = Brak skonfigurowanego ACL bramy dla tej przestrzeni nazw
runtime-claim-persisted = Właściciel zapisany w konfiguracji.
runtime-already-claimed = Runtime już zarejestrowany.


# Namespace creation (:create)
acl-missing-owners = Główny ACL musi zawierać wpis +owners
namespace-created = Przestrzeń nazw utworzona
namespace-already-exists = Przestrzeń nazw już istnieje
namespace-name-reserved = Nazwa przestrzeni nazw jest zarezerwowana
namespace-create-denied = Tworzenie przestrzeni nazw: odmowa dostępu
namespace-create-usage = Użycie: :create <nazwa>
crud-message-received = Odebrano wiadomość CRUD
crud-acl-updated = Zaktualizowano główny ACL transportu
