
//extern crate rconv;

//pub const gsm_conv_rach: super::Encoder = super::Encoder::new(2, 5, 14, vec![23, 33], 0, super::EncoderType::ConvTermFlush);

use super::Encoder;

pub fn gsm_conv_rach() -> Encoder {
    let gsm_conv_rach: Encoder =
            Encoder{ n: 2,
                        k: 5,
                        len: 14,
                        polys: vec![0o23, 0o33],
                        start_state: 0,
                        termination: super::EncoderType::ConvTermFlush
        };
    gsm_conv_rach
}
