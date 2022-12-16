## Notes

### Day 1

- `Result::inspect_err` is a nice for 'Do something trivial if Result is Err' situations. The method is not stabilized though (https://github.com/rust-lang/rust/issues/91345).
- [`Tap`](https://docs.rs/tap/latest/tap/) crate if you prefer method like ergonomics for [inspecting](https://docs.rs/tap/latest/tap/#tapping) or [piping](https://docs.rs/tap/latest/tap/#piping).
