use std::fs;

#[derive(Debug)]
struct Grid {
    trees: Vec<Vec<char>>,
    size: i32,
}

impl Grid {
    fn find_visible_and_highest_scenic_score(&self) -> (i32, i32) {
        let mut total: i32 = 0;
        let mut _score: i32 = 0;
        let mut scores: Vec<i32> = Vec::new();

        // These are used in part one
        // holds value of number of trees that tree x is taller than in a direction
        let mut left_taller_than: i32 = 0;
        let mut right_taller_than: i32 = 0;
        let mut top_taller_than: i32 = 0;
        let mut bottom_taller_than: i32 = 0;

        // Blocked variables
        // Holds value if tree is blocked in a direction
        let mut left_blocked: bool = false;
        let mut right_blocked: bool = false;
        let mut top_blocked: bool = false;
        let mut bottom_blocked: bool = false;

        // Score variables
        // used in part two
        // scenic score in a direction
        let mut left_score: i32 = 0;
        let mut right_score: i32 = 0;
        let mut top_score: i32 = 0;
        let mut bottom_score: i32 = 0;

        // Loop through rows
        for i in 0..self.size {
            // Loop through item in row
            for j in 0..self.size {
                // If tree is visible from a direction
                let mut right_visible: bool = false;
                let mut left_visible: bool = false;
                let mut top_visible: bool = false;
                let mut bottom_visible: bool = false;

                // Get current tree
                let tree = self.trees[i as usize][j as usize];

                // Edge trees are always visible
                if i == 0 || i == self.size - 1 {
                    total += 1;
                } else if j == 0 || j == self.size - 1 {
                    total += 1;
                    // Check visibility of inner trees
                } else {
                    // Left visibility
                    // Reverse iter because we need to look from the tree to its direct left
                    for t in self.trees[i as usize][0..j as usize].iter().rev() {
                        // If tree is blocked, and is already not blocked
                        if tree <= *t && !left_blocked {
                            left_blocked = true;
                            left_score += 1;
                        }
                        if tree > *t {
                            left_taller_than += 1;
                            if !left_blocked {
                                left_score += 1;
                            }
                        }
                        if left_taller_than == j {
                            left_visible = true;
                        }
                    }
                    // Right visibility
                    for t in &self.trees[i as usize][(j + 1) as usize..] {
                        if tree <= *t && !right_blocked {
                            right_blocked = true;
                            right_score += 1;
                        }
                        if tree > *t {
                            right_taller_than += 1;
                            if !right_blocked {
                                right_score += 1;
                            }
                        }
                        if right_taller_than == (self.size - 1) - j {
                            right_visible = true;
                        }
                    }
                    // Top visibility
                    for k in (0..i).rev() {
                        let tmp_tree = self.trees[k as usize][j as usize];
                        if tree <= tmp_tree && !top_blocked {
                            top_blocked = true;
                            top_score += 1;
                        }

                        if tree > tmp_tree {
                            top_taller_than += 1;
                            if !top_blocked {
                                top_score += 1;
                            }
                        }
                        if top_taller_than == i {
                            top_visible = true;
                        }
                    }
                    // Bottom visibility
                    for k in (i + 1)..self.size {
                        let tmp_tree = self.trees[k as usize][j as usize];
                        if tree <= tmp_tree && !bottom_blocked {
                            bottom_blocked = true;
                            bottom_score += 1;
                        }

                        if tree > tmp_tree {
                            bottom_taller_than += 1;
                            if !bottom_blocked {
                                bottom_score += 1;
                            }
                        }
                        if bottom_taller_than == (self.size - 1) - i {
                            bottom_visible = true;
                        }
                    }
                    // Visibility is only counted once per tree
                    if left_visible {
                        total += 1;
                    } else if right_visible {
                        total += 1;
                    } else if top_visible {
                        total += 1;
                    } else if bottom_visible {
                        total += 1;
                    }

                    // Calculate tree's scenic score
                    _score = left_score * right_score * top_score * bottom_score;
                    scores.push(_score);

                    right_taller_than = 0;
                    left_taller_than = 0;
                    top_taller_than = 0;
                    bottom_taller_than = 0;

                    left_blocked = false;
                    right_blocked = false;
                    top_blocked = false;
                    bottom_blocked = false;

                    left_score = 0;
                    right_score = 0;
                    top_score = 0;
                    bottom_score = 0;
                }
            }
        }

        // Sort scores
        scores.sort();

        (total, *scores.last().unwrap())
    }
}
fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    let mut grid: Grid = Grid {
        trees: Vec::new(),
        size: 0,
    };

    // Create grid
    for l in file.lines() {
        // Assume grid is a square
        grid.size += 1;
        grid.trees.push(l.chars().collect());
    }

    let (visible_trees, highest_scenic_score) = grid.find_visible_and_highest_scenic_score();
    println!("Total trees visible in grid: {visible_trees}\nHighest scenic score: {highest_scenic_score}");
}
