use std::u8;

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

    fn reverse(&self) -> Color {
        let reversed = Color::new(
            u8::MAX - &self.red,
            u8::MAX - &self.green,
            u8::MAX - &self.blue,
        );

        return reversed;
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

#[test]
fn to_reverse_color() {
    let color = Color::new(255, 255, 255);
    let reverse = color.reverse();

    assert_eq!(reverse.red, 0);
    assert_eq!(reverse.green, 0);
    assert_eq!(reverse.blue, 0);
}
