pub type NTPTimeSourceSubOptType = u16;
pub const NTP_SUBOPTION_SRV_ADDR: NTPTimeSourceSubOptType = 1;	// [RFC5908]
pub const NTP_SUBOPTION_MC_ADDR: NTPTimeSourceSubOptType = 2;	// [RFC5908]
pub const NTP_SUBOPTION_SRV_FQDN: NTPTimeSourceSubOptType = 3;	// [RFC5908]

pub fn to_str(a: NTPTimeSourceSubOptType) -> Result<&'static str, ()> {
  match a {
    1 => Ok("NTP_SUBOPTION_SRV_ADDR"),
    2 => Ok("NTP_SUBOPTION_MC_ADDR"),
    3 => Ok("NTP_SUBOPTION_SRV_FQDN"),
    _ => Err(()),
  }
}
