pub(crate) struct KError {
    pub message: String,
    pub level: Level,
}

pub enum Level {
    MINOR,
    MEDIUM,
    SEVERE,
}

impl KError {
    pub fn new(level: Level) -> KError {
        KError {
            level,
            message: String::from("An error"),
        }
    }
    pub fn new_minor(message: &str) -> KError {
        KError {
            level: Level::MINOR,
            message: String::from(message),
        }
    }
    pub fn new_medium(message: &str) -> KError {
        KError {
            level: Level::MEDIUM,
            message: String::from(message),
        }
    }
    pub fn new_severe(message: &str) -> KError {
        KError {
            level: Level::SEVERE,
            message: String::from(message),
        }
    }
}
