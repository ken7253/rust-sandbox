struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    fn to_hex(&self) -> String {
        return format!("#{:X}{:X}{:X}", self.red, self.green, self.blue);
    }
}

#[test]
fn initialize() {
    let color = Color::new(100, 20, 50);

    assert_eq!(color.red, 100);
    assert_eq!(color.green, 20);
    assert_eq!(color.blue, 50);
}

#[test]
fn to_hex_color() {
    let color = Color::new(255, 255, 255);
    let hex = color.to_hex();

    let correct = "#FFFFFF";
    assert_eq!(hex, correct);
}
