pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn new(entries: Vec<String>) -> Self {
        Self { entries }
    }
}