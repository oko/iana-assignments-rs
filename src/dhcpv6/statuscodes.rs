pub type StatusCode = u16;
pub const SUCCESS: StatusCode = 0;	// [RFC8415]
pub const UNSPEC_FAIL: StatusCode = 1;	// [RFC8415]
pub const NO_ADDRS_AVAIL: StatusCode = 2;	// [RFC8415]
pub const NO_BINDING: StatusCode = 3;	// [RFC8415]
pub const NOT_ON_LINK: StatusCode = 4;	// [RFC8415]
pub const USE_MULTICAST: StatusCode = 5;	// [RFC8415]
pub const NO_PREFIX_AVAIL: StatusCode = 6;	// [RFC3633][RFC8415]
pub const UNKNOWN_QUERY_TYPE: StatusCode = 7;	// [RFC5007]
pub const MALFORMED_QUERY: StatusCode = 8;	// [RFC5007]
pub const NOT_CONFIGURED: StatusCode = 9;	// [RFC5007]
pub const NOT_ALLOWED: StatusCode = 10;	// [RFC5007]
pub const QUERY_TERMINATED: StatusCode = 11;	// [RFC5460]
pub const DATA_MISSING: StatusCode = 12;	// [RFC7653]
pub const CATCH_UP_COMPLETE: StatusCode = 13;	// [RFC7653]
pub const NOT_SUPPORTED: StatusCode = 14;	// [RFC7653]
pub const TLS_CONNECTION_REFUSED: StatusCode = 15;	// [RFC7653]
pub const ADDRESS_IN_USE: StatusCode = 16;	// [RFC8156]
pub const CONFIGURATION_CONFLICT: StatusCode = 17;	// [RFC8156]
pub const MISSING_BINDING_INFORMATION: StatusCode = 18;	// [RFC8156]
pub const OUTDATED_BINDING_INFORMATION: StatusCode = 19;	// [RFC8156]
pub const SERVER_SHUTTING_DOWN: StatusCode = 20;	// [RFC8156]
pub const DNS_UPDATE_NOT_SUPPORTED: StatusCode = 21;	// [RFC8156]
pub const EXCESSIVE_TIME_SKEW: StatusCode = 22;	// [RFC8156]

pub fn to_str(a: StatusCode) -> Result<&'static str, ()> {
  match a {
    0 => Ok("SUCCESS"),
    1 => Ok("UNSPEC_FAIL"),
    2 => Ok("NO_ADDRS_AVAIL"),
    3 => Ok("NO_BINDING"),
    4 => Ok("NOT_ON_LINK"),
    5 => Ok("USE_MULTICAST"),
    6 => Ok("NO_PREFIX_AVAIL"),
    7 => Ok("UNKNOWN_QUERY_TYPE"),
    8 => Ok("MALFORMED_QUERY"),
    9 => Ok("NOT_CONFIGURED"),
    10 => Ok("NOT_ALLOWED"),
    11 => Ok("QUERY_TERMINATED"),
    12 => Ok("DATA_MISSING"),
    13 => Ok("CATCH_UP_COMPLETE"),
    14 => Ok("NOT_SUPPORTED"),
    15 => Ok("TLS_CONNECTION_REFUSED"),
    16 => Ok("ADDRESS_IN_USE"),
    17 => Ok("CONFIGURATION_CONFLICT"),
    18 => Ok("MISSING_BINDING_INFORMATION"),
    19 => Ok("OUTDATED_BINDING_INFORMATION"),
    20 => Ok("SERVER_SHUTTING_DOWN"),
    21 => Ok("DNS_UPDATE_NOT_SUPPORTED"),
    22 => Ok("EXCESSIVE_TIME_SKEW"),
    _ => Err(()),
  }
}
