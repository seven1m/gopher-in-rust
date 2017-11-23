# Gopher client written in Rust

(Name yet to be decided)

This just reads and prints to the screen for now.

## Building

```
cargo build
```

## Usage

```
target/debug/gopher-in-rust gopherpedia.com
target/debug/gopher-in-rust gopherpedia.com/Cadmium
target/debug/gopher-in-rust "gopherpedia.com/Demon Hunter"
```

## To Do

- [x] connect and print document to screen
- [ ] use curses
- [ ] page/scroll screen
- [ ] allow selection of links with arrow keys and enter
- [ ] accept search string for search resources
