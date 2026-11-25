#[cfg(target_os = "android")]
pub fn init() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("Nodule")
            .with_max_level(log::LevelFilter::Debug),
    );
}

#[cfg(not(target_os = "android"))]
pub fn init() {
    let _ = env_logger_init();
}

#[cfg(not(target_os = "android"))]
fn env_logger_init() -> Result<(), ()> {
    // no-op on desktop; wire env_logger later
    Ok(())
}