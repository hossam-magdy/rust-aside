```rust
// is the stream mutable?

// can we pass non-referenced mut object?

// how was the buffer defined?

// do we "move" in thread::spawn?
thread::spawn(move || {
  handle_request(&mut stream);
});


// Not sure about:
// Arc reference
```
