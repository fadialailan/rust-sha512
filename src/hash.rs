use crate::operations;
use operations::sha::*;

const H_DEFAULT: [u64; 8] = [
    0x6a09e667f3bcc908u64,
    0xbb67ae8584caa73bu64,
    0x3c6ef372fe94f82bu64,
    0xa54ff53a5f1d36f1u64,
    0x510e527fade682d1u64,
    0x9b05688c2b3e6c1fu64,
    0x1f83d9abfb41bd6bu64,
	0x5be0cd19137e2179u64,
];

const K:[u64;80] = [
0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
];


pub fn hash(data:&[u64]) -> [u64;8] {
    let mut hash_array: [u64; 8] = H_DEFAULT.clone();
    let mut w_array: [u64;80] = [0;80];
    
    for loop_iterator1 in (0..data.len()).step_by(16) {
        for loop_iterator2 in 0..80 {
            if loop_iterator2 <= 15 {
                w_array[loop_iterator2] = data[loop_iterator1+loop_iterator2];
            } else {
                w_array[loop_iterator2] = sigma1(w_array[loop_iterator2-2]).wrapping_add(w_array[loop_iterator2-7]).wrapping_add(sigma0(w_array[loop_iterator2-15])).wrapping_add(w_array[loop_iterator2-16]);
            }
        }
        let mut a:u64 = hash_array[0];
        let mut b:u64 = hash_array[1];
        let mut c:u64 = hash_array[2];
        let mut d:u64 = hash_array[3];
        let mut e:u64 = hash_array[4];
        let mut f:u64 = hash_array[5];
        let mut g:u64 = hash_array[6];
        let mut h:u64 = hash_array[7];

        
        for t in 0..80 {
            let t1:u64 = h.wrapping_add(bsigma1(e)).wrapping_add(ch(e, f, g)).wrapping_add(K[t]).wrapping_add(w_array[t]);
            let t2:u64 = bsigma0(a).wrapping_add(maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        hash_array[0] = hash_array[0].wrapping_add(a);
        hash_array[1] = hash_array[1].wrapping_add(b);
        hash_array[2] = hash_array[2].wrapping_add(c);
        hash_array[3] = hash_array[3].wrapping_add(d);
        hash_array[4] = hash_array[4].wrapping_add(e);
        hash_array[5] = hash_array[5].wrapping_add(f);
        hash_array[6] = hash_array[6].wrapping_add(g);
        hash_array[7] = hash_array[7].wrapping_add(h);

    }

    return hash_array;
}

pub fn sha512(data:&mut Vec<u8>) -> [u64; 8] {
    operations::print_hex2(data);
    println!("");

	operations::padding(data);
    operations::print_hex2(data);
    println!("");

	let blocks = operations::parsing(data);
    operations::print_hex(&blocks);
    println!("");

    let final_hash = hash(&blocks);

    return final_hash; 

}