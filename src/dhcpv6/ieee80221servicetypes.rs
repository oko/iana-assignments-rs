pub type IEEE80221ServiceType = u16;
pub const IS: IEEE80221ServiceType = 1;	// [RFC5678]
pub const CS: IEEE80221ServiceType = 2;	// [RFC5678]
pub const ES: IEEE80221ServiceType = 3;	// [RFC5678]

pub fn to_str(a: IEEE80221ServiceType) -> Result<&'static str, ()> {
  match a {
    1 => Ok("IS"),
    2 => Ok("CS"),
    3 => Ok("ES"),
    _ => Err(()),
  }
}
