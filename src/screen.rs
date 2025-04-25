pub mod screen {
    use termion::*;
    use crate::common::common::*;

    pub struct Screen {
        x: u16,
        y: u16,
        buffer: Vec<Vec<Display>>
    }
    impl Screen {
        pub fn new() -> Self {
            let (x, y) = terminal_size().unwrap();
            info!("Terminal dimensions: {}x{}", x, y);
            let mut s = Screen {
                x,
                y,
                buffer: vec![vec![DEFAULT_DISPLAY; y as usize]; x as usize],
            };
            for i in 0..y {
                for j in 0..x {
                    s.buffer[j as usize][i as usize] = char::from_digit((i%32) as u32, 32).unwrap();
                }
            }
            s
        }
        fn clear_screen(&self) {
            print!("{}", cursor::Up(self.y));
        }
    }
    impl fmt::Display for Screen {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clear_screen();

            for y in 0..self.y {
                for x in 0..self.x {
                    write!(f, "{}", self.buffer[x as usize][y as usize])?;
                }
                if y != self.y-1 {
                    write!(f, "\n")?;
                }
            }

            Ok(())
        }
    }
}