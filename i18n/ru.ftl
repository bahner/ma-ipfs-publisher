# ma-runtime – Русский
lang-name = Русский

own-did-published = Собственный DID-документ опубликован в IPNS
own-did-publish-failed = Не удалось опубликовать собственный DID-документ
own-did-publish-timeout = Публикация собственного DID-документа прервана по истечении 2 минут
started = ma runtime запущен
shutdown-requested = Запрошено завершение работы
closing-endpoint = Закрытие конечной точки iroh...
shutdown-complete = Завершение работы выполнено
status-listening = Сервер статуса ожидает подключений
rpc-message-received = Получено RPC-сообщение
rpc-message-rejected = RPC-сообщение отклонено
ipfs-message-rejected = IPFS-сообщение отклонено
ctrlc-handler-failed = Сбой обработчика Ctrl-C
node-connected = Узел подключён к протоколу
received-encrypted-ma-msg = Получено зашифрованное ma-сообщение на /ma/ipfs/0.0.1
unknown-rpc-atom = Неизвестный RPC-атом, игнорируется
rpc-not-text-atom = Нагрузка RPC не является текстовым атомом
rpc-unknown-verb = Неизвестный RPC-глагол
rpc-reply-sent = RPC-ответ отправлен
ping-received = Получен :ping, отправляю :pong
did-publish-request-received = Получен запрос на публикацию DID-документа
document-published = Документ опубликован
did-publish-cid-reply-sent = Отправлен CID-ответ для публикации DID
did-publish-resolve-failed = Не удалось определить отправителя для доставки ответа ipfs-publish
ipfs-store-request-received = Получен запрос на сохранение в IPFS
ipfs-stored = Содержимое сохранено в IPFS
ipfs-store-cid-reply-sent = CID-ответ отправлен
ipfs-store-resolve-failed = Не удалось определить отправителя для доставки ответа ipfs-store

# Диспетчеризация сущностей
bootstrap-complete = Bootstrap завершён
entity-loaded = Плагин сущности загружен
entity-load-failed = Не удалось загрузить плагин сущности
entity-not-found = Сущность не найдена, RPC игнорируется
entity-dispatched = RPC отправлен сущности
entity-replied = Сущность отправила RPC-ответ
root-create-entity = #root: создать сущность
root-list-entities = #root: список сущностей
root-delete-entity = #root: удалить сущность
root-entity-updated = Манифест runtime обновлён
entity-created = Сущность создана
entity-deleted = Сущность удалена
entity-states-saving = Сохранение состояний сущностей в IPFS
entity-state-saving = Сохранение состояния сущности
entity-state-saved = Состояние сущности сохранено
entity-state-empty = Плагин вернул пустое состояние, сохранение пропущено
entity-states-saved = Состояния сущностей сохранены
link-set = Ссылка установлена
ftl-loaded = Языковые сообщения загружены из IPFS

# Первый запуск / авто-инициализация
no-config-found = Конфигурация не найдена.
initialising-new-identity = Инициализация новой runtime-идентичности.
generated-headless-config = Безголовая конфигурация сгенерирована.

# Владение
runtime-claimed = Runtime зарегистрирован.

# Защищённые корневые элементы
refuse-delete-root = Категорически отказываюсь удалять обязательный корневой элемент
no-root-acl = Корневой ACL не настроен — runtime работает без контроля доступа
acl-owners-access = Вызывающему предоставлен доступ как члену группы +owners
namespace-not-found = Пространство имён не найдено
no-ns-gate-acl = ACL шлюза не настроен для этого пространства имён
runtime-claim-persisted = Владелец записан в конфигурацию.
runtime-already-claimed = Runtime уже зарегистрирован.


# Namespace creation (:create)
namespace-created = Пространство имён создано
namespace-already-exists = Пространство имён уже существует
namespace-name-reserved = Имя пространства имён зарезервировано
namespace-create-denied = Создание пространства имён: доступ запрещён
namespace-create-usage = Использование: :create <имя>
crud-message-received = Получено CRUD-сообщение
crud-acl-updated = Корневой транспортный ACL обновлён

# CRUD validation errors
blob-value-ipfs-path = значение blob должно быть путём IPFS (/ipfs/, /ipns/ или /ipld/)
acl-value-ipfs-path = значение ACL должно быть путём IPFS (/ipfs/, /ipns/ или /ipld/)
kind-value-ipfs-path = значение kind должно быть путём IPFS (/ipfs/, /ipns/ или /ipld/)
kind-not-found = Тип не найден
cidv1-required = значение должно быть голым CIDv1 (начинается с 'b'; CIDv0 'Qm…' не принимается)
config-key-protected = ключ config '%key%' защищён
config-key-no-delete = ключ config '%key%' демона не может быть удалён
config-key-not-manifest = ключ config '%key%' не является известным ключом manifest config
wrong-crud-protocol = неверный протокол CRUD: %type%
entity-name-invalid = имя entity должно быть печатным UTF-8
reserved-entity-name = имя entity '%name%' зарезервировано
