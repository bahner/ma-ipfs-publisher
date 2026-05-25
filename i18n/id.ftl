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
acl-owners-access = Pemanggil diberi akses sebagai anggota +owners
namespace-not-found = Namespace tidak ditemukan
no-ns-gate-acl = ACL gate tidak dikonfigurasi untuk namespace ini
runtime-claim-persisted = Pemilik ditulis ke konfigurasi.
runtime-already-claimed = Runtime sudah terdaftar.


# Namespace creation (:create)
namespace-created = Ruang nama dibuat
namespace-already-exists = Ruang nama sudah ada
namespace-name-reserved = Nama ruang nama sudah dipesan
namespace-create-denied = Pembuatan namespace: akses ditolak
namespace-create-usage = Penggunaan: :create <nama>
crud-message-received = Pesan CRUD diterima
crud-acl-updated = ACL transport root diperbarui

# CRUD validation errors
blob-value-ipfs-path = nilai blob harus berupa jalur IPFS (/ipfs/, /ipns/, atau /ipld/)
acl-value-ipfs-path = nilai ACL harus berupa jalur IPFS (/ipfs/, /ipns/, atau /ipld/)
kind-value-ipfs-path = nilai kind harus berupa jalur IPFS (/ipfs/, /ipns/, atau /ipld/)
kind-not-found = Jenis tidak ditemukan
cidv1-required = nilai harus berupa CIDv1 mentah (dimulai dengan 'b'; CIDv0 'Qm…' tidak diterima)
config-key-protected = kunci config '%key%' dilindungi
config-key-no-delete = kunci config daemon '%key%' tidak dapat dihapus
config-key-not-manifest = kunci config '%key%' bukan kunci manifest config yang dikenal
wrong-crud-protocol = protokol CRUD salah: %type%
entity-name-invalid = nama entity harus berupa UTF-8 yang dapat dicetak
reserved-entity-name = nama entity '%name%' sudah dicadangkan
