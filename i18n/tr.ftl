# ma-runtime – Türkçe
lang-name = Türkçe

own-did-published = Kendi DID belgesi IPNS'e yayınlandı
own-did-publish-failed = Kendi DID belgesi yayınlanamadı
own-did-publish-timeout = Kendi DID belgesinin yayınlanması 2 dakika sonra zaman aşımına uğradı
started = ma runtime başlatıldı
shutdown-requested = Kapatma istendi
closing-endpoint = iroh uç noktası kapatılıyor...
shutdown-complete = Kapatma tamamlandı
status-listening = Durum sunucusu dinliyor
rpc-message-received = RPC mesajı alındı
rpc-message-rejected = RPC mesajı reddedildi
ipfs-message-rejected = IPFS mesajı reddedildi
ctrlc-handler-failed = Ctrl-C işleyicisi başarısız oldu
node-connected = Düğüm protokole bağlandı
received-encrypted-ma-msg = /ma/ipfs/0.0.1 üzerinde şifreli ma mesajı alındı
unknown-rpc-atom = Bilinmeyen RPC atomu, yoksayılıyor
rpc-not-text-atom = RPC yükü metin atomu değil
rpc-unknown-verb = Bilinmeyen RPC fiili
rpc-reply-sent = RPC yanıtı gönderildi
ping-received = :ping alındı, :pong gönderiliyor
did-publish-request-received = DID belgesi yayınlama isteği alındı
document-published = Belge yayınlandı
did-publish-cid-reply-sent = DID yayınlama için CID yanıtı gönderildi
did-publish-resolve-failed = ipfs-publish yanıtı teslimi için gönderici çözümlenemedi
ipfs-store-request-received = IPFS depolama isteği alındı
ipfs-stored = İçerik IPFS'e depolandı
ipfs-store-cid-reply-sent = CID yanıtı gönderildi
ipfs-store-resolve-failed = ipfs-store yanıtı teslimi için gönderici çözümlenemedi

# Varlık gönderimi
bootstrap-complete = Bootstrap tamamlandı
entity-loaded = Varlık eklentisi yüklendi
entity-load-failed = Varlık eklentisi yüklenemedi
entity-not-found = Varlık bulunamadı, RPC yoksayılıyor
entity-dispatched = RPC varlığa gönderildi
entity-replied = Varlık RPC yanıtı gönderdi
root-create-entity = #root: varlık oluştur
root-list-entities = #root: varlıkları listele
root-delete-entity = #root: varlığı sil
root-entity-updated = Runtime manifestosu güncellendi
entity-created = Varlık oluşturuldu
entity-deleted = Varlık silindi
entity-states-saving = Varlık durumları IPFS'e kaydediliyor
entity-state-saving = Varlık durumu kaydediliyor
entity-state-saved = Varlık durumu kaydedildi
entity-state-empty = Eklenti boş durum döndürdü, kaydetme atlandı
entity-states-saved = Varlık durumları kaydedildi
link-set = Bağlantı ayarlandı
ftl-loaded = Dil mesajları IPFS'ten yüklendi

# İlk başlatma / otomatik başlatma
no-config-found = Yapılandırma bulunamadı.
initialising-new-identity = Yeni runtime kimliği başlatılıyor.
generated-headless-config = Başsız yapılandırma oluşturuldu.

# Sahiplik
runtime-claimed = Runtime kaydedildi.

# Korunan kök öğeler
refuse-delete-root = Gerekli kök öğeyi silmeyi kesinlikle reddediyorum
no-root-acl = Kök ACL yapılandırılmamış — runtime erişim kontrolü olmadan çalışıyor
acl-owners-access = Arayana +owners üyesi olarak erişim izni verildi
namespace-not-found = Ad alanı bulunamadı
no-ns-gate-acl = Bu ad alanı için kapı ACL'si yapılandırılmamış
runtime-claim-persisted = Sahip yapılandırmaya yazıldı.
runtime-already-claimed = Runtime zaten kayıtlı.


# Namespace creation (:create)
namespace-created = Ad alanı oluşturuldu
namespace-already-exists = Ad alanı zaten mevcut
namespace-name-reserved = Ad alanı adı ayrılmış
namespace-create-denied = Ad alanı oluşturma: erişim reddedildi
namespace-create-usage = Kullanım: :create <ad>
crud-message-received = CRUD mesajı alındı
crud-acl-updated = Kök taşıma ACL'si güncellendi

# CRUD validation errors
blob-value-ipfs-path = blob değeri bir IPFS yolu (/ipfs/, /ipns/ veya /ipld/) olmalıdır
acl-value-ipfs-path = ACL değeri bir IPFS yolu (/ipfs/, /ipns/ veya /ipld/) olmalıdır
kind-value-ipfs-path = kind değeri bir IPFS yolu (/ipfs/, /ipns/ veya /ipld/) olmalıdır
kind-not-found = Tür bulunamadı
cidv1-required = değer ham bir CIDv1 olmalıdır ('b' ile başlar; CIDv0 'Qm…' kabul edilmez)
config-key-protected = config anahtarı '%key%' korunmaktadır
config-key-no-delete = daemon config anahtarı '%key%' silinemez
config-key-not-manifest = config anahtarı '%key%' bilinen bir manifest config anahtarı değil
wrong-crud-protocol = yanlış CRUD protokolü: %type%
entity-name-invalid = entity adı yazdırılabilir UTF-8 olmalıdır
reserved-entity-name = entity adı '%name%' ayrılmıştır
