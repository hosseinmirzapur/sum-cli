use clap::Parser;

#[derive(Parser)]
struct Args {
    numbers: String,
}

impl Args {
    fn values(&self) -> Vec<i32> {
        self.numbers
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }
}

fn main() {
    let args = Args::parse();

    let mut sum = 0;

    for num in args.values() {
        sum += num;
    }

    println!(
        "The sum of all these numbers is {} ! Say hello to Rust as well :D",
        sum
    )
}
