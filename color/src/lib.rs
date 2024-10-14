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

    fn complementary(&self) -> Color {
        let max = self.red.max(self.green.max(self.blue));
        let min = self.red.min(self.green.min(self.blue));
        let buf = max + min;
        let to_complementary = Color::new(buf - self.red, buf - self.green, buf - self.blue);

        return to_complementary;
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;

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

    #[test]
    fn to_complementary_color() {
        let color = Color::new(102, 153, 51);
        let complementary = color.complementary();

        assert_eq!(complementary.red, 102);
        assert_eq!(complementary.green, 51);
        assert_eq!(complementary.blue, 153);
    }
}
