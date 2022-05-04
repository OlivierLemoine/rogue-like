mod terrain;

use crate::AppState;
use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops::{Index, IndexMut, SubAssign};
pub use terrain::*;

const VERTICAL_SIZE: usize = 16;

#[derive(Clone, Copy)]
enum Possibility {
    Air,
    Terrain,
    Door,
    Monster,
    Torch,
    Chest,
    FlyingMonster,
    AirPath,
}

impl Distribution<Possibility> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Possibility {
        match rng.gen_range(0..=7) {
            1 => Possibility::Terrain,
            2 => Possibility::Door,
            3 => Possibility::Monster,
            4 => Possibility::Torch,
            5 => Possibility::Chest,
            6 => Possibility::FlyingMonster,
            _ => Possibility::Air,
        }
    }
}

impl TryFrom<Tile> for Possibility {
    type Error = ();
    fn try_from(item: Tile) -> Result<Self, ()> {
        match item.raw {
            0b0000_0001 => Ok(Possibility::Air),
            0b0000_0010 => Ok(Possibility::Terrain),
            0b0000_0100 => Ok(Possibility::Monster),
            0b0000_1000 => Ok(Possibility::Torch),
            0b0001_0000 => Ok(Possibility::Door),
            0b0010_0000 => Ok(Possibility::Chest),
            0b0100_0000 => Ok(Possibility::FlyingMonster),
            0b1000_0000 => Ok(Possibility::AirPath),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Tile {
    raw: u32, //Vec<f32>,
}

impl PartialEq<Possibility> for Tile {
    fn eq(&self, other: &Possibility) -> bool {
        self.raw & Tile::from(*other).raw != 0
    }
}

impl SubAssign<Possibility> for Tile {
    fn sub_assign(&mut self, rhs: Possibility) {
        self.remove(rhs);
        assert!(self.raw != 0);
    }
}

impl Default for Tile {
    ///! Verifier la taille de l'enum
    fn default() -> Self {
        Tile { raw: 0b1111_1111 }
    }
}

impl From<Possibility> for Tile {
    fn from(item: Possibility) -> Self {
        Tile {
            raw: 1 << item as u8,
        }
    }
}

impl Tile {
    pub fn set(&mut self, value: Possibility) {
        assert!(*self == value);
        self.raw = 1 << value as u8;
    }

    pub unsafe fn overwrite(&mut self, value: Possibility) {
        self.raw = 1 << value as u8;
    }

    pub fn remove(&mut self, value: Possibility) {
        self.raw &= !(1 << value as u8);
    }

    pub fn collapsed(&self) -> bool {
        self.raw.count_ones() == 1
    }
}

// Rules:
//              | Air | Terrain | Door | Mob | Flying Mob | Chest | Torch |
// Air          |  x  |    x    |  ↔️   |  x  |     ↧      |   x   |   x   |
// Terrain      |  x  |    x    |  ↕   |  ⬆  |            |   ⬆   |  ⬆    |
// Door         |  ↔️  |    ↕️    |      |     |            |   x   |       |
// Mob          |     |    ↕    |      |  x  |            |       |       |
// Flying Mob   |  x  |    x    |      |     |            |       |       |
// Chest        |  x  |    ⩣    |  x   |     |     x      |       |   x   |
// Torch        |  x  |    x    |      |  x  |     x      |   x   |       |
//todo ∃ => random

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos(usize, usize);

impl Index<Pos> for Vec<[Tile; VERTICAL_SIZE]> {
    type Output = Tile;

    fn index(&self, Pos(x, y): Pos) -> &Self::Output {
        &self[x][y]
    }
}
impl IndexMut<Pos> for Vec<[Tile; VERTICAL_SIZE]> {
    fn index_mut(&mut self, Pos(x, y): Pos) -> &mut Self::Output {
        &mut self[x][y]
    }
}

impl Index<Pos> for Dungeon {
    type Output = Tile;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.content[pos]
    }
}
impl IndexMut<Pos> for Dungeon {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.content[pos]
    }
}

#[derive(Component)]
pub struct Dungeon {
    terrain_atlas: Handle<TextureAtlas>,
    has_generate: bool,
    content: Vec<[Tile; VERTICAL_SIZE]>,
}
impl Dungeon {
    pub fn new(terrain_atlas: Handle<TextureAtlas>) -> Self {
        Dungeon {
            terrain_atlas,
            has_generate: false,
            content: vec![],
        }
    }

    pub fn system_set() -> SystemSet {
        SystemSet::on_update(AppState::RunningGame).with_system(Dungeon::generate)
    }

    fn relative_pos(&mut self, mut pos: Pos, right: i32, above: i32) -> Option<Pos> {
        match right.signum() {
            1 if (self.content.len() - pos.0 - 1) as i32 <= right => {
                pos.0 += right.abs() as usize;
            }
            0 => {}
            -1 if pos.0 as i32 >= right.abs() => {
                pos.0 -= right.abs() as usize;
            }
            _ => return None,
        }

        match above.signum() {
            1 if (VERTICAL_SIZE - pos.1 - 1) as i32 <= right => {
                pos.1 += right.abs() as usize;
            }
            0 => {}
            -1 if pos.1 as i32 >= right.abs() => {
                pos.1 -= right.abs() as usize;
            }
            _ => return None,
        }

        Some(pos)
    }

    fn recurse_reduce_possibilities(&mut self, this: Pos) {
        let above = self.relative_pos(this, 0, 1);
        let below = self.relative_pos(this, 0, -1);
        let left = self.relative_pos(this, -1, 0);
        let right = self.relative_pos(this, 1, 0);

        if let Some(below) = below {
            if self[below] != Possibility::Terrain {
                self[this] -= Possibility::Chest;
                self[this] -= Possibility::Monster;
                self[this] -= Possibility::Door;
            }
        } else {
            self[this] -= Possibility::Chest;
            self[this] -= Possibility::Monster;
            self[this] -= Possibility::Door;
        }

        if let Some(above) = above {
            if self[this] != Possibility::Terrain {
                let before = self[above];
                self[above] -= Possibility::Chest;
                self[above] -= Possibility::Monster;
                self[above] -= Possibility::Door;
                if before != self[above] {
                    self.recurse_reduce_possibilities(above);
                }
            }
        } else {
        }

        if self[this] == Possibility::Air {
            match (above, below) {
                (Some(x), _) | (_, Some(x)) if self[x] == Possibility::Air => {}
                _ => self[this] -= Possibility::Air,
            }
        }

        if self[this] != Possibility::Air {
            if let Some(above) = above {
                let before = self[above];
                self[above] -= Possibility::FlyingMonster;
                if before != self[above] {
                    self.recurse_reduce_possibilities(above);
                }
            }
        }

        if self[this].collapsed() {
            match self[this].try_into().unwrap() {
                Possibility::AirPath => {}
                Possibility::Air => {}
                Possibility::Terrain => {}
                Possibility::Door => {
                    above.map(|above| {
                        self.content[above].set(Possibility::Terrain);
                        self.recurse_reduce_possibilities(above);
                    });
                    below.map(|below| {
                        self.content[below].set(Possibility::Terrain);
                        self.recurse_reduce_possibilities(below);
                    });
                }
                Possibility::Monster => {
                    below.map(|below| {
                        if self.content[below] != Possibility::Terrain {
                            unimplemented!("Should always have a possibility")
                        }
                        self.content[below].set(Possibility::Air);
                        self.recurse_reduce_possibilities(below);
                    });
                }
                Possibility::Chest => {
                    below.map(|below| {
                        if self.content[below] != Possibility::Terrain {
                            unimplemented!("Should always have a possibility")
                        }
                        self.content[below].set(Possibility::Air);
                        self.recurse_reduce_possibilities(below);
                    });
                    [Possibility::Monster, Possibility::Chest]
                        .into_iter()
                        .for_each(|p| {
                            [above, below, left, right].into_iter().for_each(|z| {
                                if let Some(z) = z {
                                    self.content[z] -= p;
                                    self.recurse_reduce_possibilities(z);
                                }
                            });
                        });
                }
                Possibility::Torch => {
                    [Possibility::Torch, Possibility::Door]
                        .into_iter()
                        .for_each(|p| {
                            [left, right, above, below].into_iter().for_each(|z| {
                                z.map(|z| {
                                    self.content[z] -= p;
                                    self.recurse_reduce_possibilities(z);
                                });
                            });
                        });
                }
                Possibility::FlyingMonster => {}
            }
        }
    }

    fn collapse(&mut self, extends: usize) {
        let start_new_content = self.content.len();
        let content_len = if start_new_content == 0 {
            5
        } else {
            start_new_content
        } + extends;

        self.content
            .resize(content_len, [Tile::default(); VERTICAL_SIZE]);

        if start_new_content == 0 {
            for y in 0..VERTICAL_SIZE {
                self.content[0][y].set(Possibility::Terrain);
            }
            for x in 1..3 {
                self.content[x][6].set(Possibility::Terrain);
                self.content[x][7].set(Possibility::Air);
                self.content[x][8].set(Possibility::Air);

                self.recurse_reduce_possibilities(Pos(x, 6));
                self.recurse_reduce_possibilities(Pos(x, 7));
                self.recurse_reduce_possibilities(Pos(x, 8));
            }
        }

        for x in start_new_content..content_len {
            self.content[x][0].set(Possibility::Terrain);
            self.content[x][15].set(Possibility::Terrain);

            self.recurse_reduce_possibilities(Pos(x, 0));
            self.recurse_reduce_possibilities(Pos(x, 15));
        }

        // let mut rng = rand::thread_rng();
        // loop {
        //     self.content[rng.gen_range(new_content + 1..self.content.len())][rng.gen_range(0..VERTICAL_SIZE)]
        //         .set(rng.gen());
        // }
    }

    pub fn generate(mut commands: Commands, mut query: Query<&mut Dungeon>) {
        if query.is_empty() {
            return;
        }

        let mut dungeon = query.single_mut();
        if dungeon.has_generate {
            return;
        }
        dungeon.has_generate = true;
        dungeon.collapse(30);
        dungeon.content.iter().enumerate().for_each(|(x, column)| {
            column
                .into_iter()
                .filter_map(|&tile| Possibility::try_from(tile).ok())
                .enumerate()
                .for_each(|(y, tile)| match tile {
                    Possibility::AirPath => (),
                    Possibility::Air => (),
                    Possibility::Terrain => {
                        commands.spawn_bundle(TerrainTileBundle::new(
                            dungeon.terrain_atlas.clone(),
                            27,
                            Vec3::new(x as f32 * 16., y as f32 * 16., 0.),
                        ));
                    }
                    Possibility::Door => (),
                    Possibility::Monster => (),
                    Possibility::Torch => (),
                    Possibility::Chest => (),
                    Possibility::FlyingMonster => (),
                });
        });
        commands.spawn_bundle(TerrainTileBundle::new(
            dungeon.terrain_atlas.clone(),
            1,
            Vec3::new(0., 0., 0.),
        ));
    }
}
