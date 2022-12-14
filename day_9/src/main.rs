use std::collections::HashSet;
use std::fs;

struct Rope {
    head: Knot,
    tail: Knot,
    knots: Vec<Knot>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Knot {
                x: 0,
                y: 0,
                moved: 0,
                visited: HashSet::new(),
            },
            tail: Knot {
                x: 0,
                y: 0,
                moved: 1,
                visited: HashSet::new(),
            },
            knots: Vec::new(),
        }
    }
    fn move_head(&mut self, dir: &str, amt: i32) {
        match dir {
            "L" => {
                for _i in 0..amt {
                    self.head.x -= 1;
                    self.check_knots();
                }
            }
            "R" => {
                for _i in 0..amt {
                    self.head.x += 1;
                    self.check_knots();
                }
            }
            "U" => {
                for _i in 0..amt {
                    self.head.y += 1;
                    self.check_knots();
                }
            }
            "D" => {
                for _i in 0..amt {
                    self.head.y -= 1;
                    self.check_knots();
                }
            }
            _ => (),
        };
    }
    fn check_knots(&mut self) {
        let mut head: &Knot;
        let mut knot: &mut Knot;
        for i in 0..self.knots.len() {
            let mut coordinate: Coordinate = Coordinate { x: 0, y: 0 };
            let (left, right) = self.knots.split_at_mut(i);
            // Assign head based on current knot
            match i {
                0 => head = &self.head,
                _ => head = left.last().unwrap(),
            }
            knot = right.get_mut(0).unwrap();
            // If head and knot are on the same y value
            if head.y == knot.y {
                // Left
                if (knot.x - head.x) > 1 {
                    knot.x -= 1;
                    knot.moved += 1;
                    coordinate.x = knot.x;
                    coordinate.y = knot.y;
                    knot.visited.insert(coordinate);
                }
                // Right
                else if (head.x - knot.x) > 1 {
                    knot.x += 1;
                    knot.moved += 1;
                    coordinate.x = knot.x;
                    coordinate.y = knot.y;
                    knot.visited.insert(coordinate);
                }
            }
            // Head and knot are on the same vertical line
            else if head.x == knot.x {
                if (head.y - knot.y) > 1 {
                    knot.y += 1;
                    knot.moved += 1;
                    coordinate.x = knot.x;
                    coordinate.y = knot.y;
                    knot.visited.insert(coordinate);
                } else if (knot.y - head.y) > 1 {
                    knot.y -= 1;
                    knot.moved += 1;
                    coordinate.x = knot.x;
                    coordinate.y = knot.y;
                    knot.visited.insert(coordinate);
                }
            }
            // Head and knot are not on same x or y coordinates
            // meaning head is ahead on some diagonal
            else {
                if head.x > knot.x {
                    if head.y > knot.y && (head.x - 1 > knot.x || head.y - 1 > knot.y) {
                        knot.x += 1;
                        knot.y += 1;
                        knot.moved += 1;
                        coordinate.x = knot.x;
                        coordinate.y = knot.y;
                        knot.visited.insert(coordinate);
                    } else if knot.y > head.y && (head.x - 1 > knot.x || knot.y - 1 > head.y) {
                        knot.x += 1;
                        knot.y -= 1;
                        knot.moved += 1;
                        coordinate.x = knot.x;
                        coordinate.y = knot.y;
                        knot.visited.insert(coordinate);
                    }
                } else if head.x < knot.x {
                    if head.y > knot.y && (head.x + 1 < knot.x || head.y - 1 > knot.y) {
                        knot.x -= 1;
                        knot.y += 1;
                        knot.moved += 1;
                        coordinate.x = knot.x;
                        coordinate.y = knot.y;
                        knot.visited.insert(coordinate);
                    } else if knot.y > head.y && (head.x + 1 < knot.x || head.y + 1 < knot.y) {
                        knot.x -= 1;
                        knot.y -= 1;
                        knot.moved += 1;
                        coordinate.x = knot.x;
                        coordinate.y = knot.y;
                        knot.visited.insert(coordinate);
                    }
                }
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Knot {
    x: i32,
    y: i32,
    moved: usize,
    visited: HashSet<Coordinate>,
}

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    let mut rope: Rope = Rope::new();

    // Set number of knots with this variable
    let num_of_knots: i32 = 9;

    for _i in 0..num_of_knots {
        let mut knot: Knot = Knot {
            x: 0,
            y: 0,
            moved: 0,
            visited: HashSet::new(),
        };
        knot.visited.insert(Coordinate { x: 0, y: 0 });
        rope.knots.push(knot);
    }
    rope.tail.visited.insert(Coordinate { x: 0, y: 0 });

    for l in file.lines() {
        let words: Vec<&str> = l.split(' ').collect();
        rope.move_head(words[0], words[1].parse().unwrap());
    }

    println!("Tail moved: {} times", rope.knots.last().unwrap().moved);
    println!(
        "Tail visited: {} spaces",
        rope.knots.last().unwrap().visited.len()
    );
}
