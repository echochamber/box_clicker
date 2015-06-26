extern crate rand;
mod block;

fn main() {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let num1 = rng.gen::<u32>();
    let num2 = rng.gen::<u32>();
    let num3 = num1 % 7;

    let blk = block::ActiveBlock::new();
    println!("{:?}", blk);
}
