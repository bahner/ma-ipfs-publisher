# ma-runtime – Português
lang-name = Português

own-did-published = Documento DID próprio publicado no IPNS
own-did-publish-failed = Falha ao publicar o documento DID próprio
own-did-publish-timeout = Publicação do documento DID próprio expirou após 2 minutos
started = ma runtime iniciado
shutdown-requested = Encerramento solicitado
closing-endpoint = Fechando ponto de conexão iroh...
shutdown-complete = Encerramento concluído
status-listening = Servidor de status a escutar
rpc-message-received = Mensagem RPC recebida
rpc-message-rejected = Mensagem RPC rejeitada
ipfs-message-rejected = Mensagem IPFS rejeitada
ctrlc-handler-failed = Falha no manipulador Ctrl-C
node-connected = Nó conectado ao protocolo
received-encrypted-ma-msg = Mensagem ma cifrada recebida em /ma/ipfs/0.0.1
unknown-rpc-atom = Átomo RPC desconhecido, ignorando
rpc-not-text-atom = O payload RPC não é um átomo de texto
rpc-unknown-verb = Verbo RPC desconhecido
rpc-reply-sent = Resposta RPC enviada
ping-received = :ping recebido, enviando :pong
did-publish-request-received = Pedido de publicação de documento DID recebido
document-published = Documento publicado
did-publish-cid-reply-sent = Resposta CID enviada para publicação DID
did-publish-resolve-failed = Não foi possível resolver o remetente para entregar a resposta ipfs-publish
ipfs-store-request-received = Pedido de armazenamento IPFS recebido
ipfs-stored = Conteúdo armazenado no IPFS
ipfs-store-cid-reply-sent = Resposta CID enviada
ipfs-store-resolve-failed = Não foi possível resolver o remetente para entregar a resposta ipfs-store

# Despacho de entidades
bootstrap-complete = Bootstrap concluído
entity-loaded = Plugin de entidade carregado
entity-load-failed = Falha ao carregar o plugin de entidade
entity-not-found = Entidade não encontrada, ignorando RPC
entity-dispatched = RPC despachado para a entidade
entity-replied = Entidade enviou resposta RPC
root-create-entity = #root: criar entidade
root-list-entities = #root: listar entidades
root-delete-entity = #root: eliminar entidade
root-entity-updated = Manifesto runtime atualizado
entity-created = Entidade criada
entity-deleted = Entidade eliminada
entity-states-saving = Guardando estados de entidades no IPFS
entity-state-saving = Guardando estado de entidade
entity-state-saved = Estado de entidade guardado
entity-state-empty = Plugin retornou estado vazio, ignorando persistência
entity-states-saved = Estados de entidades guardados
link-set = Ligação definida
ftl-loaded = Mensagens de idioma carregadas do IPFS

# Primeiro arranque / auto-init
no-config-found = Nenhuma configuração encontrada.
initialising-new-identity = A inicializar nova identidade runtime.
generated-headless-config = Configuração headless gerada.

# Propriedade
runtime-claimed = Runtime registado.

# Elementos raiz protegidos
refuse-delete-root = Recuso-me firmemente a eliminar um elemento raiz necessário
no-root-acl = Nenhuma ACL raiz configurada — o runtime opera sem controlo de acesso
acl-owners-access = O chamador obteve acesso como membro de +owners
namespace-not-found = Espaço de nomes não encontrado
no-ns-gate-acl = Nenhuma ACL de porta configurada para este espaço de nomes
runtime-claim-persisted = Proprietário escrito na configuração.
runtime-already-claimed = Runtime já registado.


# Namespace creation (:create)
namespace-created = Espaço de nomes criado
namespace-already-exists = Espaço de nomes já existe
namespace-name-reserved = Nome de espaço de nomes reservado
namespace-create-denied = Criação de espaço de nomes: acesso negado
namespace-create-usage = Uso: :create <nome>
crud-message-received = Mensagem CRUD recebida
crud-acl-updated = ACL de transporte raiz atualizada

# CRUD validation errors
blob-value-ipfs-path = o valor blob deve ser um caminho IPFS (/ipfs/, /ipns/ ou /ipld/)
acl-value-ipfs-path = o valor ACL deve ser um caminho IPFS (/ipfs/, /ipns/ ou /ipld/)
kind-value-ipfs-path = o valor kind deve ser um caminho IPFS (/ipfs/, /ipns/ ou /ipld/)
kind-not-found = Tipo não encontrado
cidv1-required = o valor deve ser um CIDv1 puro (começa com 'b'; CIDv0 'Qm…' não aceito)
config-key-protected = a chave de config '%key%' está protegida
config-key-no-delete = a chave de config '%key%' do daemon não pode ser eliminada
config-key-not-manifest = a chave de config '%key%' não é uma chave de manifest config conhecida
wrong-crud-protocol = protocolo CRUD incorreto: %type%
entity-name-invalid = o nome da entity deve ser UTF-8 imprimível
reserved-entity-name = o nome da entity '%name%' está reservado
