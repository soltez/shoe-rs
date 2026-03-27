//! Playing card dealing shoe library.
//!
//! This crate provides a [`Shoe`] — a multi-deck card dealing shoe as used in
//! casino table games such as blackjack and baccarat.
//!
//! # Core types
//!
//! - [`Card`] — a playing card that is either a standard [`Card::Play`] card
//!   (wrapping a [`kev::CardInt`]) or a [`Card::Cut`] card that marks the
//!   reshuffle point.
//! - [`Shoe`] — holds one or more shuffled 52-card decks plus a cut card, and
//!   exposes methods to deal, burn, and reshuffle.
//!
//! # Example
//!
//! ```
//! use shoe::{Card, Shoe};
//!
//! // Create a 6-deck shoe and place the cut card at 75% penetration.
//! let mut shoe = Shoe::new(6);
//! shoe.cut(0.75);
//!
//! // Deal cards until the cut card is reached.
//! while !shoe.has_reached_cut_card() {
//!     if let Some(card) = shoe.deal() {
//!         let _ = card;
//!     }
//! }
//!
//! // Reshuffle 3 times for the next session.
//! shoe.shuffle(3);
//! shoe.cut(0.75);
//! ```

use kev::CardInt;
use num_traits::cast::ToPrimitive;
use rand::rng;
use rand::seq::SliceRandom;

/// A standard 52-card deck in suit order: spades, hearts, diamonds, clubs.
pub const DECK: [Card; 52] = [
    // Spades
    Card::Play(CardInt::CardAs),
    Card::Play(CardInt::CardKs),
    Card::Play(CardInt::CardQs),
    Card::Play(CardInt::CardJs),
    Card::Play(CardInt::CardTs),
    Card::Play(CardInt::Card9s),
    Card::Play(CardInt::Card8s),
    Card::Play(CardInt::Card7s),
    Card::Play(CardInt::Card6s),
    Card::Play(CardInt::Card5s),
    Card::Play(CardInt::Card4s),
    Card::Play(CardInt::Card3s),
    Card::Play(CardInt::Card2s),
    // Hearts
    Card::Play(CardInt::CardAh),
    Card::Play(CardInt::CardKh),
    Card::Play(CardInt::CardQh),
    Card::Play(CardInt::CardJh),
    Card::Play(CardInt::CardTh),
    Card::Play(CardInt::Card9h),
    Card::Play(CardInt::Card8h),
    Card::Play(CardInt::Card7h),
    Card::Play(CardInt::Card6h),
    Card::Play(CardInt::Card5h),
    Card::Play(CardInt::Card4h),
    Card::Play(CardInt::Card3h),
    Card::Play(CardInt::Card2h),
    // Diamonds
    Card::Play(CardInt::CardAd),
    Card::Play(CardInt::CardKd),
    Card::Play(CardInt::CardQd),
    Card::Play(CardInt::CardJd),
    Card::Play(CardInt::CardTd),
    Card::Play(CardInt::Card9d),
    Card::Play(CardInt::Card8d),
    Card::Play(CardInt::Card7d),
    Card::Play(CardInt::Card6d),
    Card::Play(CardInt::Card5d),
    Card::Play(CardInt::Card4d),
    Card::Play(CardInt::Card3d),
    Card::Play(CardInt::Card2d),
    // Clubs
    Card::Play(CardInt::CardAc),
    Card::Play(CardInt::CardKc),
    Card::Play(CardInt::CardQc),
    Card::Play(CardInt::CardJc),
    Card::Play(CardInt::CardTc),
    Card::Play(CardInt::Card9c),
    Card::Play(CardInt::Card8c),
    Card::Play(CardInt::Card7c),
    Card::Play(CardInt::Card6c),
    Card::Play(CardInt::Card5c),
    Card::Play(CardInt::Card4c),
    Card::Play(CardInt::Card3c),
    Card::Play(CardInt::Card2c),
];

/// A card that can be held in a [`Shoe`].
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Card {
    /// A standard playing card.
    Play(CardInt),
    /// A cut card used to signal a reshuffle.
    Cut,
}

/// A multi-deck dealing shoe.
pub struct Shoe {
    cards: Vec<Card>,
    cursor: usize,
    cut_pos: usize,
}

impl Shoe {
    /// Create a new shoe with `num_decks` standard 52-card decks. The cut card
    /// is placed at the end of the shoe. Call [`Shoe::cut`] before dealing to
    /// set the cut position and enable dealing.
    ///
    /// # Panics
    /// Panics if `num_decks` is zero.
    #[must_use]
    pub fn new(num_decks: usize) -> Self {
        assert!(num_decks > 0, "a shoe must contain at least one deck");

        let capacity: usize = num_decks * DECK.len();
        let mut cards: Vec<Card> = Vec::with_capacity(capacity + 1);

        for _ in 0..num_decks {
            cards.extend_from_slice(&DECK);
        }
        cards.shuffle(&mut rng());
        cards.push(Card::Cut);
        cards.swap(capacity, 0);

        Self {
            cards,
            cursor: 0,
            cut_pos: 0,
        }
    }

    /// How many cards remain to be dealt.
    #[cfg(test)]
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.cursor
    }

    /// Returns `true` when the cut card has been reached, signalling that the
    /// current hand is the last before a reshuffle is required. Also returns
    /// `true` before [`Shoe::cut`] has been called, since the shoe is not yet
    /// ready to deal.
    #[must_use]
    pub fn has_reached_cut_card(&self) -> bool {
        self.cursor <= self.cut_pos
    }

    /// Deal the next card from the shoe, returning it by value.
    /// Returns `Some(Card::Cut)` when the cut card is dealt, and `None` when
    /// the shoe is fully exhausted or [`Shoe::cut`] has not yet been called.
    pub fn deal(&mut self) -> Option<Card> {
        if self.cursor > 0 {
            let card: Card = self.cards[self.cursor];
            self.cursor -= 1;
            Some(card)
        } else {
            None
        }
    }

    /// Shuffle all cards `n` times, move the cut card to the end of the shoe,
    /// and reset both the cursor and cut position, disabling dealing until
    /// [`Shoe::cut`] is called to place the cut card and restore the cursor.
    ///
    /// # Panics
    /// Panics if `n` is zero.
    pub fn shuffle(&mut self, n: u8) {
        assert!(n > 0, "shuffle count must be at least 1");
        let mut rng = rng();
        for _ in 0..n {
            self.cards.shuffle(&mut rng);
        }

        let cut = self.cards.iter().position(|&x| x == Card::Cut).unwrap();
        self.cards.swap(cut, 0);

        self.cursor = 0;
        self.cut_pos = 0;
    }

    /// Place the cut card at a position determined by the penetration ratio.
    ///
    /// `pen` is the penetration ratio: the fraction of the shoe dealt before
    /// reshuffling. For example, `0.75` places the cut card 75% of the way
    /// through the shoe, leaving 25% undealt.
    ///
    /// The cut card must be at the end of the shoe before this is called, as
    /// guaranteed by [`Shoe::new`] and [`Shoe::shuffle`]. Call this method
    /// after shuffling to enable dealing.
    ///
    /// # Panics
    /// Panics if `pen` is not in the range `[0.5, 1.0]`.
    /// Panics if the cut card is not at the end of the shoe.
    pub fn cut(&mut self, pen: f32) {
        assert!(
            (0.5..=1.0).contains(&pen),
            "penetration ratio must be in the range [0.5, 1.0]"
        );

        assert!(
            self.cards[0] == Card::Cut,
            "cut card must be at the end of the shoe; call shuffle first"
        );

        let last = self.cards.len() - 1;
        let shoe_len = last.to_f32().expect("shoe length fits in f32");
        let cut_pos = ((1.0 - pen) * shoe_len)
            .to_usize()
            .expect("cut position fits in usize");
        self.cards.swap(0, cut_pos);

        self.cursor = last;
        self.cut_pos = cut_pos;
    }

    /// Discard the next `n` cards from the shoe without returning them.
    ///
    /// # Panics
    /// Panics if burning `n` cards would move the cursor past the cut card position.
    pub fn burn(&mut self, n: usize) {
        assert!(self.cut_pos + n <= self.cursor, "burning too many cards");
        self.cursor -= n;
    }

    /// Returns the number of cards that remain in the shoe after the cut card.
    #[must_use]
    pub fn stub_size(&self) -> usize {
        self.cut_pos
    }
}

impl From<Vec<Card>> for Shoe {
    /// Create a [`Shoe`] from an ordered `Vec<Card>`.
    ///
    /// The vector must contain exactly one [`Card::Cut`], which determines the
    /// cut position. The cursor is set to the last index so dealing begins from
    /// the back of the vector.
    ///
    /// # Panics
    /// Panics if the vector contains zero or more than one [`Card::Cut`].
    fn from(cards: Vec<Card>) -> Self {
        let mut cut_iter = cards
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| (c == Card::Cut).then_some(i));
        let cut_pos = cut_iter.next().expect("vector must contain a cut card");
        assert!(
            cut_iter.next().is_none(),
            "vector must contain exactly one cut card"
        );
        let n = cards.len();
        Shoe {
            cards,
            cursor: n - 1,
            cut_pos,
        }
    }
}

#[cfg(test)]
mod shoe_tests {
    use super::*;

    #[test]
    fn correct_card_count() {
        for n in [1, 4, 6, 8] {
            let mut shoe = Shoe::new(n);
            shoe.cut(0.5);
            assert_eq!(shoe.remaining(), n * 52);
        }
    }

    #[test]
    #[should_panic(expected = "at least one deck")]
    fn zero_decks_panics() {
        let _ = Shoe::new(0);
    }

    #[test]
    fn deal_returns_none_before_cut() {
        let mut shoe = Shoe::new(1);
        assert!(shoe.deal().is_none());
    }

    #[test]
    #[should_panic(expected = "shuffle count must be at least 1")]
    fn zero_shuffles_panics() {
        let mut shoe = Shoe::new(1);
        shoe.shuffle(0);
    }

    #[test]
    #[should_panic(expected = "cut card must be at the end of the shoe")]
    fn cut_without_shuffle_after_cut_panics() {
        let mut shoe = Shoe::new(1);
        shoe.cut(0.75);
        shoe.cut(0.75);
    }

    #[test]
    fn has_reached_cut_card_initially() {
        let shoe = Shoe::new(1);
        assert!(shoe.has_reached_cut_card());
    }

    #[test]
    fn has_reached_cut_card_after_dealing_past_cut() {
        let mut shoe = Shoe::new(8);
        shoe.cut(0.965);
        for _ in 0..402 {
            assert!(shoe.deal().is_some());
        }
        assert!(shoe.has_reached_cut_card());
        assert_eq!(shoe.deal(), Some(Card::Cut));
        assert!(shoe.has_reached_cut_card());
        assert!(shoe.deal().is_some());
    }

    #[test]
    fn deal_returns_card() {
        let mut shoe = Shoe::new(1);
        shoe.cut(0.5);
        let _: Card = shoe.deal().expect("shoe should have cards");
        assert_eq!(shoe.remaining(), 51);
    }

    #[test]
    fn exhausted_shoe_returns_none() {
        let mut shoe = Shoe::new(1);
        shoe.cut(1.0);
        for _ in 0..52 {
            assert!(shoe.deal().is_some());
        }
        assert_eq!(shoe.remaining(), 0);
        assert!(shoe.deal().is_none());
    }

    #[test]
    fn shuffle_and_cut_restores_full_shoe() {
        let mut shoe = Shoe::new(1);
        shoe.cut(0.75);
        for _ in 0..10 {
            assert!(shoe.deal().is_some());
        }
        shoe.shuffle(1);
        shoe.cut(0.75);
        assert_eq!(shoe.remaining(), 52);
    }

    #[test]
    #[should_panic(expected = "penetration ratio must be in the range [0.5, 1.0]")]
    fn low_pen_panics() {
        let mut shoe = Shoe::new(1);
        shoe.cut(0.49);
    }

    #[test]
    #[should_panic(expected = "penetration ratio must be in the range [0.5, 1.0]")]
    fn high_pen_panics() {
        let mut shoe = Shoe::new(1);
        shoe.cut(1.01);
    }

    #[test]
    fn burn_reduces_remaining() {
        let mut shoe = Shoe::new(1);
        shoe.cut(0.5);
        shoe.burn(5);
        assert_eq!(shoe.remaining(), 47);
    }

    #[test]
    fn from_vec_initial_state() {
        // vec: [A, Cut, B]  →  cursor = 2 (last index), cut_pos = 1
        let cards = vec![
            Card::Play(CardInt::CardAs),
            Card::Cut,
            Card::Play(CardInt::CardKs),
        ];
        let shoe = Shoe::from(cards);
        assert_eq!(shoe.remaining(), 2); // cursor = n - 1 = 2
        assert!(!shoe.has_reached_cut_card()); // cursor(2) > cut_pos(1)
    }

    #[test]
    fn from_vec_deals_in_reverse_order() {
        // cursor starts at last index and decrements; cards dealt top-down
        let cards = vec![
            Card::Play(CardInt::CardAs),
            Card::Cut,
            Card::Play(CardInt::CardKs),
        ];
        let mut shoe = Shoe::from(cards);
        assert_eq!(shoe.deal(), Some(Card::Play(CardInt::CardKs))); // cards[2]
        assert_eq!(shoe.deal(), Some(Card::Cut)); // cards[1]
        assert_eq!(shoe.deal(), None); // cursor = 0
    }

    #[test]
    fn from_vec_has_reached_cut_card_after_dealing_past_cut() {
        // [A, B, Cut, C, D]  →  cursor = 4, cut_pos = 2
        let cards = vec![
            Card::Play(CardInt::CardAs),
            Card::Play(CardInt::CardKs),
            Card::Cut,
            Card::Play(CardInt::CardQs),
            Card::Play(CardInt::CardJs),
        ];
        let mut shoe = Shoe::from(cards);
        assert!(!shoe.has_reached_cut_card()); // cursor = 4 > cut_pos = 2
        assert!(shoe.deal().is_some()); // cursor = 3
        assert!(!shoe.has_reached_cut_card()); // 3 > 2
        assert!(shoe.deal().is_some()); // cursor = 2
        assert!(shoe.has_reached_cut_card()); // 2 = 2
        assert!(shoe.deal().is_some()); // cursor = 1
        assert!(shoe.has_reached_cut_card()); // 1 < 2
    }

    #[test]
    fn stub_size_is_zero_before_cut() {
        let shoe = Shoe::new(1);
        assert_eq!(shoe.stub_size(), 0);
    }

    #[test]
    fn stub_size_reflects_penetration() {
        // 1-deck shoe: last index = 52; cut_pos = (1.0 - 0.75) * 52 = 13
        let mut shoe = Shoe::new(1);
        shoe.cut(0.75);
        assert_eq!(shoe.stub_size(), 13);
    }

    #[test]
    fn stub_size_is_zero_at_full_penetration() {
        let mut shoe = Shoe::new(1);
        shoe.cut(1.0);
        assert_eq!(shoe.stub_size(), 0);
    }

    #[test]
    fn stub_size_is_zero_after_shuffle() {
        let mut shoe = Shoe::new(1);
        shoe.cut(0.75);
        shoe.shuffle(1);
        assert_eq!(shoe.stub_size(), 0);
    }

    #[test]
    fn stub_size_from_vec() {
        // [A, Cut, B, C] -> cut_pos = 1, stub_size = 1
        let cards = vec![
            Card::Play(CardInt::CardAs),
            Card::Cut,
            Card::Play(CardInt::CardKs),
            Card::Play(CardInt::CardQs),
        ];
        let shoe = Shoe::from(cards);
        assert_eq!(shoe.stub_size(), 1);
    }

    #[test]
    #[should_panic(expected = "vector must contain a cut card")]
    fn from_vec_no_cut_panics() {
        let cards = vec![Card::Play(CardInt::CardAs), Card::Play(CardInt::CardKs)];
        let _ = Shoe::from(cards);
    }

    #[test]
    #[should_panic(expected = "vector must contain exactly one cut card")]
    fn from_vec_multiple_cuts_panics() {
        let cards = vec![Card::Cut, Card::Play(CardInt::CardAs), Card::Cut];
        let _ = Shoe::from(cards);
    }
}
