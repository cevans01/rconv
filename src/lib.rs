
pub mod codes;

macro_rules! parity {
    ( $x:expr ) => {{
        $x.count_ones() % 2
    }}
}

pub enum EncoderType {
    ConvTermFlush,
    ConvTermTailBiting,
}

pub struct Encoder {
    n:      u32,
    k:      u32,
    len:    u32,
    polys:  Vec<u32>,
    start_state:    u8,
    termination:    EncoderType,
    //partab: Vec<u32>
}

pub fn encode(code: Encoder, input: &Vec<u8>) -> Vec<u8>
{
    let output_len: u32;
    match code.termination {
        EncoderType::ConvTermTailBiting => output_len = code.len * code.n,
        EncoderType::ConvTermFlush =>       output_len = (code.len + code.k - 1) * code.n
    }
    let mut output: Vec<u8> = vec![0; output_len as usize];
    let mut reg:    u32     = 0;

    match code.termination {
        EncoderType::ConvTermTailBiting => {
            for i in 0..(code.k-1) {
                reg |= (input[(code.len - 1 - i) as usize] as u32) << (code.k - 2 - i);
            }
        }
        _ => { }
    }

    for i in 0..code.len as usize {
        reg |= (input[i] as u32) << (code.k-1);
        output[2 * i + 0] = parity!(reg & code.polys[0]) as u8;
        output[2 * i + 1] = parity!(reg & code.polys[1]) as u8;
        reg = reg >> 1;
    }

    // TOOD:
    /*
    match code.termination {
        EncoderType::ConvTermTailBiting => { }
        _ => {
        }
    }
    */

    for i in code.len..(code.len + code.k - 1) {
        output[(2 * i + 0) as usize] = parity!(reg & code.polys[0]) as u8;
        output[(2 * i + 1) as usize] = parity!(reg & code.polys[1]) as u8;
        reg = reg >> 1;
    }

    /*
    let mut my_state = code.start_state;
    for i in 0..code.len {
        my_state = (my_state << 1) | (input[i as usize] & 1);
        for j in 0..code.rate {
            if code.parity(my_state & code.polys[j as usize]) == 0 {
                output[(i * code.rate + j) as usize] = 0;
            } else {
                output[(i * code.rate + j) as usize] = 1;
            }
        }
    }
    */
    output
}

impl Encoder {
    pub fn new(n: u32, k: u32, len: u32, polys: Vec<u32>, start_state: u8, termination: EncoderType) -> Encoder {

        assert_eq!( n, polys.len() as u32 );

        // Initialize parity table

        //let partab_len: usize = 2u32.pow(k) as usize;
        //let partab_len = std::pow(2, k);
        //let two = 2;
        //let partab_len = two.pow(k);
        //let mut partab: Vec<u32> = vec![0; partab_len];

        /*
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
        */

        Encoder {
            k: k,
            n: n,
            polys: polys,
            start_state: start_state,
            len: len,
            termination: termination
            //partab: partab
        }
    }
}

pub fn print_side_by_side(v0: &Vec<u8>, v1: &Vec<u8>) {
	for i in 0..v0.len() {
	    println!( "[{}]   =  {}  =  {} ", i, v0[i], v1[i]);
    }
}

