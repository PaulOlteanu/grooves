pub struct PlayerOptions {
    pub shuffle: bool,
    pub start_element: Option<usize>,
}

impl PlayerOptions {
    pub fn new() -> Self {
        Self {
            shuffle: true,
            start_element: None,
        }
    }
}
