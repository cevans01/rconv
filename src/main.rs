
extern crate rconv;

fn usage() {
    println!("Usage: main");
}

fn main() {

    let args: Vec<_> = std::env::args().collect();


    match args.len() {
        1 => rconv::test_conv(),
        _ => usage()
    }

    let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
    let polys: Vec<u8> = vec![2,0];
    let enc = rconv::Encoder::new(2, 2, polys, 0, input.len() as u32);
    let output = enc.encode(&input);

    //println!("output is {}", output);
    for x in &output {
        println!("x = {}", x);
    }

}
