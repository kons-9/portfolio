pub struct Style {
    display: &'static str,
    width: i32,
    height: i32,
}

impl Style {
    pub fn new() -> Self {
        Style {
            display: "",
            width: 0,
            height: 0,
        }
    }
}
