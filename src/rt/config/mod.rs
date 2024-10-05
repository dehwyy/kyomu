mod flags;

use tokio::time::Duration;

use flags::RuntimeFlags;

const DEFAULT_FPS: u8 = 90;
struct Frame {
    frames_per_second: u8,
    frame_time: Duration,
}

impl Frame {
    fn new(fps: u8) -> Self {
        Self {
            frames_per_second: fps,
            frame_time: Duration::from_secs_f64(1f64 / (fps as f64)),
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new(DEFAULT_FPS)
    }
}

pub struct RuntimeConfig {
    frame: Frame,
    flags: RuntimeFlags,
}

impl RuntimeConfig {
    pub fn new() -> Self {
        let args = std::env::args().collect::<Vec<String>>();

        let mut flags = vec![];

        let cfg_default = RuntimeConfig::default();
        let mut fps = cfg_default.get_fps();

        for arg in args {
            if arg.starts_with("--") {
                // "--fps=60" => ["fps", "60"]
                // "--debug" => ["debug"]
                let parsed = arg.as_str()[2..]
                    .split("=")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();

                if let Some(key) = parsed.get(0) {
                    let value = parsed.get(1);

                    if value.is_none() {
                        flags.push(key.clone());
                    }

                    if let Some(value) = value {
                        match key.as_str() {
                            "fps" => fps = value.parse().expect("failed to parse `fps` argument"),
                            _ => {}
                        }
                    }
                }
            }
        }

        Self {
            frame: Frame::new(fps),
            flags: RuntimeFlags::initialize(flags),
        }
    }

    pub const fn get_fps(&self) -> u8 {
        self.frame.frames_per_second
    }
    pub const fn get_frame_time(&self) -> Duration {
        self.frame.frame_time
    }
    pub const fn get_flags(&self) -> RuntimeFlags {
        self.flags
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            flags: RuntimeFlags::empty(),
            frame: Frame::default(),
        }
    }
}
