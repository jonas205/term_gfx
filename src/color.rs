use std::io::Error;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn white() -> Color {
        Color::rgb(255, 255, 255)
    }

    pub fn cyan() -> Color {
        Color::rgb(0, 255, 255)
    }

    pub fn pink() -> Color {
        Color::rgb(255, 0, 255)
    }

    pub fn blue() -> Color {
        Color::rgb(0, 0, 255)
    }

    pub fn yellow() -> Color {
        Color::rgb(255, 255, 0)
    }

    pub fn green() -> Color {
        Color::rgb(0, 255, 0)
    }

    pub fn red() -> Color {
        Color::rgb(255, 0, 0)
    }

    pub fn black() -> Color {
        Color::rgb(0, 0, 0)
    }

    pub fn grey(grey: u8) -> Color {
        Color {
            red: grey,
            green: grey,
            blue: grey,
        }
    }

    fn color_to_c_str(color: u8) -> [u8; 3] {
        let mut array: [u8; 3] = [48; 3];

        let single = color % 10;
        let color = color / 10;

        array[2] += single;

        if color != 0 {
            let single = color % 10;
            let color = color / 10;

            array[1] += single;
            array[0] += color;
        }

        array
    }

    pub fn reset<R>(out: &mut R) -> Result<usize, Error>
    where
        R: std::io::Write,
    {
        out.write(b"\x1b[0m")
    }

    pub fn apply<R>(&self, out: &mut R) -> Result<usize, Error>
    where
        R: std::io::Write,
    {
        let mut setter: [u8; 38] = *b"\x1b[38;2;000;000;000m\x1b[48;2;000;000;000m";

        let r = Color::color_to_c_str(self.red);
        let g = Color::color_to_c_str(self.green);
        let b = Color::color_to_c_str(self.blue);

        for i in 0..=2 {
            setter[i + 7] = r[i];
            setter[i + 26] = r[i];
            setter[i + 11] = g[i];
            setter[i + 30] = g[i];
            setter[i + 15] = b[i];
            setter[i + 34] = b[i];
        }

        out.write(&setter)
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_color_to_c_str() {
        for i in 0..=255 {
            let s = format!("{:0>3}", i);
            assert_eq!(s.as_bytes(), crate::color::Color::color_to_c_str(i));
        }
    }
}
