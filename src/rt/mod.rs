pub mod config;

use config::RuntimeConfig;
use std::sync::OnceLock;
use tokio::sync::RwLock;

static RT_CONFIG: OnceLock<RwLock<RuntimeConfig>> = OnceLock::new();

pub fn get_rt_config() -> &'static RwLock<RuntimeConfig> {
    RT_CONFIG.get_or_init(|| RwLock::new(RuntimeConfig::new()))
}
