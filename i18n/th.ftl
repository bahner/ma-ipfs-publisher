# ma-runtime – ภาษาไทย
lang-name = ภาษาไทย

own-did-published = เผยแพร่เอกสาร DID ของตนเองไปยัง IPNS แล้ว
own-did-publish-failed = ล้มเหลวในการเผยแพร่เอกสาร DID ของตนเอง
own-did-publish-timeout = การเผยแพร่เอกสาร DID ของตนเองหมดเวลาหลังจาก 2 นาที
started = ma runtime เริ่มทำงานแล้ว
shutdown-requested = ได้รับคำขอปิดระบบ
closing-endpoint = กำลังปิด iroh endpoint...
shutdown-complete = การปิดระบบเสร็จสมบูรณ์
status-listening = เซิร์ฟเวอร์สถานะกำลังรับฟัง
rpc-message-received = ได้รับข้อความ RPC
rpc-message-rejected = ปฏิเสธข้อความ RPC แล้ว
ipfs-message-rejected = ปฏิเสธข้อความ IPFS แล้ว
ctrlc-handler-failed = ตัวจัดการ Ctrl-C ล้มเหลว
node-connected = โหนดเชื่อมต่อกับโปรโตคอลแล้ว
received-encrypted-ma-msg = ได้รับข้อความ ma ที่เข้ารหัสแล้วที่ /ma/ipfs/0.0.1
unknown-rpc-atom = RPC อะตอมที่ไม่รู้จัก กำลังละเว้น
rpc-reply-sent = ส่งการตอบกลับ RPC แล้ว
ping-received = ได้รับ :ping กำลังส่ง :pong
did-publish-request-received = ได้รับคำขอเผยแพร่เอกสาร DID
document-published = เผยแพร่เอกสารแล้ว
did-publish-cid-reply-sent = ส่งการตอบกลับ CID สำหรับการเผยแพร่ DID แล้ว
did-publish-resolve-failed = ไม่สามารถแก้ไขผู้ส่งเพื่อส่งการตอบกลับ ipfs-publish
ipfs-store-request-received = ได้รับคำขอจัดเก็บ IPFS
ipfs-stored = จัดเก็บเนื้อหาไปยัง IPFS แล้ว
ipfs-store-cid-reply-sent = ส่งการตอบกลับ CID แล้ว
ipfs-store-resolve-failed = ไม่สามารถแก้ไขผู้ส่งเพื่อส่งการตอบกลับ ipfs-store

# การส่งเอนทิตี
bootstrap-complete = Bootstrap เสร็จสมบูรณ์
entity-loaded = โหลดปลั๊กอินเอนทิตีแล้ว
entity-load-failed = ล้มเหลวในการโหลดปลั๊กอินเอนทิตี
entity-not-found = ไม่พบเอนทิตี ละเว้น RPC
entity-dispatched = ส่ง RPC ไปยังเอนทิตีแล้ว
entity-replied = เอนทิตีส่งการตอบกลับ RPC แล้ว
root-create-entity = #root: สร้างเอนทิตี
root-list-entities = #root: รายการเอนทิตี
root-delete-entity = #root: ลบเอนทิตี
root-entity-updated = อัปเดตไฟล์ manifest ของ runtime แล้ว
entity-created = สร้างเอนทิตีแล้ว
entity-deleted = ลบเอนทิตีแล้ว
entity-states-saving = กำลังบันทึกสถานะเอนทิตีไปยัง IPFS
entity-state-saving = กำลังบันทึกสถานะเอนทิตี
entity-state-saved = บันทึกสถานะเอนทิตีแล้ว
entity-state-empty = ปลั๊กอินส่งคืนสถานะว่าง ข้ามการบันทึก
entity-states-saved = บันทึกสถานะเอนทิตีแล้ว
link-set = ตั้งค่าลิงก์แล้ว
ftl-loaded = โหลดข้อความภาษาจาก IPFS แล้ว

# การเริ่มต้นครั้งแรก / การเริ่มต้นอัตโนมัติ
no-config-found = ไม่พบการกำหนดค่า
initialising-new-identity = กำลังเริ่มต้น runtime identity ใหม่
generated-headless-config = สร้างการกำหนดค่าแบบ headless แล้ว

# ความเป็นเจ้าของ
runtime-claimed = ลงทะเบียน runtime แล้ว

# องค์ประกอบ root ที่ได้รับการปกป้อง
refuse-delete-root = ปฏิเสธอย่างเด็ดขาดที่จะลบองค์ประกอบ root ที่จำเป็น
no-root-acl = ไม่ได้กำหนดค่า ACL ของ root — runtime ทำงานโดยไม่มีการควบคุมการเข้าถึง
namespace-not-found = ไม่พบ namespace
no-ns-gate-acl = ไม่ได้กำหนดค่า ACL ของเกตสำหรับ namespace นี้
runtime-claim-persisted = เขียนเจ้าของลงในการกำหนดค่าแล้ว
runtime-already-claimed = runtime ลงทะเบียนแล้ว


# Namespace creation (:create)
acl-missing-owners = ACL รากต้องมีรายการ +owners
namespace-created = สร้าง namespace แล้ว
namespace-already-exists = namespace มีอยู่แล้ว
namespace-name-reserved = ชื่อ namespace ถูกสงวนไว้
namespace-create-denied = สร้าง namespace: ปฏิเสธการเข้าถึง
namespace-create-usage = การใช้งาน: :create <ชื่อ>
