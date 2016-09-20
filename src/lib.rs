pub fn test_conv() {
    println!("Test!");
}

pub struct Encoder {
    k:      u32,
    rate:   u32,
    polys:  Vec<u8>,
    start_state:    u8,
    frame_size:     u32,
    partab: Vec<u32>
}

impl Encoder {
    pub fn new(k: u32, rate: u32, polys: Vec<u8>, start_state: u8, frame_size: u32) -> Encoder {

        assert_eq!( rate, polys.len() as u32 );

        // Initialize parity table

        let partab_len: usize = 2u32.pow(k) as usize;
        //let partab_len = std::pow(2, k);
        //let two = 2;
        //let partab_len = two.pow(k);
        let mut partab: Vec<u32> = vec![0; partab_len];

        for i in 0..partab_len {
            let mut cnt = 0;
            let mut ti = i;
            while ti != 0 {
                if (ti & 1) != 0 {
                    cnt += 1;
                }
                ti >>= 1;
            }
            partab[i] = cnt & 1;
        }

        for i in 0..partab_len {
            //println!("partab[{0}] = {1}", i, partab[i]);
        }

        Encoder {
            k: k,
            rate: rate,
            polys: polys,
            start_state: start_state,
            frame_size: frame_size,
            partab: partab
        }
    }

    fn parity(&self, x: u8) -> u32 {
        self.partab[x as usize]
    }

    pub fn encode(&self, input: &Vec<u8>) -> Vec<u8>
    {
        let mut output: Vec<u8> = vec![0; (self.frame_size * self.rate) as usize];
        let mut my_state = self.start_state;
        for i in 0..self.frame_size {
            my_state = (my_state << 1) | (input[i as usize] & 1);
            for j in 0..self.rate {
                if self.parity(my_state & self.polys[j as usize]) == 0 {
                    output[(i * self.rate + j) as usize] = 0;
                } else {
                    output[(i * self.rate + j) as usize] = 1;
                }
            }
        }
        output
    }
}



#[cfg(test)]
mod tests {
    use Encoder;
    #[test]
    fn voyager_code() {
        //g0 = [1,1,1,1,0,0,1] and g1 = [1,0,1,1,0,1,1]
        let polys: Vec<u8> = vec![109,71];
        let input: Vec<u8> = vec![0,1,1,0,1,0,0,0,0,1,1,0,1,0,0,1];
        //let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
        let enc = Encoder::new(7, 2, polys, 3, input.len() as u32);
        let test_output = enc.encode(&input);

        let truth_output: Vec<u8> = vec![
            0,0,1,1,0,1,0,1,1,1,0,1,1,0,0,1,1,1,1,0,1,0,0,1,1,1,0,1,1,0,1,0,0,1,1,0,0,0,0,0,0,1,1,1,0,0];

        println!( "truth.len() = {}", truth_output.len() );
        println!( "test.len()  = {}", test_output.len() );
        assert_eq!( truth_output, test_output );
    }


    #[test]
    fn simple_code() {
        //g0 = [1,1,1,1,0,0,1] and g1 = [1,0,1,1,0,1,1]
        let polys: Vec<u8> = vec![13,11];
        let input: Vec<u8> = vec![1,0,1,1,0];
        //let input: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0, 1];
        let enc = Encoder::new(4, 2, polys, 0, input.len() as u32);
        let test_output = enc.encode(&input);

        /*
        let truth_output: Vec<u8> = vec![
            0,0,1,1,0,1,0,1,1,1,0,1,1,0,0,1,1,1,1,0,1,0,0,1,1,1,0,1,1,0,1,0,0,1,1,0,0,0,0,0,0,1,1,1,0,0];
            */
        let truth_output: Vec<u8> = vec![1,1,0,1,0,1,0,1,1,1,0,1,1,1,0,0];

        println!( "truth.len() = {}", truth_output.len() );
        println!( "test.len()  = {}", test_output.len() );
        assert_eq!( truth_output, test_output );
    }
}

