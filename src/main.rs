use embassy_time::{Duration, Timer};
use esp_idf_hal::task::executor::EspExecutor;
use esp_idf_sys::{self as sys, esp, EspError};

sys::esp_app_desc!();

fn main() -> Result<(), EspError> {
    init()?;
    run()
}

fn init() -> Result<(), EspError> {
    sys::link_patches();
    esp_idf_hal::cs::critical_section::link();
    esp_idf_svc::timer::embassy_time::driver::link();
    esp_idf_svc::timer::embassy_time::queue::link();

    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe {
        #[allow(clippy::needless_update)]
        esp!(sys::esp_vfs_eventfd_register(
            &sys::esp_vfs_eventfd_config_t {
                max_fds: 5,
                ..Default::default()
            }
        ))?;
    }

    Ok(())
}

fn run() -> Result<(), EspError> {
    let executor = EspExecutor::<16, _>::new();
    let tasks = [
        executor.spawn_local(async move {
            log::info!("starting");
            loop {
                Timer::after(Duration::from_secs(5)).await;
                log::info!("woke up");
            }
        }).unwrap(),
    ];
    executor.run_tasks(|| true, tasks);
    Ok(())
}
