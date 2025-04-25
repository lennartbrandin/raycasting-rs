pub mod game {
    use crate::common::common::*;

    #[derive(Clone, Copy)]
    struct Object {
        display: Display, // Map representation
        transparent: bool, // See through?
        traversable: bool, // Walk through?
    }
    impl Object {
        const fn new() -> Self {
            Object { display: 'W', transparent: false, traversable: false }
        }
    }
    impl Default for Object {
        fn default() -> Self {
            Object::new()
        }
    }
    impl fmt::Display for Object {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.display)
        }
    }

    struct Entity {
        properties: Object,
        x: f32,
        y: f32,
        angle: u16,
        fov: u16,
    }
    impl Entity {
        const fn new() -> Self {
            Entity {
                properties: Object { display: 'P', transparent: false, traversable: false },
                x: (ROOM_SIZE/2) as f32,
                y: (ROOM_SIZE/2) as f32,
                angle: 0,
                fov: 60,
            }
        }
    }
    impl fmt::Display for Entity{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "{} at ({}/{}): {}°, fov: {}", self.properties.display, self.x, self.y, self.angle, self.fov)
        }
    }

    const ROOM_SIZE: usize = 10;
    type Room = [[Object; ROOM_SIZE]; ROOM_SIZE];

    pub struct Game {
        player: Entity,
        map: Room,
    }
    impl Game {
        pub const fn new() -> Self {
            Game { 
                player: Entity::new(),
                map: Game::generate_room(),
            }
        }


        const EMPTY: Object = Object { display: ' ', transparent: true, traversable: true };
        const WALL: Object = Object::new();

        const fn generate_room() -> Room {
            let mut room= [[Self::EMPTY; ROOM_SIZE]; ROOM_SIZE];
            let mut i = 0;
            while i < ROOM_SIZE*ROOM_SIZE {
                let x = i % ROOM_SIZE;
                let y = i / ROOM_SIZE;
                if x == 0 || y == 0 || x == ROOM_SIZE-1 || y == ROOM_SIZE-1 {
                    room[x][y] = Self::WALL;
                }
                i += 1;
            }
            room
        }
    }
    impl fmt::Display for Game {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for x in 0..ROOM_SIZE {
                for y in 0..ROOM_SIZE {
                    let display = 
                    if self.player.x.round() as usize == x  && self.player.y.round() as usize == y { 
                        self.player.properties
                    } else { 
                        self.map[x][y]
                    };
                    write!(f, "{} ", display)?;
                }
                write!(f, "\n")?;
            }
            write!(f, "{}", self.player)
        }
    }
}