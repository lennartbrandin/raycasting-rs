use std::fmt;

type Display = char;
const FIELD_SIZE: usize = 10;

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

type Room = [[Object; FIELD_SIZE]; FIELD_SIZE];

const EMPTY: Object = Object { display: ' ', transparent: true, traversable: true };
const WALL: Object = Object::new();


struct Entity {
    properties: Object,
    x: f32,
    y: f32,
    angle: u16,
    fov: u16,
}
impl fmt::Display for Entity{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} at ({}/{}): {}°, fov: {}", self.properties.display, self.x, self.y, self.angle, self.fov)
    }
}

struct Game {
    player: Entity,
    map: Room,
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..FIELD_SIZE {
            for y in 0..FIELD_SIZE {
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


const fn generate_room() -> Room {
    let mut room= [[EMPTY; FIELD_SIZE]; FIELD_SIZE];
    let mut i = 0;
    while i < FIELD_SIZE*FIELD_SIZE {
        let x = i % FIELD_SIZE;
        let y = i / FIELD_SIZE;
        if x == 0 || y == 0 || x == FIELD_SIZE-1 || y == FIELD_SIZE-1 {
            room[x][y] = WALL;
        }
        i += 1;
    }
    room
}

const fn generate_game() -> Game {
    Game {
        player: Entity { 
            properties: Object { display: 'P', transparent: false, traversable: false },
            x: (FIELD_SIZE/2) as f32,
            y: (FIELD_SIZE/2) as f32,
            angle: 0,
            fov: 60,
        },
        map: generate_room(),
    }
}


fn main() {
    const GAME: Game = generate_game();
    print!("{}", GAME);
}
