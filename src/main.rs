
use std::error::Error;
use std::fs::File;

mod learning;

#[allow(dead_code)]
fn files_option() -> Result<(), Box<dyn Error>> {
    let _f = File::open("hello.txt")?;
    Ok(())
}

fn main() {
    // guessing();
    // files_option().expect("TODO: panic message");

    // generics();

    traits();
}


#[allow(dead_code)]
fn traits(){
    #[allow(unused_imports)]
    use learning::traits::*;

    dosth();
}


#[allow(dead_code)]
fn generics() {
    use learning::generices::*;

    let number_list = vec![34, 50, 25, 100, 65];
    max_numbers(&number_list);
    dbg!(largest(&number_list));
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    max_numbers(&number_list);
    dbg!(largest(&number_list));

    dbg!(Point{
        x: 140,
        y: 150,
    }.x());
    dbg!(Point{
        x: 34.4,
        y: 40.9
    }.intval());
}


#[allow(dead_code)]
fn guessing() {
    use learning::guessing::*;

    let guess: Guess = Guess::new(99);
    dbg!(guess.value());
    dosth(guess);
}
