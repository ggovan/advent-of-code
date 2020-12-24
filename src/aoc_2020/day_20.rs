use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::HashMap;
use std::fs::read_to_string;

pub struct Day20;

impl Aoc2020 for Day20 {
    type Input = Vec<Tile>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        20
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_20.in")?;
        Ok(input.split("\n\n").map(parse_tile).collect())
    }

    fn part_1(tiles: &Self::Input) -> Self::Result1 {
        get_corners(tiles, &get_lookup_map(tiles))
            .iter()
            .map(|t| t.id)
            .product()
    }

    fn part_2(tiles: &Self::Input) -> Self::Result2 {
        let global_dim = (tiles.len() as f64).sqrt() as usize;
        let lookup_map = get_lookup_map(tiles);
        let corners = get_corners(tiles, &get_lookup_map(tiles))
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(
            lookup_map
                .iter()
                .all(|(_k, v)| v.len() == 1 || v.len() == 2),
            true
        );
        assert_eq!(
            lookup_map.iter().filter(|(_k, v)| v.len() == 1).count(),
            4 * global_dim
        );

        // pick one of the corners
        let mut corner_piece = corners[0].clone();
        let flipped_corners = lookup_map
            .iter()
            .filter(|(_k, v)| v.len() == 1 && v.contains(&corner_piece.id))
            .map(|(k, _v)| *k)
            .collect::<Vec<_>>();
        assert_eq!(flipped_corners.len(), 2);

        // rotate it so that it a top left corner
        while !is_top_left(&corner_piece, &flipped_corners) {
            corner_piece.rotate();
        }

        let mut global_map: Vec<Tile> = Vec::new();
        global_map.push(corner_piece);

        for r in 0..global_dim {
            if r == 0 {
                // we have the top left, just continue
            } else {
                // pick the tile to match the one above
                let placed = &global_map[(r - 1) * global_dim];
                let next_edge_with_flip = placed.bottom_binary(true);
                let next_tile = lookup_map[&next_edge_with_flip]
                    .iter()
                    .find(|&id| *id != placed.id)
                    .unwrap();

                let next_edge = placed.bottom_binary(false);

                let mut next_tile = tiles.iter().find(|t| t.id == *next_tile).unwrap().clone();
                let mut found = false;
                for _ in 0..=3 {
                    if next_tile.top_binary(false) == next_edge {
                        found = true;
                        break;
                    }

                    if next_tile.top_binary(true) == next_edge_with_flip {
                        next_tile.flip_horizontal();
                        found = true;
                        break;
                    }

                    next_tile.rotate();
                }
                assert_eq!(found, true);

                global_map.push(next_tile);
            }

            for _c in 1..global_dim {
                let placed = global_map.last().unwrap();
                let next_edge_with_flip = placed.right_binary(true);

                let next_tile = lookup_map[&next_edge_with_flip]
                    .iter()
                    .find(|&id| *id != placed.id)
                    .unwrap();

                let next_edge = placed.right_binary(false);

                let mut next_tile = tiles.iter().find(|t| t.id == *next_tile).unwrap().clone();
                let mut found = false;
                for _ in 0..=3 {
                    if next_tile.left_binary(false) == next_edge {
                        found = true;
                        break;
                    }

                    if next_tile.left_binary(true) == next_edge_with_flip {
                        next_tile.flip_vertical();
                        found = true;
                        break;
                    }

                    next_tile.rotate();
                }
                assert_eq!(found, true);

                global_map.push(next_tile);
            }
        }

        // println!(
        //     "{}",
        //     global_map.iter().map(|t| t.to_string()).collect::<String>()
        // );
        let local_dim = (tiles[0].content.len() as f64).sqrt() as usize;
        let god_dim = global_dim * (local_dim - 2);
        let mut god_content = Vec::new();
        god_content.resize(god_dim * god_dim, ' ');
        for r in 0..global_dim {
            for c in 0..global_dim {
                global_map[r * global_dim + c]
                    .content
                    .iter()
                    .enumerate()
                    .for_each(|(i, v)| {
                        let l_r = i / local_dim;
                        let l_c = i - l_r * local_dim;
                        if (1..(local_dim - 1)).contains(&l_r)
                            && (1..(local_dim - 1)).contains(&l_c)
                        {
                            let n_r = l_r - 1 + r * (local_dim - 2);
                            let n_c = l_c - 1 + c * (local_dim - 2);
                            god_content[n_r * god_dim + n_c] = *v;
                        }
                    })
            }
        }

        let mut god_tile = Tile {
            id: 0,
            content: god_content,
        };
        // println!("{}", &god_tile);

        let sm = [
            18,
            god_dim,
            god_dim + 5,
            god_dim + 6,
            god_dim + 11,
            god_dim + 12,
            god_dim + 17,
            god_dim + 18,
            god_dim + 19,
            2 * god_dim + 1,
            2 * god_dim + 4,
            2 * god_dim + 7,
            2 * god_dim + 10,
            2 * god_dim + 13,
            2 * god_dim + 16,
        ];

        let mut sm_count = 0;
        'outer: for _ in 0..2 {
            for _ in 0..4 {
                for r in 0..(god_dim - 2) {
                    for c in 0..(god_dim - 20) {
                        let start = r * god_dim + c;
                        if sm.iter().all(|m| god_tile.content[start + m] == '#') {
                            sm_count += 1;
                        }
                    }
                }
                if sm_count > 0 {
                    break 'outer;
                }
                god_tile.rotate();
            }
            god_tile.flip_horizontal();
        }

        let hashes = god_tile.content.iter().filter(|&v| *v == '#').count();

        (hashes - sm_count * 15) as u64
    }
}

fn is_top_left(tile: &Tile, edges: &[u16]) -> bool {
    let top = tile.top_binary(false);
    let left = tile.left_binary(false);
    (edges.contains(&top) || edges.contains(&flip_bits(top)))
        && (edges.contains(&left) || edges.contains(&flip_bits(left)))
}

fn get_corners(tiles: &[Tile], lookup_map: &HashMap<u16, Vec<u64>>) -> Vec<Tile> {
    tiles
        .iter()
        .filter(|t| {
            // A corner will have exactly 2 unmatched edges
            lookup_map
                .iter()
                .filter(|(_, v)| v.len() == 1 && v.contains(&t.id))
                .count()
                > 1
        })
        .cloned()
        .collect::<Vec<_>>()
}

fn get_lookup_map(tiles: &[Tile]) -> HashMap<u16, Vec<u64>> {
    let mut map = HashMap::new();

    for t in tiles {
        for e in &t.get_edges(true) {
            let entry = map.entry(*e).or_insert_with(Vec::new);
            entry.push(t.id);
        }
    }

    map
}

#[derive(Clone)]
pub struct Tile {
    id: u64,
    content: Vec<char>,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dim = (self.content.len() as f64).sqrt() as usize;
        let content = (0..dim)
            .map(|r| {
                let mut row = self
                    .content
                    .iter()
                    .skip(r * dim)
                    .take(dim)
                    .collect::<String>();
                row.push('\n');
                row
            })
            .collect::<String>();
        write!(f, "Tile {}:\n{}", self.id, content)
    }
}

fn flip_bits(b: u16) -> u16 {
    let mut f_b = 0;
    for i in 0..10 {
        let bit = b & (1 << i);
        let bit = bit >> i;
        f_b |= bit << (9 - i)
    }
    f_b
}

impl Tile {
    fn get_edges(&self, include_flipped: bool) -> [u16; 4] {
        [
            self.top_binary(include_flipped),
            self.bottom_binary(include_flipped),
            self.left_binary(include_flipped),
            self.right_binary(include_flipped),
        ]
    }

    fn top_binary(&self, include_flipped: bool) -> u16 {
        let b = self
            .content
            .iter()
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        if include_flipped {
            let f_b = flip_bits(b);
            std::cmp::max(b, f_b)
        } else {
            b
        }
    }

    fn bottom_binary(&self, include_flipped: bool) -> u16 {
        let b = self
            .content
            .iter()
            .skip(90)
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        if include_flipped {
            let f_b = flip_bits(b);
            std::cmp::max(b, f_b)
        } else {
            b
        }
    }

    fn left_binary(&self, include_flipped: bool) -> u16 {
        let b = self
            .content
            .iter()
            .step_by(10)
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        if include_flipped {
            let f_b = flip_bits(b);
            std::cmp::max(b, f_b)
        } else {
            b
        }
    }

    fn right_binary(&self, include_flipped: bool) -> u16 {
        let b = self
            .content
            .iter()
            .skip(9)
            .step_by(10)
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        if include_flipped {
            let f_b = flip_bits(b);
            std::cmp::max(b, f_b)
        } else {
            b
        }
    }

    fn rotate(&mut self) {
        // dbg!("Rotating");
        let mut new_vec = Vec::with_capacity(self.content.len());
        new_vec.resize(self.content.len(), ' ');
        let dim = (self.content.len() as f64).sqrt() as usize;
        for (i, v) in self.content.iter().enumerate() {
            let r = i / dim;
            let c = i - (r * dim);
            let n_r = c;
            let n_c = dim - r - 1;
            new_vec[n_r * dim + n_c] = *v;
        }
        self.content = new_vec;
    }

    fn flip_vertical(&mut self) {
        // dbg!("Flipping");
        let mut new_vec = Vec::with_capacity(self.content.len());
        new_vec.resize(self.content.len(), ' ');
        let dim = (self.content.len() as f64).sqrt() as usize;
        for (i, v) in self.content.iter().enumerate() {
            let r = i / dim;
            let c = i - (r * dim);
            let n_r = dim - r - 1;
            let n_c = c;
            new_vec[n_r * dim + n_c] = *v;
        }
        self.content = new_vec;
    }

    fn flip_horizontal(&mut self) {
        // dbg!("Flipping");
        let mut new_vec = Vec::with_capacity(self.content.len());
        new_vec.resize(self.content.len(), ' ');
        let dim = (self.content.len() as f64).sqrt() as usize;
        for (i, v) in self.content.iter().enumerate() {
            let r = i / dim;
            let c = i - (r * dim);
            let n_r = r;
            let n_c = dim - c - 1;
            new_vec[n_r * dim + n_c] = *v;
        }
        self.content = new_vec;
    }
}

fn parse_tile(s: &str) -> Tile {
    let id = s
        .lines()
        .next()
        .unwrap()
        .chars()
        .skip(5)
        .take(4)
        .collect::<String>()
        .parse()
        .unwrap();
    let content = s
        .lines()
        .skip(1)
        .flat_map(|l| l.chars())
        .filter(|c| *c != '\n')
        .collect();

    Tile { id, content }
}
