mod terrain;
use std::ops::SubAssign;

use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
pub use terrain::*;

use crate::AppState;

#[derive(Clone, Copy)]
enum Possibility {
    Air,
    Terrain,
    Door,
    Monster,
    Torch,
    Chest,
    FlyingMonster,
}

impl Distribution<Possibility> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Possibility {
        match rng.gen_range(0..5) {
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
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    raw: u64,
}

impl PartialEq<Possibility> for Tile {
    fn eq(&self, other: &Possibility) -> bool {
        self.raw & Tile::from(*other).raw != 0
    }
}

impl SubAssign<Possibility> for Tile {
    fn sub_assign(&mut self, rhs: Possibility) {
        self.remove(rhs);
    }
}

impl Default for Tile {
    ///! Verifier la taille de l'enum
    fn default() -> Self {
        Tile { raw: 0b111111 }
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

    pub fn overwrite(&mut self, value: Possibility) {
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
// Air          |  x  |    x    |      |  x  |     ↧      |   x   |   x   |
// Terrain      |  x  |    x    |  ↕   |  ⬆  |            |   ⬆   |  ⬆    |
// Door         |  ↔️  |    ↕️    |      |     |            |       |       |
// Mob          |     |    ↕    |      |  x  |            |       |       |
// Flying Mob   |  x  |    x    |      |     |            |       |       |
// Chest        |     |    ⩣    |      |     |     x      |       |   x   |
// Torch        |  x  |    x    |      |  x  |     x      |   x   |       |
//todo ∃ => random
#[derive(Component)]
pub struct Dungeon {
    terrain_atlas: Handle<TextureAtlas>,
    has_generate: bool,
    content: Vec<[Tile; 16]>,
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

    fn reduce_possibilites(&mut self, (x, y): (usize, usize)) {
        let tile = self.content[x][y];
        let above = self.content[x].get(y + 1);
        let below = self.content[x].get(y - 1);
        let left = self.content.get(x - 1).map(|v| &v[y]);
        let right = self.content.get(x + 1).map(|v| &v[y]);

        if tile.collapsed() {
            match tile.try_into().unwrap() {
                Possibility::Air => {
                    //
                    // match (
                    //     above.map(|v| *v == Possibility::Air),
                    //     below.map(|v| *v == Possibility::Air),
                    // ) {
                    //     (Some(true), Some(true)) => {}

                    // }
                }
                Possibility::Terrain => {}
                Possibility::Door => {
                    *above = Possibility::Terrain;
                    *below = Possibility::Terrain;
                }
                Possibility::Monster => {
                    if self.content[x][y - 1] != Possibility::Terrain {
                        unimplemented!("Should always have a possibility")
                    }
                    self.content[x][y - 1].set(Possibility::Air);
                    self.reduce_possibilites((x, y - 1));
                }
                Possibility::Chest => {
                    if self.content[x][y - 1] != Possibility::Terrain {
                        unimplemented!("Should always have a possibility")
                    }
                    self.content[x][y - 1].set(Possibility::Air);
                    self.reduce_possibilites((x, y - 1));
                }
                Possibility::Torch => {}
                Possibility::FlyingMonster => {}
            }
        } else {
        }
    }

    fn collapse(&mut self, extends: usize) {
        let start_new_content = self.content.len();
        let content_len = if start_new_content == 0 {
            5
        } else {
            start_new_content
        } + extends;

        self.content.resize(content_len, [Tile::default(); 16]);

        if start_new_content == 0 {
            for y in 0..16 {
                self.content[0][y].set(Possibility::Terrain);
            }
            for x in 1..3 {
                self.content[x][6].set(Possibility::Terrain);
                self.content[x][7].set(Possibility::Air);
                self.content[x][8].set(Possibility::Air);

                self.reduce_possibilites((x, 6));
                self.reduce_possibilites((x, 7));
                self.reduce_possibilites((x, 8));
            }
        }

        for x in start_new_content..content_len {
            self.content[x][0].set(Possibility::Terrain);
            self.content[x][15].set(Possibility::Terrain);

            self.reduce_possibilites((x, 0));
            self.reduce_possibilites((x, 15));
        }

        // let mut rng = rand::thread_rng();
        // loop {
        //     self.content[rng.gen_range(new_content + 1..self.content.len())][rng.gen_range(0..16)]
        //         .set(rng.gen());
        // }
    }

    pub fn generate(mut commands: Commands, mut query: Query<&mut Dungeon>) {
        if query.is_empty() {
            return;
        }

        let mut terrain = query.single_mut();
        if terrain.has_generate {
            return;
        }
        terrain.has_generate = true;

        commands.spawn_bundle(TerrainTileBundle::new(
            terrain.terrain_atlas.clone(),
            1,
            Vec3::new(0., 0., 0.),
        ));
    }
}
