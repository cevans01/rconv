extern crate rconv;

#[cfg(test)]
mod tests {
    use rconv::encode;
    use rconv::print_side_by_side;
    use rconv::codes;
    /*
    #[test]
    fn voyager_code() {
        //g0 = [1,1,1,1,0,0,1] and g1 = [1,0,1,1,0,1,1]
        let polys: Vec<u8> = vec![109,71];
        let input: Vec<u8> = vec![0,1,1,0,1,0,0,0,0,1,1,0,1,0,0,1];
        //let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
        let enc = Encoder::new(7, 2, polys, 0, input.len() as u32);
        //let test_output = enc.encode(&input);
        let mut test_output = encode(enc, &input);

        println!("test_output is = {}", test_output);
        let truth_output: Vec<u8> = vec![
            0,0,1,1,0,1,0,1,1,1,0,1,1,0,0,1,1,1,1,0,1,0,0,1,1,1,0,1,1,0,1,0,0,1,1,0,0,0,0,0,0,1,1,1,0,0];

        assert!( truth_output == test_output );
    }
    */

    #[test]
    fn gsm_rach_code() {
        //let polys: Vec<u32> = vec![23,33];
        let mut input: Vec<u8> = vec![0; 14];
		input[0]=   1;
		input[1]=   0;
		input[2]=   0;
		input[3]=   0;
		input[4]=   0;
		input[5]=   0;
		input[6]=   0;
		input[7]=   0;
		input[8]=   0;
		input[9]=   0;
		input[10]=   0;
		input[11]=   0;
		input[12]=   0;
		input[13]=   0;

        //let k: u32 = 5;
        let enc = codes::gsm_conv_rach();
        let test_output = encode(enc, &input);

        let mut truth_output: Vec<u8> = vec![0; 36];
		truth_output[0]=  1;
		truth_output[1]=  1;
		truth_output[2]=  0;
		truth_output[3]=  1;
		truth_output[4]=  0;
		truth_output[5]=  0;
		truth_output[6]=  1;
		truth_output[7]=  1;
		truth_output[8]=  1;
		truth_output[9]=  1;
		truth_output[10]=  0;
		truth_output[11]=  0;
		truth_output[12]=  0;
		truth_output[13]=  0;
		truth_output[14]=  0;
		truth_output[15]=  0;
		truth_output[16]=  0;
		truth_output[17]=  0;
		truth_output[18]=  0;
		truth_output[19]=  0;
		truth_output[20]=  0;
		truth_output[21]=  0;
		truth_output[22]=  0;
		truth_output[23]=  0;
		truth_output[24]=  0;
		truth_output[25]=  0;
		truth_output[26]=  0;
		truth_output[27]=  0;
		truth_output[28]=  0;
		truth_output[29]=  0;
		truth_output[30]=  0;
		truth_output[31]=  0;
		truth_output[32]=  0;
		truth_output[33]=  0;
		truth_output[34]=  0;
		truth_output[35]=  0;


        print_side_by_side(&truth_output, &test_output);

        /*
        let xor_vec: Vec<u8> = (0..truth_output.len()).map(|i| truth_output[i] ^ test_output[i]).collect();
        for x in &xor_vec {
            println!( "xor = {}", x );
        }
        */

        println!( "truth.len() = {}", truth_output.len() );
        println!( "test.len()  = {}", test_output.len() );
        //assert_eq!( truth_output, test_output );
        assert_eq!( truth_output, test_output );
    }


}

