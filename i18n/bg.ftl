# ma-runtime – Български
lang-name = Български

own-did-published = Собственият DID документ е публикуван в IPNS
own-did-publish-failed = Неуспешна публикация на собствения DID документ
own-did-publish-timeout = Публикацията на собствения DID документ изтече след 2 минути
started = ma runtime стартиран
shutdown-requested = Заявено изключване
closing-endpoint = Затваряне на iroh крайната точка...
shutdown-complete = Изключването завършено
status-listening = Сървърът за статус слуша
rpc-message-received = Получено RPC съобщение
rpc-message-rejected = RPC съобщението е отхвърлено
ipfs-message-rejected = IPFS съобщението е отхвърлено
ctrlc-handler-failed = Неуспех на манипулатора Ctrl-C
node-connected = Възелът е свързан с протокол
received-encrypted-ma-msg = Получено криптирано ma съобщение на /ma/ipfs/0.0.1
unknown-rpc-atom = Непознат RPC атом, игнорирано
rpc-not-text-atom = RPC данните не са текстов атом
rpc-unknown-verb = Непознат RPC глагол
rpc-reply-sent = RPC отговорът е изпратен
ping-received = Получен :ping, изпращам :pong
did-publish-request-received = Получена заявка за публикуване на DID документ
document-published = Документът е публикуван
did-publish-cid-reply-sent = Изпратен CID отговор за публикуване на DID
did-publish-resolve-failed = Неуспешно разрешаване на подателя за доставяне на ipfs-publish отговор
ipfs-store-request-received = Получена заявка за съхранение в IPFS
ipfs-stored = Съдържанието е съхранено в IPFS
ipfs-store-cid-reply-sent = CID отговорът е изпратен
ipfs-store-resolve-failed = Неуспешно разрешаване на подателя за доставяне на ipfs-store отговор

# Изпращане на същности
bootstrap-complete = Bootstrap завършен
entity-loaded = Плъгинът на същността е зареден
entity-load-failed = Неуспешно зареждане на плъгина на същността
entity-not-found = Същността не е намерена, RPC се игнорира
entity-dispatched = RPC е изпратено до същността
entity-replied = Същността изпрати RPC отговор
root-create-entity = #root: създай същност
root-list-entities = #root: списък на същностите
root-delete-entity = #root: изтрий същност
root-entity-updated = Runtime манифестът е актуализиран
entity-created = Същността е създадена
entity-deleted = Същността е изтрита
entity-states-saving = Записване на състоянията на същностите в IPFS
entity-state-saving = Записване на състоянието на същността
entity-state-saved = Състоянието на същността е записано
entity-state-empty = Плъгинът върна празно състояние, записването е пропуснато
entity-states-saved = Състоянията на същностите са записани
link-set = Връзката е зададена
ftl-loaded = Езиковите съобщения са заредени от IPFS

# Първо стартиране / авто-инициализация
no-config-found = Не е намерена конфигурация.
initialising-new-identity = Инициализиране на нова runtime идентичност.
generated-headless-config = Headless конфигурацията е генерирана.

# Собственост
runtime-claimed = Runtime е регистриран.

# Защитени корени елементи
refuse-delete-root = Категорично отказвам да изтрия задължителен корен елемент
no-root-acl = Не е конфигуриран root ACL — runtime работи без контрол на достъпа
namespace-not-found = Пространството от имена не е намерено
no-ns-gate-acl = За това пространство от имена не е конфигуриран gate ACL
runtime-claim-persisted = Собственикът е записан в конфигурацията.
runtime-already-claimed = Runtime вече е регистриран.


# Namespace creation (:create)
acl-missing-owners = Основното ACL трябва да съдържа запис +owners
namespace-created = Пространството от имена е създадено
namespace-already-exists = Пространството от имена вече съществува
namespace-name-reserved = Запазено наименование на пространство
namespace-create-denied = Създаване на пространство от имена: достъп отказан
namespace-create-usage = Употреба: :create <ime>
crud-message-received = Получено CRUD съобщение
crud-acl-updated = Основният транспортен ACL е актуализиран
