pub trait Direction {
    type Opposite: Direction;
    const DX: i8;
    const DY: i8;
}

pub struct North;
pub struct South;
pub struct East;
pub struct West;

pub struct NorthEast;
pub struct NorthWest;
pub struct SouthEast;
pub struct SouthWest;

impl Direction for North {
    type Opposite = South;
    const DX: i8 = 0;
    const DY: i8 = 1;
}

impl Direction for South {
    type Opposite = North;
    const DX: i8 = 0;
    const DY: i8 = -1;
}

impl Direction for East {
    type Opposite = West;
    const DX: i8 = 1;
    const DY: i8 = 0;
}

impl Direction for West {
    type Opposite = East;
    const DX: i8 = -1;
    const DY: i8 = 0;
}

impl Direction for NorthEast {
    type Opposite = SouthWest;
    const DX: i8 = 1;
    const DY: i8 = 1;
}

impl Direction for NorthWest {
    type Opposite = SouthEast;
    const DX: i8 = -1;
    const DY: i8 = 1;
}

impl Direction for SouthEast {
    type Opposite = NorthWest;
    const DX: i8 = 1;
    const DY: i8 = -1;
}

impl Direction for SouthWest {
    type Opposite = NorthEast;
    const DX: i8 = -1;
    const DY: i8 = -1;
}

#[inline]
pub const fn shift_mask<D: Direction>(amt: u8) -> u64 {
    let horizontal = match D::DX.is_positive() {
        true => 0xFFu8 >> (D::DX as u8 * amt),
        false => 0xFFu8 << (D::DX.unsigned_abs() * amt),
    };
    let horizontal = horizontal as u64 * 0x0101010101010101;

    match D::DY.is_positive() {
        true => horizontal >> (D::DY as u8 * amt * 8),
        false => horizontal << (D::DY.unsigned_abs() * amt * 8),
    }
}

#[test]
fn directions() {
    assert_eq!(shift_mask::<North>(1), 0x00FFFFFFFFFFFFFFu64);
    assert_eq!(shift_mask::<North>(2), 0x0000FFFFFFFFFFFFu64);
    assert_eq!(shift_mask::<South>(1), 0xFFFFFFFFFFFFFF00u64);
    assert_eq!(shift_mask::<South>(2), 0xFFFFFFFFFFFF0000u64);
    assert_eq!(shift_mask::<West>(1), 0xFEFEFEFEFEFEFEFEu64);
    assert_eq!(shift_mask::<West>(2), 0xFCFCFCFCFCFCFCFCu64);
    assert_eq!(shift_mask::<East>(1), 0x7F7F7F7F7F7F7F7Fu64);
    assert_eq!(shift_mask::<East>(2), 0x3F3F3F3F3F3F3F3Fu64);

    assert_eq!(shift_mask::<NorthEast>(1), 0x007F7F7F7F7F7F7Fu64);
    assert_eq!(shift_mask::<NorthEast>(2), 0x00003F3F3F3F3F3Fu64);
    assert_eq!(shift_mask::<SouthWest>(1), 0xFEFEFEFEFEFEFE00u64);
    assert_eq!(shift_mask::<SouthWest>(2), 0xFCFCFCFCFCFC0000u64);
}
