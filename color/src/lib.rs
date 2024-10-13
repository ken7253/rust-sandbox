struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[test]
fn initialize() {
    let color = Color::new(100, 20, 50);

    assert_eq!(color.red, 100);
    assert_eq!(color.green, 20);
    assert_eq!(color.blue, 50);
}
