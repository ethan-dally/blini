use crate::common::{bitboard::Bitboard, direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West}, file::File, rank::Rank, square::Square};
use core::time;
use std::{collections::HashMap, sync::{Arc, OnceLock, atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering}, mpsc}, thread::{self}, time::Instant};
use arrayvec::ArrayVec;
use rand::{RngExt};
use color_eyre::eyre::{OptionExt, Result, eyre};

static MAGIC_TABLE: OnceLock<MagicTable> = OnceLock::new();

const ROOK_TABLE_SIZES: [usize; 64] = [14, 12, 12, 12, 12, 12, 12, 14, 12, 11, 11, 11, 11, 11, 11, 12, 12, 11, 11, 11, 11, 11, 11, 12, 12, 11, 11, 11, 11, 11, 11, 12, 12, 11, 11, 11, 11, 11, 11, 12, 12, 11, 11, 11, 11, 11, 11, 12, 12, 11, 11, 11, 11, 11, 11, 12, 14, 12, 12, 12, 12, 12, 12, 14];
const BISHOP_TABLE_SIZES: [usize; 64] = [6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 10, 10, 7, 5, 5, 5, 5, 7, 10, 10, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6];
const MAX_KEYS: usize = 4096;

const DIAGONAL_MAGICS: [u64; 64] = [
    3508751241539745188,
    6660618467923842809,
    7934295809900510560,
    17790367612272358219,
    3644766422567327621,
    6707280270262720880,
    10955537196762955764,
    5910063008183287806,
    16203115511072760,
    16901661302039238147,
    17735144886633891288,
    3522967598452292835,
    2502343415746459023,
    10086859388888108112,
    5188305877986148166,
    14093800775000227758,
    800755020786151393,
    8799637281240803302,
    15237952360241222355,
    7648238513877574756,
    10724196674980299043,
    5834696003445920807,
    11189529544353645205,
    12214993854692253664,
    13344377339129659169,
    3016373815134847765,
    6079129446194446754,
    11693747758696641566,
    10725088672403305734,
    17366175931936293634,
    12216016228292640258,
    8900874337943395330,
    17798139392719873610,
    15419344429196022405,
    10437410599912081417,
    14401441850849146962,
    16687768237999197952,
    7879630899147898949,
    5395161759150773266,
    14730032645964857864,
    1914337224131002399,
    16751087335317670209,
    13656334889661552641,
    9138024920922682880,
    8799519720593246848,
    9366374003905694208,
    5665298447426094816,
    5332151779635679041,
    17084392558752166969,
    8479675858331174239,
    7331243379469061314,
    1344224724741795542,
    16088068235748312710,
    7568616759517589206,
    8722874447004599091,
    13997170925391417289,
    3092846821378704523,
    5907019610091998392,
    3258490002758971403,
    7537074156297291807,
    1703076294661270540,
    8390306148828838400,
    7320836027878461780,
    15068587046606832002,
];

const ORTHOGONAL_MAGICS: [u64; 64] = [
    17073962558134552494,
    17464406370266149337,
    12207399262895017617,
    13841833520409596948,
    14987817168657403601,
    293882022304507616,
    12913191513954150684,
    10911462786525081930,
    13787348034887056683,
    14588000816946956280,
    11055012544029521097,
    5425335574058008594,
    16872160280120605637,
    6542820708328195172,
    10302536450065056669,
    16439142173284872074,
    2092557354735638042,
    7674074661349009199,
    1154280199278294854,
    7218932041675923707,
    11164847345641483789,
    1476809043052200045,
    17800686622703638915,
    16785787599434870985,
    1090271221508652308,
    3457628440928829807,
    1183642185635955271,
    1614486033881815636,
    4429008965507867131,
    16372792568994735879,
    5941343621316451369,
    6977798683544194784,
    6966629880653933678,
    1685418411161095084,
    10132505566722728540,
    1897697688906723448,
    9977161891576806630,
    9677462237742096394,
    3820571368123003530,
    3762235607925084535,
    7601311821469123944,
    7855767312741865452,
    2723658596962340953,
    1625729509148850950,
    17444042236001766267,
    9438562505670296269,
    15873597948141764592,
    10504810576528519482,
    17667456968411284275,
    9805574117980399384,
    14027876435445867184,
    5317394076849275392,
    3191718402425953792,
    12179797416794656800,
    5497233636121179136,
    773795193600775508,
    13498519627132292863,
    9949313048757937342,
    15766666096412466690,
    4647714025374320530,
    2521966450923670022,
    16479829991187814427,
    994676198098138356,
    12002096410175927254,
];

pub fn magic_table() -> &'static MagicTable {
    MAGIC_TABLE.get_or_init(||{
        MagicTable::build(DIAGONAL_MAGICS, ORTHOGONAL_MAGICS)
            .expect("failed to build magic tables")
    })
}

fn get_magic_index(key: Bitboard, magic_num: u64, table_pow: usize) -> usize {
    (key.0.wrapping_mul(magic_num) >> (64 - table_pow)) as usize
}

#[derive(Debug, Default, Clone, Copy)]
struct Magic {
    offset: usize,
    table_pow: usize,
    magic_num: u64
}

pub struct MagicTable {
    orth_magics: [Magic; 64],
    diag_magics: [Magic; 64],
    orth_table: Box<[Bitboard]>,
    diag_table: Box<[Bitboard]>,
}

impl MagicTable {

    #[inline]
    const fn orth_table_size() -> usize {
        let mut total = 0;
        let mut i = 0;
        while i < 64 {
            total += 2usize.pow(ROOK_TABLE_SIZES[i] as u32);
            i += 1;
        }
        total
    }

    #[inline]
    const fn diag_table_size() -> usize {
        let mut total = 0;
        let mut i = 0;
        while i < 64 {
            total += 2usize.pow(BISHOP_TABLE_SIZES[i] as u32);
            i += 1;
        }
        total
    }

    #[inline]
    pub fn get_orth(&self, board: Bitboard, sqr: Square) -> Bitboard {
        let magic = self.orth_magics[sqr as usize];
        let key = board & ROOK_MASKS[sqr as usize];
        let index = get_magic_index(key, magic.magic_num, magic.table_pow);
        self.orth_table[magic.offset + index]
    }

    #[inline]
    pub fn get_diag(&self, board: Bitboard, sqr: Square) -> Bitboard {
        let magic = self.diag_magics[sqr as usize];
        let key = board & BISHOP_MASKS[sqr as usize];
        let index = get_magic_index(key, magic.magic_num, magic.table_pow);
        self.diag_table[magic.offset + index]
    }

    #[allow(dead_code)]
    fn size_of(&self) -> usize {
        (self.orth_table.len() + self.diag_table.len()) * size_of::<Bitboard>()
    }

    fn build(diag_vals: [u64; 64], orth_vals: [u64; 64]) -> Result<MagicTable> {

        let mut orth_table: Box<[Bitboard]> = vec![Bitboard::EMPTY; MagicTable::orth_table_size()].into();
        let mut diag_table: Box<[Bitboard]> = vec![Bitboard::EMPTY; MagicTable::diag_table_size()].into();
        let mut orth_magics = [Magic::default(); 64];
        let mut diag_magics = [Magic::default(); 64];
        let mut orth_offset: usize = 0;
        let mut diag_offset: usize = 0;

        for sqr in Square::ALL {

            /*
            orthogonal
            */
            let magic_num = orth_vals[sqr as usize];
            let table_pow = ROOK_TABLE_SIZES[sqr as usize];

            for (key, val) in gen_key_val_pairs(sqr, false) {

                let index = get_magic_index(key, magic_num, table_pow);

                if
                    orth_table[index + orth_offset] != Bitboard::EMPTY && 
                    orth_table[index + orth_offset] != val
                {
                    return Err(eyre!("incorrect magic orthogonal {sqr}"));
                }

                orth_table[index + orth_offset] = val;
                orth_magics[sqr as usize] = Magic { offset: orth_offset, table_pow, magic_num};

            }
            orth_offset += 2usize.pow(table_pow as u32);

            /*
            diagonal
            */
            let magic_num = diag_vals[sqr as usize];
            let table_pow = BISHOP_TABLE_SIZES[sqr as usize];

            for (key, val) in gen_key_val_pairs(sqr, true) {

                let index = get_magic_index(key, magic_num, table_pow);

                if
                    diag_table[index + diag_offset] != Bitboard::EMPTY && 
                    diag_table[index + diag_offset] != val
                 {
                    return Err(eyre!("incorrect magic diagonal {sqr}"));
                }

                diag_table[index + diag_offset] = val;
                diag_magics[sqr as usize] = Magic { offset: diag_offset, table_pow, magic_num};
            }
            diag_offset += 2usize.pow(table_pow as u32);

        }
        Ok(MagicTable{orth_magics, orth_table, diag_magics, diag_table})

    }
}

struct ThreadData {
    stop: AtomicBool,
    thread_best: AtomicU32,
    counted: AtomicU64
}

impl ThreadData {
    fn new() -> ThreadData {
        ThreadData { stop: false.into(), thread_best: 0.into(), counted: 0.into() }
    }
    fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Copy)]
struct FindMagic {
    sqr: Square,
    is_diagonal: bool,
    table_pow: usize,
    table_size: usize,
}

impl FindMagic {
    #[inline]
    fn get_index(&self, key: Bitboard, magic_num: u64) -> usize {
        get_magic_index(key, magic_num, self.table_pow)
    }

    fn find_magic_thread(&self, thread: Arc<ThreadData>) -> Option<u64> {
        let mut rng = rand::rng();
        let map= gen_key_val_pairs(self.sqr, self.is_diagonal);
        let mut magic_table = vec![Bitboard::EMPTY; self.table_size];
        let mut magic_num: u64;
        loop {
            if thread.stop.load(Ordering::Relaxed) {
                return None;
            }
            let mut dbg_i  = 0;
            let mut failed = false;
            magic_table.fill(Bitboard::EMPTY);
            magic_num = rng.random();
            for (key,  value) in map.iter() {
                let index = self.get_index(*key, magic_num);
                let loc = magic_table[index as usize];
                if loc == Bitboard::EMPTY || loc == *value {
                    magic_table[index as usize] = *value;
                } else {
                    let prev_best = thread.thread_best.load(Ordering::Relaxed);
                    thread.thread_best.store(prev_best.max(dbg_i), Ordering::Relaxed);
                    thread.counted.fetch_add(1, Ordering::Relaxed);
                    failed = true;
                    break;
                }
                dbg_i += 1;
            }
            if !failed {
                break;
            }
        }
        Some(magic_num)
    }

    fn find_magic(self) -> Result<u64> {
        let thread_count: usize = std::thread::available_parallelism()
            .map_err(|_|{eyre!("cant get avaliable threads")})?.into();
        let (
            send, 
            rec
        ) = mpsc::channel::<Option<u64>>();
        let mut thread_data: Vec<Arc<ThreadData>> = vec![];

        let start = Instant::now();
        let to_beat = gen_key_val_pairs(self.sqr, self.is_diagonal).iter().len();

        /*
        spawn threads
        */
        for _ in 0..thread_count.min(16) {
            let send = send.clone();
            let magic_thread = Arc::new(ThreadData::new());
            thread_data.push(magic_thread.clone());
            thread::spawn(move ||{
                let out = self.find_magic_thread(magic_thread);
                let _ = send.send(out);
            });
        }

        /*
        listen
        */
        loop {

            let mut search_count: u64 = 0;
            let mut best: u32 = 0;

            for thread in thread_data.iter() {
                search_count += thread.counted.load(Ordering::Relaxed);
                best = best.max(thread.thread_best.load(Ordering::Relaxed));
            }

            println!("threads: {thread_count}, {}, size: {}, searched {search_count}, best {best}, to_beat: {to_beat}, elapsed: {}",
                self.sqr,
                match self.is_diagonal {
                    true => "Diagonal",
                    false => "Orthogonal"
                },
                start.elapsed().as_secs()
            );

            if let Ok(res) = rec.try_recv() {
                thread_data.iter().for_each(|d|{d.stop()});
                return res.ok_or_eyre("find_magic_thread unexpectedly returned None");
            }

            thread::sleep(time::Duration::from_secs(1));

        }
    }

    fn new(sqr: Square, is_diagonal: bool) -> FindMagic {
        let table_pow = match is_diagonal {
            true => BISHOP_TABLE_SIZES[sqr as usize],
            false => ROOK_TABLE_SIZES[sqr as usize]
        };
        let table_size = 2usize.pow(table_pow as u32);
        FindMagic {sqr, is_diagonal, table_pow, table_size, }
    }

    #[allow(dead_code)]
    pub fn find_all_magics() -> Result<(Vec<u64>,Vec<u64>)> {

        let mut orth_list: Vec<u64> = vec![];
        let mut diag_list: Vec<u64> = vec![];

        for is_diagonal in [true, false] {
            for sqr in Square::ALL {

                let magic = FindMagic::new(sqr, is_diagonal);

                let Ok(magic) = magic.find_magic() else {
                    return Err(eyre!("couldnt find magic {sqr}, diagonal '{is_diagonal}'"));
                };

                match is_diagonal {
                    true => diag_list.push(magic),
                    false => orth_list.push(magic),
                }

            }
        }
        Ok((orth_list, diag_list))
    }
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

#[allow(dead_code)]
fn generate_orth_and_diag_table_size() -> Result<(Vec<u32>, Vec<u32>)> {
    let mut orth: Vec<u32> = vec![];
    let mut diag: Vec<u32> = vec![];

    for sqr in Square::ALL {
        let orth_val = gen_key_val_pairs(sqr, false).len().trailing_zeros();
        let diag_val = gen_key_val_pairs(sqr, true).len().trailing_zeros();

        orth.push(match orth_val {
            0..9 => orth_val,
            9..12 => orth_val + 1,
            12 => 14,
            _ => return Err(eyre!("unexpected key size 2^{orth_val}"))
        });

        diag.push(match diag_val {
            0..9 => diag_val,
            9..12 => diag_val + 1,
            12 => 14,
            _ => return Err(eyre!("unexpected key size 2^{orth_val}"))
        });
    }

    Ok((orth, diag))
}

#[test]
fn test_magics_with_keys() {
    let table = magic_table();
    for sqr in Square::ALL {
        for (key, val) in gen_key_val_pairs(sqr, false) {
            assert_eq!(table.get_orth(key, sqr), val);
        }
        for (key, val) in gen_key_val_pairs(sqr, true) {
            assert_eq!(table.get_diag(key, sqr), val);
        }
    }
}

#[test]
fn test_magics_with_board() {
    use crate::board::board::Board;
    let table = magic_table();
    let board = Board::parse_fen("4k3/8/4p3/3B4/8/5n2/P7/4K2P w - - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    assert_eq!(
        table.get_diag(board.all_pieces(), Square::D5),
        Bitboard(72642534561677568)
    );
}