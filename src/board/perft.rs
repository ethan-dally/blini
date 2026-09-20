use crate::board::{board::Board};

impl Board {
    pub fn perft(self, depth: u8) -> u64 {
        let move_list = self.get_moves();
        if depth == 0 {
            return move_list.into_iter().len() as u64;
        }
        let mut count = 0;
        for mv in move_list {
            let mut next = self.clone();
            next.do_move(mv);
            count += next.perft(depth - 1);
        }
        return count;
    }
}

// #[test]
// fn kiwipete_1() {
//     let board = Board::parse_fen(
//         "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(1);
//     assert_eq!(perft, 48);
// }

// #[test]
// fn kiwipete_2() {
//     let board = Board::parse_fen(
//         "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(2);
//     assert_eq!(perft, 2039);
// }

#[test]
fn perft_test() {
    let board = Board::parse_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    ).expect("fen incorrect");
    board.display();
    for mv in board.get_moves() {
        mv.display();
        let mut next = board.clone();
        next.do_move(mv);
    }
    assert!(false);
}