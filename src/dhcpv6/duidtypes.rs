pub type DuidType = u16;
pub const DUID_LLT: DuidType = 1;	// [RFC8415]
pub const DUID_EN: DuidType = 2;	// [RFC8415]
pub const DUID_LL: DuidType = 3;	// [RFC8415]
pub const DUID_UUID: DuidType = 4;	// [RFC6355]

pub fn to_str(a: DuidType) -> Result<&'static str, ()> {
  match a {
    1 => Ok("DUID_LLT"),
    2 => Ok("DUID_EN"),
    3 => Ok("DUID_LL"),
    4 => Ok("DUID_UUID"),
    _ => Err(()),
  }
}