use std::{collections::HashMap, sync::{atomic::{AtomicBool, AtomicU64}, mpsc}, thread::{self, JoinHandle}};
use crate::common::{bitboard::Bitboard, direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West}, file::File, rank::Rank, square::Square};
use arrayvec::ArrayVec;
use rand::{RngExt};

const MAX_KEYS: usize = 4096; //powerset size of 12, smaller for bishop but no engine runtime cost
const MAGIC_TABLE_POW: u32 = 12;
const MAGIC_TABLE_SIZE: usize = 2_usize.pow(MAGIC_TABLE_POW);

struct Magic {
    sqr: Square,
    table: [Bitboard; MAGIC_TABLE_SIZE],
    num: u64
}

impl Magic {
    #[inline]
    fn get_index(key: Bitboard, magic_num: u64) -> usize {
        (key.0.wrapping_mul(magic_num) >> (64 - MAGIC_TABLE_POW)) as usize
    }

    #[inline]
    fn get(&self, key: Bitboard) -> Bitboard {
        self.table[Magic::get_index(key, self.num)]
    }

    pub fn find_magic(sqr: Square, is_diagonal: bool) -> Magic {
        let mut rng = rand::rng();
        let map= gen_key_val_pairs(sqr, is_diagonal);
        let mut magic_table:[Bitboard; MAGIC_TABLE_SIZE];
        let mut magic_num: u64;
        let mut best = 0;
        loop {
            let mut dbg_i  = 0;
            let mut failed = false;
            magic_table = [Bitboard::EMPTY; MAGIC_TABLE_SIZE];
            magic_num = rng.random();
            for (key,  value) in map.iter() {
                let index = Magic::get_index(*key, magic_num);
                let loc = magic_table[index as usize];
                if loc == Bitboard::EMPTY || loc == *value {
                    magic_table[index as usize] = *value;
                } else {
                    best = best.max(dbg_i);
                    failed = true;
                    break;
                }
                dbg_i += 1;
            }
            if !failed {
                break;
            }
        }
        Magic { sqr, table: magic_table, num: magic_num }
    }

    fn dbg_find_magic(sqr: Square, is_diagonal: bool) -> Result<Magic, String> {
        let thread_count = std::thread::available_parallelism()
            .map_err(|e|{e.to_string()})?;
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let (send, rec) = mpsc::channel::<Magic>();
        for _ in 0..thread_count.into() {
            let send = send.clone();
            let handle = thread::spawn(move ||{
                let out = Magic::find_magic(sqr, is_diagonal);
                send.send(out);
            });
            handles.push(handle);
        }
        
        println!("using {thread_count} threads");
        todo!()
    }
}

/*
these masks GENERATE the key's for the magic
*/
const ROOK_MASKS: [Bitboard; 64] = {
    let cutout_1 = 0xFFFFFFFFFFFFFF00u64;
    let cutout_8 = 0x00FFFFFFFFFFFFFFu64;
    let cutout_a = 0xFEFEFEFEFEFEFEFEu64;
    let cutout_h = 0x7F7F7F7F7F7F7F7Fu64;
    let mut masks = [Bitboard::EMPTY; 64];
    let mut index = 0;
    while index < 64 {
        let sqr = Square::try_index(index).expect("unreachable");
        let file_bb = sqr.file().to_bb().0;
        let rank_bb = sqr.rank().to_bb().0;
        let mut mask = file_bb ^ rank_bb;
        if sqr.file() as u8 != File::A as u8 {mask &= cutout_a}
        if sqr.file() as u8 != File::H as u8 {mask &= cutout_h}
        if sqr.rank() as u8 != Rank::One as u8 {mask &= cutout_1}
        if sqr.rank() as u8 != Rank::Eight as u8 {mask &= cutout_8}
        masks[index as usize] = Bitboard(mask);
        index += 1;
    }
    masks
};

struct MagicThread {
    stop: AtomicBool,
    counted: AtomicU64
}

const BISHOP_MASKS: [Bitboard; 64] = {
    let cutout = 0x007E7E7E7E7E7E00u64;
    let mut masks = [Bitboard::EMPTY; 64];
    let mut index = 0;
    while index < 64 {
        let sqr = Square::try_index(index).expect("unreachable");
        let diag = gen_diag_mask(sqr).0 & !sqr.to_bb().0 & cutout;
        masks[index as usize] = Bitboard(diag);
        index += 1;
    }
    masks
};

const fn gen_diag_mask(sqr: Square) -> Bitboard {
    //slow, intended for only const use
    let mut bb = sqr.to_bb().0;
    let mut ne= 1;
    let mut se= 1;
    let mut sw= 1;
    let mut nw= 1;
    loop {
        let next_ne = sqr.shift::<NorthEast>(ne);
        let next_se = sqr.shift::<SouthEast>(se);
        let next_sw = sqr.shift::<SouthWest>(sw);
        let next_nw = sqr.shift::<NorthWest>(nw);
        if let Some(next) = next_ne { bb |= next.to_bb().0; }
        if let Some(next) = next_se { bb |= next.to_bb().0; }
        if let Some(next) = next_sw { bb |= next.to_bb().0; }
        if let Some(next) = next_nw { bb |= next.to_bb().0; }
        ne += 1;
        se += 1;
        sw += 1;
        nw += 1;
        if next_ne.is_none() & next_se.is_none() & next_sw.is_none() & next_nw.is_none() {
            break;
        }
    }
    Bitboard(bb)
}

fn gen_rook_magic_keys(sqr: Square) -> ArrayVec<Bitboard, MAX_KEYS> {
    let mut keys: ArrayVec<Bitboard, MAX_KEYS> = ArrayVec::new();
    let full_mask = ROOK_MASKS[sqr as usize].0;
    let mut sub_mask: u64 = 0;
    loop {
        keys.push(Bitboard(sub_mask));
        sub_mask = (sub_mask.wrapping_sub(full_mask)) & full_mask;
        if sub_mask == 0 {
            break;
        }
    }
    keys
}

fn gen_bishop_magic_keys(sqr: Square) -> ArrayVec<Bitboard, MAX_KEYS> {
    let mut keys: ArrayVec<Bitboard, MAX_KEYS> = ArrayVec::new();
    let full_mask = BISHOP_MASKS[sqr as usize].0;
    let mut sub_mask: u64 = 0;
    loop {
        keys.push(Bitboard(sub_mask));
        sub_mask = (sub_mask.wrapping_sub(full_mask)) & full_mask;
        if sub_mask == 0 {
            break;
        }
    }
    keys
}

fn gen_key_val_pairs(sqr: Square, is_diagonal: bool) -> HashMap<Bitboard, Bitboard> {
    /*
    - suuper inefficient btw but it dosent matter
    */
    let dirs: [fn(Square, u8) -> Option<Square>; 4] = match is_diagonal {
        true => [
            Square::shift::<NorthEast>,
            Square::shift::<SouthEast>,
            Square::shift::<SouthWest>,
            Square::shift::<NorthWest>,
        ],
        false => [
            Square::shift::<North>,
            Square::shift::<East>,
            Square::shift::<South>,
            Square::shift::<West>,
        ]
    };
    let magic_keys = match is_diagonal {
        true => gen_bishop_magic_keys(sqr),
        false => gen_rook_magic_keys(sqr)
    };

    let mut map: HashMap<Bitboard, Bitboard> = HashMap::new();
    for key in magic_keys {
        let mut value = Bitboard::EMPTY;
        for shift in dirs {
            let mut amt = 1;
            loop {
                let Some(dest) = shift(sqr, amt) else {
                    break;
                };
                value |= dest.to_bb();
                if key.has(dest) {
                    break;
                }
                amt += 1;
            }
        }
        map.insert(key, value);
    }
    map
}

#[test]
fn test_magics_1() {
    let magic = Magic::find_magic(Square::B2, false);
    let map = gen_key_val_pairs(Square::B2, false);
    for (key, val) in map{
        assert_eq!(magic.get(key), val);
    }
}

#[test]
fn test_magics_2() {
    let magic = Magic::find_magic(Square::H4, true);
    let map = gen_key_val_pairs(Square::H4, true);
    for (key, val) in map{
        assert_eq!(magic.get(key), val);
    }
}