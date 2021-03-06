use sponge::*;
use extra::digest::Digest;

pub struct Keccak {
    priv sponge_state: SpongeState,
    priv hash_size: uint,
}

impl Keccak {
    pub fn new(hash_size: uint) -> Keccak {

        let mut sponge = match hash_size {
            0 => SpongeState::new(1024, 576),
            224 => SpongeState::new(1152, 448),
            256 => SpongeState::new(1088, 512),
            384 => SpongeState::new(832, 768),
            512 => SpongeState::new(576, 1024),
            _ => fail!("hash_size must be 0, 224, 256, 384, or 512")
        };

        sponge.fixed_out_len = hash_size;

        Keccak {
            hash_size: hash_size,
            sponge_state: sponge,
        }
    }
}

impl Digest for Keccak {
    pub fn input(&mut self, input: &[u8]) {
        let data_bit_len = input.len() * 8;

        let mut err;

        if data_bit_len % 8 == 0 {
            self.sponge_state.absorb(input, data_bit_len);
        } else {
            err = self.sponge_state.absorb(input, data_bit_len - (data_bit_len % 8));
            if err == Success {
                let last_byte = input[data_bit_len/8] >> (8 - (data_bit_len % 8));
                debug!("Got to second absorb");
                self.sponge_state.absorb(&[last_byte], data_bit_len % 8);
            } else {
                fail!(err.to_str());
            }
        }
    }

    pub fn result(&mut self, out: &mut [u8]) {
//         assert!(out.len() >= self.hash_size);

        self.sponge_state.squeeze(out, self.sponge_state.fixed_out_len);
    }

    pub fn reset(&mut self) {
        self.sponge_state = match self.hash_size {
            0 => SpongeState::new(1024, 576),
            224 => SpongeState::new(1152, 448),
            256 => SpongeState::new(1088, 512),
            384 => SpongeState::new(832, 768),
            512 => SpongeState::new(576, 1024),
            _ => fail!("hash_size must be 0, 224, 256, 384, or 512")
        }
    }

    pub fn output_bits(&self) -> uint {
        self.hash_size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashing() {
        use extra::digest::*;

        let sizes = [224, 256, 384, 512];

        for sizes.iter().advance |n| {
            let mut hasher = Keccak::new(224);

            hasher.input_str("The quick brown fox jumps over the lazy dog");
            let res = hasher.result_str();

            println(res);
        }
    }
}
