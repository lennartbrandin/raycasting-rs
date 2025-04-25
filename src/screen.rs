pub mod screen {
    use termion::*;
    use crate::common::common::*;

    pub struct Screen {
        x: usize,
        y: usize,
        buffer: Vec<Vec<Display>>
    }
    impl Screen {
        pub fn new() -> Self {
            let (x, y) = terminal_size().unwrap();
            let (x, y) = (usize::from(x), usize::from(y));
            Screen {
                x,
                y,
                buffer: vec![vec![DEFAULT_DISPLAY; y]; x],
            }
        }
    }
    impl fmt::Display for Screen {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for y in 0..self.y {
                for x in 0..self.x {
                    write!(f, "{}", self.buffer[x][y])?;
                }
            }
            for y in 0..self.y {
                for x in 0..self.x {
                    write!(f, "{}", self.buffer[x][y])?;
                }
            }
            Ok(())
        }
    }
}