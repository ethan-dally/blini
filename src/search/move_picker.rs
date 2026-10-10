use crate::common::r#move::{Move, MoveFlag, MoveList};

impl MoveList {
    #[inline]
    pub fn iter_ordered(self) -> impl Iterator<Item = Move> {
        MovePicker {
            list: self,
            searched: 0,
        }
    }
}

#[derive(Debug)]
struct MovePicker {
    list: MoveList,
    searched: usize,
}

impl Iterator for MovePicker {
    type Item = Move;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.list.len() == self.searched {
            return None;
        }

        let mut best = self.searched;
        self.searched += 1;

        for index in self.searched..self.list.len() {
            let index_mv = self.list.list[index];
            let best_mv = self.list.list[best];
            if index_mv.flag().flag_prio() < best_mv.flag().flag_prio() {
                best = index;
            }
        }

        let out = self.list.list[best];
        self.list.list.swap(best, self.searched - 1);
        Some(out)
    }
}

impl MoveFlag {
    #[inline]
    fn flag_prio(self) -> u8 {
        match self {
            //high
            MoveFlag::CapturePromotionQueen => 0b000,
            MoveFlag::PromotionQueen => 0b000,
            //med-high
            MoveFlag::Capture => 0b001,
            MoveFlag::EnPassant => 0b001,
            //med
            MoveFlag::CastleShort => 0b010,
            MoveFlag::CastleLong => 0b010,
            //med low
            MoveFlag::NonCapture => 0b011,
            MoveFlag::PawnDouble => 0b011,
            //low
            MoveFlag::PromotionKnight => 0b100,
            MoveFlag::CapturePromotionKnight => 0b100,
            //v-low
            MoveFlag::PromotionRook => 0b111,
            MoveFlag::PromotionBishop => 0b111,
            MoveFlag::CapturePromotionRook => 0b111,
            MoveFlag::CapturePromotionBishop => 0b111,
        }
    }
}
