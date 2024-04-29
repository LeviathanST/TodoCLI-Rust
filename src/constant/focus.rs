pub enum FOCUS {
    TODO,
    DONE,
}

impl FOCUS {
    pub fn toggle(&self) -> Self {
        match self {
            FOCUS::TODO => FOCUS::DONE,
            FOCUS::DONE => FOCUS::TODO,
        }
    }
}