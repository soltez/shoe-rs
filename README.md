# shoe-rs

Playing card dealing shoe library for Rust.

A multi-deck card dealing shoe as used in casino table games such as blackjack and baccarat. Deals and burns cards, tracks the cut card position. Shuffling and cut card placement are the caller's responsibility.

## Usage

```rust
use arrayvec::ArrayVec;
use shoe::{Card, Shoe, DECK, MAX_SHOE_SIZE};

// ArrayVec is used here, but any type with as_slice() works (e.g. heapless::Vec, plain arrays).
// Build a 6-deck shoe, shuffle with your own RNG, place the cut card at 75% penetration.
let mut cards: ArrayVec<Card, MAX_SHOE_SIZE> = ArrayVec::new();
for _ in 0..6 {
    cards.try_extend_from_slice(&DECK).unwrap();
}
// Shuffle cards here with your own RNG, then place the cut card.
let cut_idx = cards.len() / 4;
let last_idx = cards.len();
cards.push(Card::Cut);
cards.swap(cut_idx, last_idx);

let mut shoe = Shoe::from(cards.as_slice());

// Deal cards until the cut card is reached.
while !shoe.has_reached_cut_card() {
    if let Some(card) = shoe.deal() {
        let _ = card;
    }
}
```

## Types

- **`Card::Play(CardInt)`** — a standard playing card wrapping a [`kev`](https://crates.io/crates/kev-rs) `CardInt`.
- **`Card::Cut`** — the cut card used to signal a reshuffle.
- **`DECK`** — a `[Card; 52]` constant with all 52 cards in suit order (spades, hearts, diamonds, clubs), ace to two.
- **`MAX_DECKS`** — maximum number of decks supported (8).
- **`MAX_SHOE_SIZE`** — maximum number of cards in a shoe (`MAX_DECKS * 52 + 1`).

## License

LGPL-3.0-only
