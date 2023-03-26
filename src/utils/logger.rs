use log_fastly::Logger;

pub fn init() {
    Logger::builder()
        .max_level(log::LevelFilter::Info)
        .default_endpoint("");
}
