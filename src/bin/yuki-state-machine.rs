use yuki_state_machine::cat::mischievous::MischievousCat;

fn main() {
    let yuki = MischievousCat::new("Yuki".to_string());
    println!("{}", yuki.describe());

    let yuki = yuki.scare();
    println!("{}", yuki.describe());

    let yuki = yuki.pass_time();
    println!("{}", yuki.describe());

    let yuki = yuki.over_cuddle();
    println!("{}", yuki.describe());

    let yuki = yuki.forget_to_feed();
    println!("{}", yuki.describe());

    let yuki = yuki.feed();
    println!("{}", yuki.describe());

    let yuki = yuki.sleep();
    println!("{}", yuki.describe());

    let yuki = yuki.scare();
    println!("{}", yuki.describe());

    let yuki = yuki.pass_time();
    println!("{}", yuki.describe());

    let yuki = yuki.gentle_cuddle();
    println!("{}", yuki.describe());
}
