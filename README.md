# javarandom-rs
javarandom-rs is a pure Rust implementation of java.util.Random

## Example
```rs
fn main() {
    let mut rng = JavaRandom::with_seed(1234);

    for i in 0..10 {
        println!("{i}: {}", rng.next_int());
    }
}
```
