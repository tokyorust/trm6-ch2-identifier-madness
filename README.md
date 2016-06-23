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

## (Optional bonus challenge 2)

There may be no identifier at all for new records that haven't been saved yet. We are currently unable to represent those at all. Make `Record`'s `id` optional. Additionally, add an `is_new(&self) -> bool` method into `Record` and create a test for it.

## (Optional bonus challenge 3)

Now we're able to create identifiers just fine. But what about when we want to use it? Imagine a situation where you have the following code:

```rust
fn find_record(id: Identifier) -> Option<Record> {
    let _query = format!("select * from records where id = {}", id);
    // Code to run the imaginary query here, irrelevant for this task.
    // Let's just return None and claim that we can't find anything.
    None
}
```

This won't compile. Think of a way to make it compile without adding too many lines inside the method itself. Note that the `UUID` should perhaps be quoted (preferably only in the context of the query), whereas `Numeric` doesn't need quoting at all.
