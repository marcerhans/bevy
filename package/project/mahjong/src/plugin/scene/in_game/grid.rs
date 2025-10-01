mod tile {
    pub(super) struct Tile;
}

mod grid {
    use std::marker::PhantomData;
    use super::tile::*;

    mod generators {
        mod turtle {
            struct Turtle;
        }
    }

    struct Generator<T> {
        _type: PhantomData<T>,
        grid: Vec<Tile>,
    }

    impl<T> Generator<T> {
        pub fn new() -> Self {
            Self {
                _type: PhantomData::<T>,
                grid: Vec::<Tile>::default(),
            }
        }
    }

    // impl Generator<Turtle> {
    //     const TILES: usize = 144;
    // }

    // impl PositionGenerator for Generator<Turtle> {
    //     fn generate(
    //         &self,
    //         tile_size: Vec2,
    //         current: usize,
    //     ) -> Option<Vec2> {
    //         if current >= Self::TILES {
    //             return None;
    //         }

    //         let layer;
    //         let row;
    //         match current {
    //             ..87 => {
    //                 layer = 0;
    //                 match current {
    //                     0..12 => row = 0,
    //                     12..20 => row = 1,
    //                     20..30 => row = 2,
    //                     30..42 => row = 3,
    //                     42..54 => row = 4,
    //                     54..64 => row = 5,
    //                     64..72 => row = 6,
    //                     72..84 => row = 7,
    //                     84..87 => row = 8,
    //                     _ => unreachable!(),
    //                 };
    //             },
    //             87..123 => {
    //                 layer = 1;
    //                 row = (current - 87) / 6 + 1;
    //             },
    //             123..139 => {
    //                 layer = 2;
    //                 row = (current - 123) / 4 + 2;
    //             },
    //             139..143 => {
    //                 layer = 3;
    //                 row = (current - 139) / 2 + 3;
    //             },
    //             143 => {
    //                 // Special case. Just return value immediately.
    //                 return Some(Vec2::new(5.5, 3.5) * tile_size);
    //             },
    //             _ => return None,
    //         }

    //         let column = match layer {
    //             0 => {
    //                 match row {
    //                     0 => 0 + current - 0,
    //                     1 => 2 + current - 12,
    //                     2 => 1 + current - 20,
    //                     3 => 0 + current - 30,
    //                     4 => 0 + current - 42,
    //                     5 => 1 + current - 54,
    //                     6 => 2 + current - 64,
    //                     7 => 0 + current - 72,
    //                     8 => match current - 84 {
    //                         // Last 3 are special cases. Do not follow a pattern.
    //                         0 => return Some(Vec2::new(-1.0, 3.5) * tile_size),
    //                         1 => return Some(Vec2::new(12.0, 3.5) * tile_size),
    //                         2 => return Some(Vec2::new(13.0, 3.5) * tile_size),
    //                         _ => unreachable!(),
    //                     },
    //                     _ => unreachable!(),
    //                 }
    //             },
    //             1 => 3 + ((current - 87) % 6),
    //             2 => 4 + ((current - 123) % 4),
    //             3 => 5 + ((current - 139) % 2),
    //             _ => unreachable!(),
    //         };

    //         Some(Vec2::new(column as f32, row as f32) * tile_size)
    //     }
    // }

    // pub trait PositionGenerator {
    //     fn generate(
    //         &self,
    //         tile_size: Vec2,
    //         current: usize,
    //     ) -> Option<Vec2>;
    // }

    // pub struct Placer<G: PositionGenerator> {
    //     tile_size: Vec2,
    //     generator: G,
    // }

    // impl<G: PositionGenerator> Placer<G> {
    //     pub fn new(
    //         tile_size: Vec2,
    //         generator: G,
    //     ) -> Self {
    //         Self {
    //             tile_size,
    //             generator,
    //         }
    //     }
    // }

    // pub struct PlacerIterator<'a, G: PositionGenerator> {
    //     placer: &'a Placer<G>,
    //     counter: usize,
    // }

    // type PlaceIteratorItem = Vec2;

    // impl<'a, G: PositionGenerator> Iterator for PlacerIterator<'a, G> {
    //     type Item = PlaceIteratorItem;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         self.counter += 1;
    //         self.placer
    //             .generator
    //             .generate(self.placer.tile_size, self.counter - 1)
    //     }
    // }

    // impl<'a, G: PositionGenerator> IntoIterator for &'a Placer<G> {
    //     type Item = PlaceIteratorItem;
    //     type IntoIter = PlacerIterator<'a, G>;

    //     fn into_iter(self) -> Self::IntoIter {
    //         PlacerIterator {
    //             placer: self,
    //             counter: 0,
    //         }
    //     }
    // }
}
