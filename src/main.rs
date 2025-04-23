use std::fmt;

type Display = char;
const FIELD_SIZE: usize = 10;

#[derive(Clone, Copy)]
struct Object {
    display: Display, // Map representation
    transparent: bool, // See through
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
    fov: u16,
}

struct Game {
    player: Entity,
    map: Room,
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

fn print_room() {
    for x in 0..FIELD_SIZE {
        for y in 0..FIELD_SIZE {
            print!("{} ", ROOM[x][y]);
        }
        print!("\n")
    }
}

const ROOM: Room = generate_room();

fn main() {
    print_room();
}
