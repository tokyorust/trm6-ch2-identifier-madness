# Challenge #2 - Identifier madness

For [Tokyo Rust Meetup's mini-hackathon on 2016-06-23](http://www.meetup.com/Tokyo-Rust-Meetup/events/231555496/).

## Setup

For this task, any Rust nightly should be fine.

## The challenge

So you have a friend who likes to use UUIDs as identifiers. Another swears by plain numbers, claiming that `u64` will surely be enough for everything. You don't feel like getting into a religious argument with them, so you decide to make your library work with both methods.

Stepping up the difficulty from challenge 1, you task is to make the provided `Record` API work with both `String` (representing UUIDs) and `u64` (representing numeric identifiers).

Simply make `cargo test` pass without removing any tests and you're done! For convenience, `cargo run` already works and can be used for quick testing.

## (Optional bonus challenge 1)

Make `Record` accept a `&str` as well, for convenience reasons. Hint: the solution from challenge 1 may not work so well this time!
