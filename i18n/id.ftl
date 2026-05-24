# ma-runtime – Bahasa Indonesia
lang-name = Bahasa Indonesia

own-did-published = Dokumen DID sendiri telah diterbitkan ke IPNS
own-did-publish-failed = Gagal menerbitkan dokumen DID sendiri
own-did-publish-timeout = Penerbitan dokumen DID sendiri melewati batas waktu setelah 2 menit
started = ma runtime dimulai
shutdown-requested = Permintaan matikan diterima
closing-endpoint = Menutup iroh endpoint...
shutdown-complete = Matikan selesai
status-listening = Server status sedang mendengarkan
rpc-message-received = Pesan RPC diterima
rpc-message-rejected = Pesan RPC ditolak
ipfs-message-rejected = Pesan IPFS ditolak
ctrlc-handler-failed = Penangan Ctrl-C gagal
node-connected = Node terhubung ke protokol
received-encrypted-ma-msg = Pesan ma terenkripsi diterima di /ma/ipfs/0.0.1
unknown-rpc-atom = Atom RPC tidak dikenal, mengabaikan
rpc-not-text-atom = Muatan RPC bukan atom teks
rpc-unknown-verb = Perintah RPC tidak dikenal
rpc-reply-sent = Balasan RPC dikirim
ping-received = :ping diterima, mengirim :pong
did-publish-request-received = Permintaan penerbitan dokumen DID diterima
document-published = Dokumen diterbitkan
did-publish-cid-reply-sent = Balasan CID untuk penerbitan DID dikirim
did-publish-resolve-failed = Gagal menyelesaikan pengirim untuk pengiriman balasan ipfs-publish
ipfs-store-request-received = Permintaan penyimpanan IPFS diterima
ipfs-stored = Konten disimpan di IPFS
ipfs-store-cid-reply-sent = Balasan CID dikirim
ipfs-store-resolve-failed = Gagal menyelesaikan pengirim untuk pengiriman balasan ipfs-store

# Pengiriman entitas
bootstrap-complete = Bootstrap selesai
entity-loaded = Plugin entitas dimuat
entity-load-failed = Gagal memuat plugin entitas
entity-not-found = Entitas tidak ditemukan, mengabaikan RPC
entity-dispatched = RPC dikirim ke entitas
entity-replied = Entitas mengirim balasan RPC
root-create-entity = #root: buat entitas
root-list-entities = #root: daftar entitas
root-delete-entity = #root: hapus entitas
root-entity-updated = Manifes runtime diperbarui
entity-created = Entitas dibuat
entity-deleted = Entitas dihapus
entity-states-saving = Menyimpan status entitas ke IPFS
entity-state-saving = Menyimpan status entitas
entity-state-saved = Status entitas disimpan
entity-state-empty = Plugin mengembalikan status kosong, lewati penyimpanan
entity-states-saved = Status entitas disimpan
link-set = Tautan disetel
ftl-loaded = Pesan bahasa dimuat dari IPFS

# Pertama kali / inisialisasi otomatis
no-config-found = Konfigurasi tidak ditemukan.
initialising-new-identity = Menginisialisasi identitas runtime baru.
generated-headless-config = Konfigurasi headless dibuat.

# Kepemilikan
runtime-claimed = Runtime terdaftar.

# Elemen root yang dilindungi
refuse-delete-root = Menolak keras untuk menghapus elemen root yang diperlukan
no-root-acl = ACL root tidak dikonfigurasi — runtime berjalan tanpa kontrol akses
namespace-not-found = Namespace tidak ditemukan
no-ns-gate-acl = ACL gate tidak dikonfigurasi untuk namespace ini
runtime-claim-persisted = Pemilik ditulis ke konfigurasi.
runtime-already-claimed = Runtime sudah terdaftar.


# Namespace creation (:create)
acl-missing-owners = ACL akar harus memuat entri +owners
namespace-created = Ruang nama dibuat
namespace-already-exists = Ruang nama sudah ada
namespace-name-reserved = Nama ruang nama sudah dipesan
namespace-create-denied = Pembuatan namespace: akses ditolak
namespace-create-usage = Penggunaan: :create <nama>
crud-message-received = Pesan CRUD diterima
crud-acl-updated = ACL transport root diperbarui
