use lot::Quote;

fn main() {
    println!("{}", Quote::try_from("Lorem ipsum dolor sit amet").unwrap());
}
