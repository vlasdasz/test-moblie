#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(_app: winit::platform::android::activity::AndroidApp) {
    dbg!("Hello");
}
