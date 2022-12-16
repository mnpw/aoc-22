## Notes

### Day 1

- `Result::inspect_err` is a nice for 'Do something trivial if Result is Err' situations. The method is not stabilized though (https://github.com/rust-lang/rust/issues/91345).
- [`Tap`](https://docs.rs/tap/latest/tap/) crate if you prefer method like ergonomics for [inspecting](https://docs.rs/tap/latest/tap/#tapping) or [piping](https://docs.rs/tap/latest/tap/#piping).

### Day 2

- Use `indoc` for mock inputs while testing code
- TDD is not bad actually
- Part 2 highlighted that parsing of input itself should be configurable, instead of treating second value in a round as a move it should be the outcome of round. 
