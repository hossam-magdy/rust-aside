# Questions during the secon implementatio of web-server:

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

// Why needed "&mut self" and ".take()"
for worker in &mut self.workers {
    if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
    }
}

// What if Drop is not implemented for ThreadPool?
```
