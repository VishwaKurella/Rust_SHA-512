static INITIAL_BUFFER: [u64; 8] = [
    0x6A09E667F3BCC908u64,
    0xBB67AE8584CAA73Bu64,
    0x3C6EF372FE94F82Bu64,
    0xA54FF53A5F1D36F1u64,
    0x510E527FADE682D1u64,
    0x9B05688C2B3E6C1Fu64,
    0x1F83D9ABFB41BD6Bu64,
    0x5BE0CD19137E2179u64,
];

const K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

fn process_user_input(input: &str) -> Vec<u8> {
    let original_input_bytes = input.as_bytes();
    let original_input_bits = (original_input_bytes.len() * 8) as u128;
    let mut buffer: Vec<u8> = original_input_bytes.to_vec();
    buffer.push(0x80);
    let mut padding_len = 128 - ((buffer.len() + 16) % 128);
    if padding_len == 128 {
        padding_len = 0;
    }
    buffer.extend(std::iter::repeat(0).take(padding_len));
    for i in (0..16).rev() {
        buffer.push(((original_input_bits >> (i * 8)) & 0xFF) as u8);
    }

    buffer
}

fn ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ ((!x) & z)
}

fn maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn sigma0(x: u64) -> u64 {
    x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
}

fn sigma1(x: u64) -> u64 {
    x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
}

fn message_schedule(chunk: &[u8]) -> [u64; 80] {
    let mut schedule: [u64; 80] = [0; 80];
    for i in 0..16 {
        let start_idx = i * 8;
        schedule[i] = u64::from_be_bytes(chunk[start_idx..start_idx + 8].try_into().unwrap());
    }
    for i in 16..80 {
        let s0 = schedule[i - 15].rotate_right(1)
            ^ schedule[i - 15].rotate_right(8)
            ^ (schedule[i - 15] >> 7);
        let s1 = schedule[i - 2].rotate_right(19)
            ^ schedule[i - 2].rotate_right(61)
            ^ (schedule[i - 2] >> 6);

        schedule[i] = schedule[i - 16]
            .wrapping_add(schedule[i - 7])
            .wrapping_add(s0)
            .wrapping_add(s1);
    }

    schedule
}

fn process_buffer_chunk(chunk: &[u8], current_buffer: &[u64; 8]) -> [u64; 8] {
    let message_schedule = message_schedule(chunk);
    let mut a = current_buffer[0];
    let mut b = current_buffer[1];
    let mut c = current_buffer[2];
    let mut d = current_buffer[3];
    let mut e = current_buffer[4];
    let mut f = current_buffer[5];
    let mut g = current_buffer[6];
    let mut h = current_buffer[7];
    for i in 0..80 {
        let temp1 = h
            .wrapping_add(sigma1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(message_schedule[i]);

        let temp2 = sigma0(a).wrapping_add(maj(a, b, c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    let mut result = *current_buffer;
    result[0] = result[0].wrapping_add(a);
    result[1] = result[1].wrapping_add(b);
    result[2] = result[2].wrapping_add(c);
    result[3] = result[3].wrapping_add(d);
    result[4] = result[4].wrapping_add(e);
    result[5] = result[5].wrapping_add(f);
    result[6] = result[6].wrapping_add(g);
    result[7] = result[7].wrapping_add(h);

    result
}

fn process_buffer(buffer: Vec<u8>) -> String {
    let mut current_buffer = INITIAL_BUFFER;
    for chunk in buffer.chunks(128) {
        current_buffer = process_buffer_chunk(chunk, &current_buffer);
    }
    current_buffer
        .iter()
        .map(|x| format!("{:08x}", x))
        .collect()
}

fn main() {
    let input = "a";
    let processed_input_buffer = process_user_input(input);
    let final_hash = process_buffer(processed_input_buffer);
    println!("SHA-512 Hash: {}", final_hash);
}
