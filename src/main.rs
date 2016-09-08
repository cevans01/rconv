
extern crate rconv;

fn usage() {
    println!("Usage: main");
}

/*
fn test_conv() {
    println!("Test!");
}
*/

fn main() {

    let args: Vec<_> = std::env::args().collect();


    match args.len() {
        1 => rconv::test_conv(),
        _ => usage()
    }

    let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
    let output = rconv::encode(&input);

    //println!("output is {}", output);
    for x in &output {
        println!("x = {}", x);
    }

}
