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

