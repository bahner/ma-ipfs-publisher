# ma-runtime – Tiếng Việt
lang-name = Tiếng Việt

own-did-published = Tài liệu DID của mình đã được công bố lên IPNS
own-did-publish-failed = Không thể công bố tài liệu DID của mình
own-did-publish-timeout = Việc công bố tài liệu DID của mình đã hết thời gian sau 2 phút
started = ma runtime đã khởi động
shutdown-requested = Yêu cầu tắt máy đã được gửi
closing-endpoint = Đang đóng iroh endpoint...
shutdown-complete = Tắt máy hoàn tất
status-listening = Máy chủ trạng thái đang lắng nghe
rpc-message-received = Đã nhận tin nhắn RPC
rpc-message-rejected = Tin nhắn RPC đã bị từ chối
ipfs-message-rejected = Tin nhắn IPFS đã bị từ chối
ctrlc-handler-failed = Trình xử lý Ctrl-C thất bại
node-connected = Nút đã kết nối với giao thức
received-encrypted-ma-msg = Đã nhận tin nhắn ma được mã hóa tại /ma/ipfs/0.0.1
unknown-rpc-atom = Atom RPC không xác định, đang bỏ qua
rpc-reply-sent = Đã gửi phản hồi RPC
ping-received = Đã nhận :ping, đang gửi :pong
did-publish-request-received = Đã nhận yêu cầu công bố tài liệu DID
document-published = Tài liệu đã được công bố
did-publish-cid-reply-sent = Đã gửi phản hồi CID cho việc công bố DID
did-publish-resolve-failed = Không thể xác định người gửi để gửi phản hồi ipfs-publish
ipfs-store-request-received = Đã nhận yêu cầu lưu trữ IPFS
ipfs-stored = Nội dung đã được lưu trữ trên IPFS
ipfs-store-cid-reply-sent = Đã gửi phản hồi CID
ipfs-store-resolve-failed = Không thể xác định người gửi để gửi phản hồi ipfs-store

# Điều phối thực thể
bootstrap-complete = Bootstrap hoàn tất
entity-loaded = Plugin thực thể đã được tải
entity-load-failed = Không thể tải plugin thực thể
entity-not-found = Không tìm thấy thực thể, bỏ qua RPC
entity-dispatched = RPC đã được điều phối đến thực thể
entity-replied = Thực thể đã gửi phản hồi RPC
root-create-entity = #root: tạo thực thể
root-list-entities = #root: danh sách thực thể
root-delete-entity = #root: xóa thực thể
root-entity-updated = Manifest runtime đã được cập nhật
entity-created = Thực thể đã được tạo
entity-deleted = Thực thể đã bị xóa
entity-states-saving = Đang lưu trạng thái thực thể vào IPFS
entity-state-saving = Đang lưu trạng thái thực thể
entity-state-saved = Đã lưu trạng thái thực thể
entity-state-empty = Plugin trả về trạng thái rỗng, bỏ qua việc lưu
entity-states-saved = Đã lưu các trạng thái thực thể
link-set = Đã đặt liên kết
ftl-loaded = Tin nhắn ngôn ngữ đã được tải từ IPFS

# Khởi động lần đầu / tự động khởi tạo
no-config-found = Không tìm thấy cấu hình.
initialising-new-identity = Đang khởi tạo danh tính runtime mới.
generated-headless-config = Đã tạo cấu hình headless.

# Quyền sở hữu
runtime-claimed = Runtime đã được đăng ký.

# Các phần tử root được bảo vệ
refuse-delete-root = Kiên quyết từ chối xóa phần tử root cần thiết
no-root-acl = ACL root chưa được cấu hình — runtime đang chạy mà không có kiểm soát truy cập
namespace-not-found = Không tìm thấy namespace
no-ns-gate-acl = ACL cổng chưa được cấu hình cho namespace này
runtime-claim-persisted = Chủ sở hữu đã được ghi vào cấu hình.
runtime-already-claimed = Runtime đã được đăng ký rồi.


# Namespace creation (:create)
acl-missing-owners = ACL gốc phải chứa mục nhập +owners
namespace-created = Namespace đã được tạo
namespace-already-exists = Namespace đã tồn tại
namespace-name-reserved = Tên namespace đã được đặt trước
namespace-create-denied = Tạo namespace: từ chối truy cập
namespace-create-usage = Cách dùng: :create <tên>
