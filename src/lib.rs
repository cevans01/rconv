#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn test_conv() {
    println!("Test!");
}

pub fn encode(input: &Vec<u8>) -> Vec<u8>
{
    println!("In encode!");
    input.clone();
}


pub struct Encoder {
    k:      u32,
    rate:   u32,
    polys:  Vec<u8>,
    start_state:    u32,
    partab: Vec<u32>
}

impl Encoder {
    pub fn new(k: u32, rate: u32, polys: Vec<u8>, start_state: u32) -> Encoder {

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
            println!("partab[{0}] = {1}", i, partab[i]);
        }

        Encoder {
            k: k,
            rate: rate,
            polys: polys,
            start_state: start_state,
            partab: partab
        }
    }

}
