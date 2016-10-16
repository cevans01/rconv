
extern crate rconv;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn voyager_code() {
        //g0 = [1,1,1,1,0,0,1] and g1 = [1,0,1,1,0,1,1]
        let polys: Vec<u8> = vec![109,71];
        let input: Vec<u8> = vec![0,1,1,0,1,0,0,0,0,1,1,0,1,0,0,1];
        //let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
        let enc = rconv::Encoder::new(7, 2, polys, 0, input.len() as u32);
        let test_output = enc.encode(&input);

        println!("test_output is = {}", test_output);
        let truth_output: Vec<u8> = vec![
            0,0,1,1,0,1,0,1,1,1,0,1,1,0,0,1,1,1,1,0,1,0,0,1,1,1,0,1,1,0,1,0,0,1,1,0,0,0,0,0,0,1,1,1,0,0];

        assert!( truth_output == test_output );
    }
}
*/


fn usage() {
    println!("Usage: main");
}

fn main() {

    let args: Vec<_> = std::env::args().collect();


    let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
    let polys: Vec<u32> = vec![2,0];
    let enc = rconv::Encoder::new(2, 2, input.len() as u32, polys, 0, rconv::EncoderType::CONV_TERM_FLUSH);
    let output = rconv::encode(enc, &input);

    for x in &input {
        println!("in = {}", x);
    }
    //println!("output is {}", output);
    for x in &output {
        println!("x = {}", x);
    }

}
