#![feature(proc_macro_hygiene)]
extern crate packros;

#[cfg(test)]
mod tests {
  use packros::packed;

  const ALPHA_ARRAY: &'static [u8; 16] = b"ABCDEFGHIJKLMNOP";
  const ALPHA_INT: u128 = packed!(b"ABCDEFGHIJKLMNOP");
  const HTTP_1_0_ARRAY: &'static [u8; 8] = b"HTTP/1.0";
  const HTTP_1_0_INT: u64 = packed!(b"HTTP/1.0");
  const GET_ARRAY: &'static [u8; 4] = b"GET ";
  const GET_INT: u32 = packed!(b"GET ");
  const TE_ARRAY: &'static [u8; 2] = b"TE";
  const TE_INT: u16 = packed!(b"TE");

  #[test]
  fn it_works() {
    // 64
    let alpha = u128::from_ne_bytes(*ALPHA_ARRAY);
    assert!(alpha == ALPHA_INT);
    // 64
    let http_1_0 = u64::from_ne_bytes(*HTTP_1_0_ARRAY);
    assert!(http_1_0 == HTTP_1_0_INT);
    // 32
    let get = u32::from_ne_bytes(*GET_ARRAY);
    assert!(get == GET_INT);
    // 16
    let te = u16::from_ne_bytes(*TE_ARRAY);
    assert!(te == TE_INT);
    
  }
}

