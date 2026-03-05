use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

const PRIMES: [u8; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// The rank of a playing card, ordered from lowest (Deuce) to highest (Ace).
///
/// The discriminant value is used as an index into the `PRIMES` table and to
/// compute the card's face value and activate the one-hot bit position
/// in the upper 16 bits of the Cactus Kev encoding.
#[repr(u8)]
#[derive(FromPrimitive, PartialEq, Debug, Clone, Copy)]
pub enum Rank {
    /// Two (2)
    Deuce   = 0,
    /// Three (3)
    Trey    = 1,
    /// Four (4)
    Four    = 2,
    /// Five (5)
    Five    = 3,
    /// Six (6)
    Six     = 4,
    /// Seven (7)
    Seven   = 5,
    /// Eight (8)
    Eight   = 6,
    /// Nine (9)
    Nine    = 7,
    /// Ten (T)
    Ten     = 8,
    /// Jack (J)
    Jack    = 9,
    /// Queen (Q)
    Queen   = 10,
    /// King (K)
    King    = 11,
    /// Ace (A)
    Ace     = 12,
}

impl Rank {
    /// Returns the unique prime number assigned to this rank.
    ///
    /// Primes are used in the Cactus Kev encoding so that any hand can
    /// be identified by the product of its ranks' primes, enabling fast lookup.
    /// Values range from 2 (Deuce) to 41 (Ace).
    pub fn prime(self) -> u32 {
        PRIMES[self as usize] as u32
    }

    /// Returns the face value of this rank (2–14).
    ///
    /// Stored in bits 8–11 of the Cactus Kev card integer.
    pub fn value(self) -> u32 {
        2 + self as u32
    }

    /// Returns a one-hot bitmask for this rank's position.
    ///
    /// Shifted left by 16 and placed in the upper 16 bits of the Cactus Kev
    /// card integer, allowing straight detection.
    fn onehot(self) -> u32 {
        1 << self as u32
    }

    /// Parses a single character into a `Rank`.
    ///
    /// Accepts both upper- and lowercase letters (`A`/`a` through `2`), plus
    /// `T`/`t` for Ten. Returns `None` for any unrecognised character.
    pub fn from_char(value: char) -> Option<Self> {
        match value {
            'A' | 'a'   => Some(Rank::Ace),
            'K' | 'k'   => Some(Rank::King),
            'Q' | 'q'   => Some(Rank::Queen),
            'J' | 'j'   => Some(Rank::Jack),
            'T' | 't'   => Some(Rank::Ten),
            '9'         => Some(Rank::Nine),
            '8'         => Some(Rank::Eight),
            '7'         => Some(Rank::Seven),
            '6'         => Some(Rank::Six),
            '5'         => Some(Rank::Five),
            '4'         => Some(Rank::Four),
            '3'         => Some(Rank::Trey),
            '2'         => Some(Rank::Deuce),
            _ => None,
        }
    }
}

#[cfg(test)]
mod rank_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('A', Some(Rank::Ace))]
    #[case('a', Some(Rank::Ace))]
    #[case('K', Some(Rank::King))]
    #[case('k', Some(Rank::King))]
    #[case('Q', Some(Rank::Queen))]
    #[case('q', Some(Rank::Queen))]
    #[case('J', Some(Rank::Jack))]
    #[case('j', Some(Rank::Jack))]
    #[case('T', Some(Rank::Ten))]
    #[case('t', Some(Rank::Ten))]
    #[case('9', Some(Rank::Nine))]
    #[case('8', Some(Rank::Eight))]
    #[case('7', Some(Rank::Seven))]
    #[case('6', Some(Rank::Six))]
    #[case('5', Some(Rank::Five))]
    #[case('4', Some(Rank::Four))]
    #[case('3', Some(Rank::Trey))]
    #[case('2', Some(Rank::Deuce))]
    #[case('1', None)]
    fn from_char(#[case] input: char, #[case] expected: Option<Rank>) {
        assert_eq!(Rank::from_char(input), expected)
    }
}

/// The suit of a playing card.
///
/// The discriminant is a one-hot nibble stored in bits 12–15 of the Cactus Kev
/// card integer, so flush detection can be tested with a
/// single bitwise AND.
#[repr(u8)]
#[derive(FromPrimitive, PartialEq, Debug)]
pub enum Suit {
    /// Spades (♠)
    Spade   = 0x1,
    /// Hearts (♥)
    Heart   = 0x2,
    /// Diamonds (♦)
    Diamond = 0x4,
    /// Clubs (♣)
    Club    = 0x8,
}

impl Suit {
    /// Parses a single character into a `Suit`.
    ///
    /// Accepts Unicode suit symbols (`♠ ♤ ♥ ♡ ♦ ♢ ♣ ♧`) as well as ASCII
    /// letters (`S`/`s`, `H`/`h`, `D`/`d`, `C`/`c`). Returns `None` for any
    /// unrecognised character.
    pub fn from_char(value: char) -> Option<Self> {
        match value {
            '♤' | '♠' | 'S' | 's' => Some(Suit::Spade),
            '♡' | '♥' | 'H' | 'h' => Some(Suit::Heart),
            '♢' | '♦' | 'D' | 'd' => Some(Suit::Diamond),
            '♧' | '♣' | 'C' | 'c' => Some(Suit::Club),
            _ => None,
        }
    }
}

#[cfg(test)]
mod suit_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('♤', Some(Suit::Spade))]
    #[case('♠', Some(Suit::Spade))]
    #[case('S', Some(Suit::Spade))]
    #[case('s', Some(Suit::Spade))]
    #[case('♡', Some(Suit::Heart))]
    #[case('♥', Some(Suit::Heart))]
    #[case('H', Some(Suit::Heart))]
    #[case('h', Some(Suit::Heart))]
    #[case('♢', Some(Suit::Diamond))]
    #[case('♦', Some(Suit::Diamond))]
    #[case('D', Some(Suit::Diamond))]
    #[case('d', Some(Suit::Diamond))]
    #[case('♧', Some(Suit::Club))]
    #[case('♣', Some(Suit::Club))]
    #[case('C', Some(Suit::Club))]
    #[case('c', Some(Suit::Club))]
    #[case('z', None)]
    fn from_char(#[case] input: char, #[case] expected: Option<Suit>) {
        assert_eq!(Suit::from_char(input), expected)
    }
}

/// A playing card represented as a 32-bit integer using the Cactus Kev encoding.
///
/// Each variant encodes a unique (rank, suit) pair in a single `u32` with the
/// following bit layout:
///
/// ```text
/// Bits 31–16  one-hot rank flag (bit 16+rank_index set)
/// Bits 15–12  one-hot suit nibble (Spade=1, Heart=2, Diamond=4, Club=8)
/// Bits 11– 8  face value (2–14)
/// Bits  7– 0  rank prime (2, 3, 5, … 41)
/// ```
///
/// Variants are named `Card<Rank><Suit>` where rank uses its conventional
/// character (`A K Q J T 9 … 2`) and suit uses its initial (`s h d c`).
#[repr(u32)]
#[derive(FromPrimitive, PartialEq, Debug)]
pub enum CardInt{
    CardAs = 0b0001_0000_0000_0000_0001_1110_0010_1001,
    CardKs = 0b0000_1000_0000_0000_0001_1101_0010_0101,
    CardQs = 0b0000_0100_0000_0000_0001_1100_0001_1111,
    CardJs = 0b0000_0010_0000_0000_0001_1011_0001_1101,
    CardTs = 0b0000_0001_0000_0000_0001_1010_0001_0111,
    Card9s = 0b0000_0000_1000_0000_0001_1001_0001_0011,
    Card8s = 0b0000_0000_0100_0000_0001_1000_0001_0001,
    Card7s = 0b0000_0000_0010_0000_0001_0111_0000_1101,
    Card6s = 0b0000_0000_0001_0000_0001_0110_0000_1011,
    Card5s = 0b0000_0000_0000_1000_0001_0101_0000_0111,
    Card4s = 0b0000_0000_0000_0100_0001_0100_0000_0101,
    Card3s = 0b0000_0000_0000_0010_0001_0011_0000_0011,
    Card2s = 0b0000_0000_0000_0001_0001_0010_0000_0010,
    CardAh = 0b0001_0000_0000_0000_0010_1110_0010_1001,
    CardKh = 0b0000_1000_0000_0000_0010_1101_0010_0101,
    CardQh = 0b0000_0100_0000_0000_0010_1100_0001_1111,
    CardJh = 0b0000_0010_0000_0000_0010_1011_0001_1101,
    CardTh = 0b0000_0001_0000_0000_0010_1010_0001_0111,
    Card9h = 0b0000_0000_1000_0000_0010_1001_0001_0011,
    Card8h = 0b0000_0000_0100_0000_0010_1000_0001_0001,
    Card7h = 0b0000_0000_0010_0000_0010_0111_0000_1101,
    Card6h = 0b0000_0000_0001_0000_0010_0110_0000_1011,
    Card5h = 0b0000_0000_0000_1000_0010_0101_0000_0111,
    Card4h = 0b0000_0000_0000_0100_0010_0100_0000_0101,
    Card3h = 0b0000_0000_0000_0010_0010_0011_0000_0011,
    Card2h = 0b0000_0000_0000_0001_0010_0010_0000_0010,
    CardAd = 0b0001_0000_0000_0000_0100_1110_0010_1001,
    CardKd = 0b0000_1000_0000_0000_0100_1101_0010_0101,
    CardQd = 0b0000_0100_0000_0000_0100_1100_0001_1111,
    CardJd = 0b0000_0010_0000_0000_0100_1011_0001_1101,
    CardTd = 0b0000_0001_0000_0000_0100_1010_0001_0111,
    Card9d = 0b0000_0000_1000_0000_0100_1001_0001_0011,
    Card8d = 0b0000_0000_0100_0000_0100_1000_0001_0001,
    Card7d = 0b0000_0000_0010_0000_0100_0111_0000_1101,
    Card6d = 0b0000_0000_0001_0000_0100_0110_0000_1011,
    Card5d = 0b0000_0000_0000_1000_0100_0101_0000_0111,
    Card4d = 0b0000_0000_0000_0100_0100_0100_0000_0101,
    Card3d = 0b0000_0000_0000_0010_0100_0011_0000_0011,
    Card2d = 0b0000_0000_0000_0001_0100_0010_0000_0010,
    CardAc = 0b0001_0000_0000_0000_1000_1110_0010_1001,
    CardKc = 0b0000_1000_0000_0000_1000_1101_0010_0101,
    CardQc = 0b0000_0100_0000_0000_1000_1100_0001_1111,
    CardJc = 0b0000_0010_0000_0000_1000_1011_0001_1101,
    CardTc = 0b0000_0001_0000_0000_1000_1010_0001_0111,
    Card9c = 0b0000_0000_1000_0000_1000_1001_0001_0011,
    Card8c = 0b0000_0000_0100_0000_1000_1000_0001_0001,
    Card7c = 0b0000_0000_0010_0000_1000_0111_0000_1101,
    Card6c = 0b0000_0000_0001_0000_1000_0110_0000_1011,
    Card5c = 0b0000_0000_0000_1000_1000_0101_0000_0111,
    Card4c = 0b0000_0000_0000_0100_1000_0100_0000_0101,
    Card3c = 0b0000_0000_0000_0010_1000_0011_0000_0011,
    Card2c = 0b0000_0000_0000_0001_1000_0010_0000_0010,
}

impl CardInt {
    /// Constructs a `CardInt` from a two-character string such as `"As"` or `"Td"`.
    ///
    /// The first character is parsed as a [`Rank`] via [`Rank::from_char`] and
    /// the second as a [`Suit`] via [`Suit::from_char`]. Returns `None` if
    /// either character is unrecognised or the string is not exactly two
    /// characters long.
    pub fn new(index: &str) -> Option<Self> {
        let mut chars = index.chars();
        let rank: Rank = match chars.next() {
            None => return None,
            Some(r) => Rank::from_char(r)?,
        };
        let suit: Suit = match chars.next() {
            None => return None,
            Some(s) => Suit::from_char(s)?,
        };
        let _: CardInt = match chars.next() {
            Some(_) => return None,
            None => {
                let bit_pattern: u32 = rank.prime() | rank.value() << 8 | (suit as u32) << 12 | rank.onehot() << 16;
                return CardInt::from_u32(bit_pattern)
            }
        };
    }

    /// Extracts the [`Rank`] from this card's face-value field (bits 8–11).
    pub fn rank(self) -> Rank {
        Rank::from_u8(((self as u32 >> 8) - 2) as u8 & 0xF).unwrap()
    }

    /// Extracts the [`Suit`] from this card's suit nibble (bits 12–15).
    pub fn suit(self) -> Suit {
        Suit::from_u16((self as u32 >> 12 & 0xF) as u16).unwrap()
    }
}

#[cfg(test)]
mod cactus_kev_card_tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[rstest]
    #[case(0b00001000_00000000_01001101_00100101, CardInt::CardKd)]
    #[case(0b00000000_00001000_00010101_00000111, CardInt::Card5s)]
    #[case(0b00000010_00000000_10001011_00011101, CardInt::CardJc)]
    #[case(0b00000100_00000000_10001100_00011111, CardInt::CardQc)]
    fn bit_pattern_example(#[case] input: u32, #[case] expected: CardInt) {
        assert_eq!(CardInt::from_u32(input), Some(expected));
    }

    #[template]
    #[rstest]
    #[case(Rank::Ace,   Suit::Spade,    CardInt::CardAs)]
    #[case(Rank::King,  Suit::Spade,    CardInt::CardKs)]
    #[case(Rank::Queen, Suit::Spade,    CardInt::CardQs)]
    #[case(Rank::Jack,  Suit::Spade,    CardInt::CardJs)]
    #[case(Rank::Ten,   Suit::Spade,    CardInt::CardTs)]
    #[case(Rank::Nine,  Suit::Spade,    CardInt::Card9s)]
    #[case(Rank::Eight, Suit::Spade,    CardInt::Card8s)]
    #[case(Rank::Seven, Suit::Spade,    CardInt::Card7s)]
    #[case(Rank::Six,   Suit::Spade,    CardInt::Card6s)]
    #[case(Rank::Five,  Suit::Spade,    CardInt::Card5s)]
    #[case(Rank::Four,  Suit::Spade,    CardInt::Card4s)]
    #[case(Rank::Trey,  Suit::Spade,    CardInt::Card3s)]
    #[case(Rank::Deuce, Suit::Spade,    CardInt::Card2s)]
    #[case(Rank::Ace,   Suit::Heart,    CardInt::CardAh)]
    #[case(Rank::King,  Suit::Heart,    CardInt::CardKh)]
    #[case(Rank::Queen, Suit::Heart,    CardInt::CardQh)]
    #[case(Rank::Jack,  Suit::Heart,    CardInt::CardJh)]
    #[case(Rank::Ten,   Suit::Heart,    CardInt::CardTh)]
    #[case(Rank::Nine,  Suit::Heart,    CardInt::Card9h)]
    #[case(Rank::Eight, Suit::Heart,    CardInt::Card8h)]
    #[case(Rank::Seven, Suit::Heart,    CardInt::Card7h)]
    #[case(Rank::Six,   Suit::Heart,    CardInt::Card6h)]
    #[case(Rank::Five,  Suit::Heart,    CardInt::Card5h)]
    #[case(Rank::Four,  Suit::Heart,    CardInt::Card4h)]
    #[case(Rank::Trey,  Suit::Heart,    CardInt::Card3h)]
    #[case(Rank::Deuce, Suit::Heart,    CardInt::Card2h)]
    #[case(Rank::Ace,   Suit::Diamond,  CardInt::CardAd)]
    #[case(Rank::King,  Suit::Diamond,  CardInt::CardKd)]
    #[case(Rank::Queen, Suit::Diamond,  CardInt::CardQd)]
    #[case(Rank::Jack,  Suit::Diamond,  CardInt::CardJd)]
    #[case(Rank::Ten,   Suit::Diamond,  CardInt::CardTd)]
    #[case(Rank::Nine,  Suit::Diamond,  CardInt::Card9d)]
    #[case(Rank::Eight, Suit::Diamond,  CardInt::Card8d)]
    #[case(Rank::Seven, Suit::Diamond,  CardInt::Card7d)]
    #[case(Rank::Six,   Suit::Diamond,  CardInt::Card6d)]
    #[case(Rank::Five,  Suit::Diamond,  CardInt::Card5d)]
    #[case(Rank::Four,  Suit::Diamond,  CardInt::Card4d)]
    #[case(Rank::Trey,  Suit::Diamond,  CardInt::Card3d)]
    #[case(Rank::Deuce, Suit::Diamond,  CardInt::Card2d)]
    #[case(Rank::Ace,   Suit::Club,     CardInt::CardAc)]
    #[case(Rank::King,  Suit::Club,     CardInt::CardKc)]
    #[case(Rank::Queen, Suit::Club,     CardInt::CardQc)]
    #[case(Rank::Jack,  Suit::Club,     CardInt::CardJc)]
    #[case(Rank::Ten,   Suit::Club,     CardInt::CardTc)]
    #[case(Rank::Nine,  Suit::Club,     CardInt::Card9c)]
    #[case(Rank::Eight, Suit::Club,     CardInt::Card8c)]
    #[case(Rank::Seven, Suit::Club,     CardInt::Card7c)]
    #[case(Rank::Six,   Suit::Club,     CardInt::Card6c)]
    #[case(Rank::Five,  Suit::Club,     CardInt::Card5c)]
    #[case(Rank::Four,  Suit::Club,     CardInt::Card4c)]
    #[case(Rank::Trey,  Suit::Club,     CardInt::Card3c)]
    #[case(Rank::Deuce, Suit::Club,     CardInt::Card2c)]
    fn all_cases(#[case] rank: Rank, #[case] suit: Suit, #[case] card: CardInt) {}

    #[apply(all_cases)]
    fn binary_literal_integrity(rank: Rank, suit: Suit, card: CardInt) {
        let bit_pattern: u32 = rank.prime() | rank.value() << 8 | (suit as u32) << 12 | rank.onehot() << 16;
        let actual: CardInt = CardInt::from_u32(bit_pattern).unwrap();
        assert_eq!(actual, card);
    }

    #[apply(all_cases)]
    fn rank_extract(rank: Rank, _suit: Suit, card: CardInt) {
        assert_eq!(card.rank(), rank);
    }

    #[apply(all_cases)]
    fn suit_extract(_rank: Rank, suit: Suit, card: CardInt) {
        assert_eq!(card.suit(), suit);
    }
}
