use ctor::ctor;

#[ctor]
fn entry() {
    crate::logger::init();
    log::info!(
        "Nodule {} ({}) loaded — package {}",
        crate::VERSION_NAME,
        crate::VERSION_CODE,
        crate::PACKAGE_ID
    );
    std::thread::spawn(|| {
        log::info!("Nodule init thread started");
        // later we will find libcocos2dcpp.so, install hooks, etc.
    });
}