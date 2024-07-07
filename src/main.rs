use std::convert::TryInto;

const BLOCK_SIZE: usize = 32; // Size of each block in bits
const HASH_SIZE: usize = 32; // Size of the hash code in bits

struct XorHasher {
    state: [u8; HASH_SIZE],
    block_count: usize,
}

impl XorHasher {
    fn new() -> Self {
        XorHasher {
            state: [0; HASH_SIZE],
            block_count: 0,
        }
    }

    // The previous update function was calculating the XOR of individual bits in sets of
    // 32 (BLOCK_SIZE). To simplify this, we can group the bits that have the same "i % BLOCK_SIZE" and perform their XOR.

    // However, since we don't want to use bitwise operations, we can utilize the special properties of XOR. For a group of bits, we can simply check the sum: if the sum is even, then the
    // resultant bit in the final result of this group is going to be zero.

    // For example: 1 ^ 1 ^ 1 ^ 1 ^ 0 ^ 1 ^ 0 ^ 0 ^ 0 ^ 1 = 1 (sum is odd)
    //              1 ^ 1 ^ 1 ^ 1 ^ 1 ^ 0 = 0 (sum is even)

    // But calculating the sum can cause number overflow in state[i % BLOCK_SIZE].
    // Therefore, we can rely on parity: two numbers with the same parity always result in an even sum.

    // I have combined all of this into the Update() function.


    fn update(&mut self, data: &[u8]) {
        for i in 0..data.len() {
            if self.state[i % BLOCK_SIZE]%2 == data[i]%2 {
                self.state[i % BLOCK_SIZE] = 0;
            } else {
                self.state[i%BLOCK_SIZE] = 1;
            }
        }

        self.block_count = (data.len() + BLOCK_SIZE)/BLOCK_SIZE;
    }

    fn finalize(self) -> [u8; HASH_SIZE] {
        self.state
    }
}

fn xor_hash(data: &[u8]) -> [u8; HASH_SIZE] {
    let mut hasher = XorHasher::new();
    hasher.update(data);
    hasher.finalize()
}

fn xor_hash_attack(data: &[u8]) -> Vec<u8> {
    let mut padded_data = Vec::new();
    let r = BLOCK_SIZE - (data.len() % BLOCK_SIZE);

    if r != 0 {
        let padding = vec![0; r];
        padded_data.extend_from_slice(data);
        padded_data.extend(padding);
    }
    let mut mathcing_message = Vec::new();

    for _ in 1..=3 {
        mathcing_message.extend_from_slice(&padded_data);
    }
    mathcing_message
}

#[cfg(test)]
mod tests {

    use quickcheck::QuickCheck;

    use super::*;

    #[test]
    fn test_xor_attack() {
        fn prop(data: Vec<u8>) -> bool {
            xor_hash(&data) == xor_hash(&xor_hash_attack(&data))
        }
        QuickCheck::new().quickcheck(prop as fn(Vec<u8>) -> bool);
    }

    #[test]
    fn attack_demo() {
        let data = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
        println!("{}", data.len());
        let attack = xor_hash_attack(&data);
        println!("{:?}", attack.len());
        println!("{:?}", xor_hash(&data));
        println!("{:?}", xor_hash(&attack));
    }
}

fn main() {
    println!("Hello, world!");
}
