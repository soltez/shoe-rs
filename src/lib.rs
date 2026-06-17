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
//! - [`Shoe`] — holds one or more 52-card decks plus a cut card, and
//!   exposes methods to deal and burn cards.
//!
//! # Example
//!
//! ```
//! use shoe::{Card, Shoe, DECK};
//!
//! // Build a 6-deck shoe with the cut card at 75% penetration (25% from front).
//! let mut cards: Vec<Card> = Vec::new();
//! for _ in 0..6 {
//!     cards.extend_from_slice(&DECK);
//! }
//! // Caller shuffles cards here with their own RNG, then places the cut card.
//! let cut_idx = cards.len() / 4;
//! let last_idx = cards.len();
//! cards.push(Card::Cut);
//! cards.swap(cut_idx, last_idx);
//!
//! let mut shoe = Shoe::from(cards);
//!
//! // Deal cards until the cut card is reached.
//! while !shoe.has_reached_cut_card() {
//!     if let Some(card) = shoe.deal() {
//!         let _ = card;
//!     }
//! }
//! ```

use kev::CardInt;

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
    /// How many cards remain to be dealt.
    #[cfg(test)]
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.cursor
    }

    /// Returns `true` when the cut card has been reached, signalling that the
    /// current hand is the last before a reshuffle is required.
    #[must_use]
    pub fn has_reached_cut_card(&self) -> bool {
        self.cursor <= self.cut_pos
    }

    /// Deal the next card from the shoe, returning it by value.
    /// Returns `Some(Card::Cut)` when the cut card is dealt, and `None` when
    /// the shoe is fully exhausted.
    pub fn deal(&mut self) -> Option<Card> {
        if self.cursor > 0 {
            let card: Card = self.cards[self.cursor];
            self.cursor -= 1;
            Some(card)
        } else {
            None
        }
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

    fn make_shoe(num_decks: usize, cut_card_idx: usize) -> Shoe {
        let mut cards: Vec<Card> = Vec::with_capacity(num_decks * DECK.len() + 1);
        cards.push(Card::Cut);
        for _ in 0..num_decks {
            cards.extend_from_slice(&DECK);
        }
        cards.swap(0, cut_card_idx);
        Shoe::from(cards)
    }

    #[test]
    fn correct_card_count() {
        for n in [1, 4, 6, 8] {
            let shoe = make_shoe(n, n * 26);
            assert_eq!(shoe.remaining(), n * 52);
        }
    }

    #[test]
    fn has_reached_cut_card_after_dealing_past_cut() {
        // 8-deck shoe with cut card at index 14 (~96.5% penetration)
        let mut shoe = make_shoe(8, 14);
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
        let mut shoe = make_shoe(1, 26);
        let _: Card = shoe.deal().expect("shoe should have cards");
        assert_eq!(shoe.remaining(), 51);
    }

    #[test]
    fn exhausted_shoe_returns_none() {
        let mut shoe = make_shoe(1, 0);
        for _ in 0..52 {
            assert!(shoe.deal().is_some());
        }
        assert_eq!(shoe.remaining(), 0);
        assert!(shoe.deal().is_none());
    }

    #[test]
    fn burn_reduces_remaining() {
        let mut shoe = make_shoe(1, 26);
        shoe.burn(5);
        assert_eq!(shoe.remaining(), 47);
    }

    #[test]
    fn from_vec_initial_state() {
        // vec: [A, Cut, B]  ->  cursor = 2 (last index), cut_pos = 1
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
        // [A, B, Cut, C, D]  ->  cursor = 4, cut_pos = 2
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
