pub type OptLQQueryType = u8;
pub const QUERY_BY_ADDRESS: OptLQQueryType = 1;	// [RFC5007]
pub const QUERY_BY_CLIENTID: OptLQQueryType = 2;	// [RFC5007]
pub const QUERY_BY_RELAY_ID: OptLQQueryType = 3;	// [RFC5460]
pub const QUERY_BY_LINK_ADDRESS: OptLQQueryType = 4;	// [RFC5460]
pub const QUERY_BY_REMOTE_ID: OptLQQueryType = 5;	// [RFC5460]
