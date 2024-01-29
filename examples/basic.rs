use javarandom::JavaRandom;

fn main() {
    let mut rnd = JavaRandom::with_seed(123);

    for i in 0..10 {
        println!("{i:<0}: {}", rnd.next_int());
    }
}
