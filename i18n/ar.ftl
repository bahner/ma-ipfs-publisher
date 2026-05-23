# ma-runtime – العربية
lang-name = العربية

own-did-published = تم نشر وثيقة DID الخاصة بي إلى IPNS
own-did-publish-failed = فشل نشر وثيقة DID الخاصة بي
own-did-publish-timeout = انتهت مهلة نشر وثيقة DID الخاصة بي بعد دقيقتين
started = بدأ تشغيل ma runtime
shutdown-requested = تم طلب الإيقاف
closing-endpoint = جارٍ إغلاق نقطة نهاية iroh...
shutdown-complete = اكتمل الإيقاف
status-listening = خادم الحالة يستمع
rpc-message-received = تم استلام رسالة RPC
rpc-message-rejected = تم رفض رسالة RPC
ipfs-message-rejected = تم رفض رسالة IPFS
ctrlc-handler-failed = فشل معالج Ctrl-C
node-connected = اتصل العقدة بالبروتوكول
received-encrypted-ma-msg = تم استلام رسالة ma مشفرة على /ma/ipfs/0.0.1
unknown-rpc-atom = ذرة RPC غير معروفة، يتم التجاهل
rpc-reply-sent = تم إرسال رد RPC
ping-received = تم استلام :ping، جارٍ إرسال :pong
did-publish-request-received = تم استلام طلب نشر وثيقة DID
document-published = تم نشر الوثيقة
did-publish-cid-reply-sent = تم إرسال رد CID لنشر DID
did-publish-resolve-failed = فشل حل المرسل لتسليم رد ipfs-publish
ipfs-store-request-received = تم استلام طلب تخزين IPFS
ipfs-stored = تم تخزين المحتوى في IPFS
ipfs-store-cid-reply-sent = تم إرسال رد CID
ipfs-store-resolve-failed = فشل حل المرسل لتسليم رد ipfs-store

# توزيع الكيانات
bootstrap-complete = اكتملت عملية Bootstrap
entity-loaded = تم تحميل مكون الكيان
entity-load-failed = فشل تحميل مكون الكيان
entity-not-found = لم يتم العثور على الكيان، يتم تجاهل RPC
entity-dispatched = تم إرسال RPC إلى الكيان
entity-replied = أرسل الكيان ردًا على RPC
root-create-entity = #root: إنشاء كيان
root-list-entities = #root: قائمة الكيانات
root-delete-entity = #root: حذف كيان
root-entity-updated = تم تحديث بيان runtime
entity-created = تم إنشاء الكيان
entity-deleted = تم حذف الكيان
entity-states-saving = جارٍ حفظ حالات الكيانات إلى IPFS
entity-state-saving = جارٍ حفظ حالة الكيان
entity-state-saved = تم حفظ حالة الكيان
entity-state-empty = أعاد المكون حالة فارغة، يتم تخطي الحفظ
entity-states-saved = تم حفظ حالات الكيانات
link-set = تم تعيين الرابط
ftl-loaded = تم تحميل رسائل اللغة من IPFS

# أول تشغيل / التهيئة التلقائية
no-config-found = لم يتم العثور على تكوين.
initialising-new-identity = جارٍ تهيئة هوية runtime جديدة.
generated-headless-config = تم إنشاء تكوين headless.

# الملكية
runtime-claimed = تم تسجيل runtime.

# عناصر الجذر المحمية
refuse-delete-root = أرفض رفضًا قاطعًا حذف عنصر الجذر المطلوب
no-root-acl = لم يتم تكوين ACL الجذر — يعمل runtime بدون التحكم في الوصول
namespace-not-found = لم يتم العثور على مساحة الاسم
no-ns-gate-acl = لم يتم تكوين ACL للبوابة لهذه مساحة الاسم
runtime-claim-persisted = تمت كتابة المالك في التكوين.
runtime-already-claimed = تم تسجيل runtime مسبقًا.


# Namespace creation (:create)
acl-missing-owners = يجب أن يحتوي ACL الجذري على مدخل +owners
namespace-created = تم إنشاء مساحة الاسم
namespace-already-exists = مساحة الاسم موجودة بالفعل
namespace-name-reserved = اسم مساحة الاسم محجوز
namespace-create-denied = إنشاء مساحة الاسم: تم رفض الوصول
namespace-create-usage = الاستخدام: :create <الاسم>
