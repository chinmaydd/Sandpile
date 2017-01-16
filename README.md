## Sandpile

Sandpile implementation in Rust. WIP.

### Example

```rust
extern crate sandpile;
use sandpile::Sandpile;

fn main() {
    let rows = 3;
    let cols = 3;

    // Create a new sandpile.
    let a = Sandpile::new(rows, cols, Some(vec![1,2,3,1,2,3,1,2,3]));

    // Create another sandpile.
    // Since no initialization data is provided,
    // the sandpile will be populated with 0s
    let b = Sandpile::new(rows, cols, None);

    // We have overloaded the '+' operator for sandpiles.
    let c = a + b;

    println!("{:?}", c);
}
```

### License

MIT
