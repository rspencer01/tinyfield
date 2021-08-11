#!/usr/bin/env python3


""" An incredibly quick and dirty script to generate the finite fields
in this crate.

Should most likely be replaced by a build script at some point."""

def make_field(p):
    print(f"""
#[derive(Clone, Copy, Debug)]
pub struct GF{p} {{}}
impl PrimeField for GF{p} {{
    const CHARACTERISTIC : u8 = {p};
    const DIVISION_TABLE : [u8; 256] = [""")
    inv = [0 for _ in range(256)]
    for i in range(1,p):
      for j in range(1,p):
        if i*j %p == 1:
          inv[i] = j
          break
    for i in range(16):
        print('        '+', '.join(map(lambda x: '{: 4}'.format(x), inv[i*16: (i+1)*16]))+',')
    print("""    ];
}""")
    print(f"""#[cfg(test)]
mod tests_{p:03} {{
    use crate::GF{p};
    use crate::prime_field::*;

    #[test]
    fn gf{p:03}() {{
        let zero = GF{p}::zero;
        let one = GF{p}::one;
        for x in GF{p}::elts() {{
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }}
        for x in GF{p}::elts() {{
            for y in GF{p}::elts() {{
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }}
        }}
        for x in GF{p}::elts() {{
            assert_eq!(x / x, one);
        }}
        let mut x = zero;
        for i in 0..{p+1} {{
            assert_eq!(x, PrimeFieldElt::from(i % {p}));
            x = x + one;
        }}
        for i in 0..{p} {{
            for j in 0..{p} {{
                if i != j {{
                  assert_ne!(PrimeFieldElt::<GF{p}>::from(i), PrimeFieldElt::from(j));
                }}
            }}
        }}
    }}
}}""")


for p in range(2,256):
  for i in range(2,p//2+1):
    if p%i == 0: break
  else:
    make_field(p)
