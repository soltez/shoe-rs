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

## API

### `Shoe::new(num_decks: usize) -> Shoe`

Creates a new shoe with the given number of standard 52-card decks, shuffled and ready to deal. The cut card starts at full penetration (end of shoe) until `shuffle_and_cut` is called.

Panics if `num_decks` is zero.

### `Shoe::shuffle_and_cut(pen: f32)`

Reshuffles all cards (2–4 passes) and places the cut card at `pen` penetration. For example, `0.75` means 75% of the shoe is dealt before the cut card is reached.

Panics if `pen` is not in `[0.5, 1.0]`.

### `Shoe::deal() -> Option<Card>`

Deals the next card. Returns `Some(Card::Cut)` when the cut card is dealt, and `None` when the shoe is fully exhausted.

### `Shoe::has_reached_cut_card() -> bool`

Returns `true` when the cut card position has been reached, signalling that the current hand is the last before a reshuffle is required.

### `Shoe::burn(n: usize)`

Discards the next `n` cards without returning them. Panics if burning would move the cursor past the cut card position.

### `From<Vec<Card>> for Shoe`

Constructs a `Shoe` from an ordered vector. The vector must contain exactly one `Card::Cut`, which determines the cut position. Dealing begins from the back of the vector.

## Types

- **`Card::Play(CardInt)`** — a standard playing card wrapping a [`kev`](https://crates.io/crates/kev-rs) `CardInt`.
- **`Card::Cut`** — the cut card used to signal a reshuffle.
- **`DECK`** — a `[Card; 52]` constant with all 52 cards in suit order (spades, hearts, diamonds, clubs), ace to two.

## License

LGPL-3.0-only
