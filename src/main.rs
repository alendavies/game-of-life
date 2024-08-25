use std::{fmt::Display, thread::sleep, time};

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Dead => write!(f, "⬜️"),
            CellState::Alive => write!(f, "⬛️"),
        }?;
        Ok(())
    }
}

struct Game {
    state: Vec<Vec<CellState>>,
}

impl Game {
    pub fn new(seed: Vec<Vec<CellState>>) -> Self {
        Game { state: seed }
    }

    fn print(&self) {
        for row in &self.state {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    fn evolve(&self) -> Vec<Vec<CellState>> {
        let mut new = self.state.clone();
        let rows = self.state.len();
        let cols = self.state[0].len();

        for i in 0..rows {
            for j in 0..cols {
                let mut vecinos_vivos = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                            if self.state[ni as usize][nj as usize] == CellState::Alive {
                                vecinos_vivos += 1;
                            }
                        }
                    }
                }
                if self.state[i][j] == CellState::Alive {
                    if vecinos_vivos < 2 || vecinos_vivos > 3 {
                        new[i][j] = CellState::Dead;
                    }
                } else if vecinos_vivos == 3 {
                    new[i][j] = CellState::Alive;
                }
            }
        }
        new
    }
}

fn main() {
    let seed = vec![
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Alive,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Dead,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Alive,
            CellState::Alive,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
        vec![
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
            CellState::Dead,
        ],
    ];

    let mut game = Game::new(seed);

    loop {
        std::process::Command::new("clear").status().unwrap();
        game.print();
        game.state = game.evolve();
        sleep(time::Duration::from_millis(1000));
    }
}
