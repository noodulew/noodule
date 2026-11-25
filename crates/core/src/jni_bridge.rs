use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_org_noodule_launcher_native_NoduleNative_versionName(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string(crate::VERSION_NAME).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_noodule_launcher_native_NoduleNative_versionCode(
    _env: JNIEnv,
    _class: JClass,
) -> jni::sys::jint {
    crate::VERSION_CODE
}

#[no_mangle]
pub extern "system" fn Java_org_noodule_launcher_native_NoduleNative_packageId(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string(crate::PACKAGE_ID).unwrap().into_raw()
}