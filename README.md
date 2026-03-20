# shoe-rs

Playing card dealing shoe library for Rust.

A multi-deck card dealing shoe as used in casino table games such as blackjack and baccarat. Supports shuffling, cut card placement, dealing, and burning cards.

## Usage

```rust
use shoe::{Card, Shoe};

// Create a 6-deck shoe, shuffle, and place the cut card at 75% penetration.
let mut shoe = Shoe::new(6);
shoe.shuffle_and_cut(0.75);

// Deal cards until the cut card is reached.
while !shoe.has_reached_cut_card() {
    if let Some(card) = shoe.deal() {
        let _ = card;
    }
}

// Reshuffle for the next session.
shoe.shuffle_and_cut(0.75);
```

## Types

- **`Card::Play(CardInt)`** — a standard playing card wrapping a [`kev`](https://crates.io/crates/kev-rs) `CardInt`.
- **`Card::Cut`** — the cut card used to signal a reshuffle.
- **`DECK`** — a `[Card; 52]` constant with all 52 cards in suit order (spades, hearts, diamonds, clubs), ace to two.

## License

LGPL-3.0-only
