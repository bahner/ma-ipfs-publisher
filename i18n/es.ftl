# ma-runtime – Español
lang-name = Español

own-did-published = Documento DID propio publicado en IPNS
own-did-publish-failed = Error al publicar el documento DID propio
own-did-publish-timeout = La publicación del documento DID propio expiró después de 2 minutos
started = ma runtime iniciado
shutdown-requested = Apagado solicitado
closing-endpoint = Cerrando punto de conexión iroh...
shutdown-complete = Apagado completado
status-listening = Servidor de estado escuchando
rpc-message-received = Mensaje RPC recibido
rpc-message-rejected = Mensaje RPC rechazado
ipfs-message-rejected = Mensaje IPFS rechazado
ctrlc-handler-failed = Error en el manejador Ctrl-C
node-connected = Nodo conectado al protocolo
received-encrypted-ma-msg = Mensaje ma cifrado recibido en /ma/ipfs/0.0.1
unknown-rpc-atom = Átomo RPC desconocido, ignorando
rpc-reply-sent = Respuesta RPC enviada
ping-received = :ping recibido, enviando :pong
did-publish-request-received = Solicitud de publicación de documento DID recibida
document-published = Documento publicado
did-publish-cid-reply-sent = Respuesta CID enviada para publicación DID
did-publish-resolve-failed = No se pudo resolver el remitente para entregar la respuesta ipfs-publish
ipfs-store-request-received = Solicitud de almacenamiento IPFS recibida
ipfs-stored = Contenido almacenado en IPFS
ipfs-store-cid-reply-sent = Respuesta CID enviada
ipfs-store-resolve-failed = No se pudo resolver el remitente para entregar la respuesta ipfs-store

# Despacho de entidades
bootstrap-complete = Bootstrap completado
entity-loaded = Plugin de entidad cargado
entity-load-failed = Error al cargar el plugin de entidad
entity-not-found = Entidad no encontrada, ignorando RPC
entity-dispatched = RPC despachado a la entidad
entity-replied = La entidad envió respuesta RPC
root-create-entity = #root: crear entidad
root-list-entities = #root: listar entidades
root-delete-entity = #root: eliminar entidad
root-entity-updated = Manifiesto runtime actualizado
entity-created = Entidad creada
entity-deleted = Entidad eliminada
entity-states-saving = Guardando estados de entidades en IPFS
entity-state-saving = Guardando estado de entidad
entity-state-saved = Estado de entidad guardado
entity-state-empty = El plugin devolvió estado vacío, omitiendo persistencia
entity-states-saved = Estados de entidades guardados
link-set = Enlace establecido
ftl-loaded = Mensajes de idioma cargados desde IPFS

# Primer inicio / auto-init
no-config-found = No se encontró configuración.
initialising-new-identity = Inicializando nueva identidad runtime.
generated-headless-config = Configuración headless generada.

# Propiedad
runtime-claimed = Runtime registrado.

# Elementos raíz protegidos
refuse-delete-root = Me niego firmemente a eliminar un elemento raíz requerido
no-root-acl = No hay ACL raíz configurada — el runtime opera sin control de acceso
namespace-not-found = Espacio de nombres no encontrado
no-ns-gate-acl = No hay ACL de puerta configurada para este espacio de nombres
runtime-claim-persisted = Propietario escrito en la configuración.
runtime-already-claimed = Runtime ya registrado.


# Namespace creation (:create)
acl-missing-owners = El ACL raíz debe contener una entrada +owners
namespace-created = Espacio de nombres creado
namespace-already-exists = El espacio de nombres ya existe
namespace-name-reserved = Nombre de espacio de nombres reservado
namespace-create-denied = Creación de espacio de nombres: acceso denegado
namespace-create-usage = Uso: :create <nombre>
