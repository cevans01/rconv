
extern crate rconv;

fn main() {

    //let args: Vec<_> = std::env::args().collect();

    let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
    let polys: Vec<u32> = vec![2,0];
    let enc = rconv::Encoder::new(2, 2, input.len() as u32, polys, 0, rconv::EncoderType::ConvTermFlush);
    let output = rconv::encode(enc, &input);

    for x in &input {
        println!("in = {}", x);
    }
    //println!("output is {}", output);
    for x in &output {
        println!("x = {}", x);
    }

}
