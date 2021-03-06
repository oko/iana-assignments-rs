pub type HWType = u16;
pub const ETHERNET_10_MB: HWType = 1;	// [Jon_Postel]
pub const EXPERIMENTAL_ETHERNET_3_MB: HWType = 2;	// [Jon_Postel]
pub const AMATEUR_RADIO_AX25: HWType = 3;	// [Philip_Koch]
pub const PROTEON_PRO_NET_TOKEN_RING: HWType = 4;	// [Avri_Doria]
pub const CHAOS: HWType = 5;	// [Gill_Pratt]
pub const IEEE_802_NETWORKS: HWType = 6;	// [Jon_Postel]
pub const ARCNET: HWType = 7;	// [RFC1201]
pub const HYPERCHANNEL: HWType = 8;	// [Jon_Postel]
pub const LANSTAR: HWType = 9;	// [Tom_Unger]
pub const AUTONET_SHORT_ADDRESS: HWType = 10;	// [Mike_Burrows]
pub const LOCAL_TALK: HWType = 11;	// [Joyce_K_Reynolds]
pub const LOCAL_NET_IBM_PC_NET_OR_SYTEK_LOCAL_NET: HWType = 12;	// [Joseph Murdock]
pub const ULTRA_LINK: HWType = 13;	// [Rajiv_Dhingra]
pub const SMDS: HWType = 14;	// [George_Clapp]
pub const FRAME_RELAY: HWType = 15;	// [Andy_Malis]
pub const ASYNCHRONOUS_TRANSMISSION_MODE_ATM: HWType = 16;	// [[JXB2]]
pub const HDLC: HWType = 17;	// [Jon_Postel]
pub const FIBRE_CHANNEL: HWType = 18;	// [RFC4338]
pub const ASYNCHRONOUS_TRANSMISSION_MODE_ATM_1: HWType = 19;	// [RFC2225]
pub const SERIAL_LINE: HWType = 20;	// [Jon_Postel]
pub const ASYNCHRONOUS_TRANSMISSION_MODE_ATM_2: HWType = 21;	// [Mike_Burrows]
pub const MIL_STD_188_220: HWType = 22;	// [Herb_Jensen]
pub const METRICOM: HWType = 23;	// [Jonathan_Stone]
pub const IEEE_13941995: HWType = 24;	// [Myron_Hattig]
pub const MAPOS: HWType = 25;	// [Mitsuru_Maruyama][RFC2176]
pub const TWINAXIAL: HWType = 26;	// [Marion_Pitts]
pub const EUI_64: HWType = 27;	// [Kenji_Fujisawa]
pub const HIPARP: HWType = 28;	// [Jean_Michel_Pittet]
pub const IP_AND_ARP_OVER_ISO_7816_3: HWType = 29;	// [Scott_Guthery]
pub const ARP_SEC: HWType = 30;	// [Jerome_Etienne]
pub const I_PSEC_TUNNEL: HWType = 31;	// [RFC3456]
pub const INFINI_BAND_TM: HWType = 32;	// [RFC4391]
pub const TIA_102_PROJECT_25_COMMON_AIR_INTERFACE_CAI: HWType = 33;	// [Jeff Anderson, Telecommunications Industry of America (TIA) -8.5 Formulating Group, <cja015&motorola.com>, June 2004]
pub const WIEGAND_INTERFACE: HWType = 34;	// [Scott_Guthery_2]
pub const PURE_IP: HWType = 35;	// [Inaky_Perez-Gonzalez]
pub const HW_EXP1: HWType = 36;	// [RFC5494]
pub const HFI: HWType = 37;	// [Tseng-Hui_Lin]
pub const HW_EXP2: HWType = 256;	// [RFC5494]
pub const A_ETHERNET: HWType = 257;	// [Geoffroy_Gramaize]

pub fn to_str(a: HWType) -> Result<&'static str, ()> {
  match a {
    1 => Ok("ETHERNET_10_MB"),
    2 => Ok("EXPERIMENTAL_ETHERNET_3_MB"),
    3 => Ok("AMATEUR_RADIO_AX25"),
    4 => Ok("PROTEON_PRO_NET_TOKEN_RING"),
    5 => Ok("CHAOS"),
    6 => Ok("IEEE_802_NETWORKS"),
    7 => Ok("ARCNET"),
    8 => Ok("HYPERCHANNEL"),
    9 => Ok("LANSTAR"),
    10 => Ok("AUTONET_SHORT_ADDRESS"),
    11 => Ok("LOCAL_TALK"),
    12 => Ok("LOCAL_NET_IBM_PC_NET_OR_SYTEK_LOCAL_NET"),
    13 => Ok("ULTRA_LINK"),
    14 => Ok("SMDS"),
    15 => Ok("FRAME_RELAY"),
    16 => Ok("ASYNCHRONOUS_TRANSMISSION_MODE_ATM"),
    17 => Ok("HDLC"),
    18 => Ok("FIBRE_CHANNEL"),
    19 => Ok("ASYNCHRONOUS_TRANSMISSION_MODE_ATM_1"),
    20 => Ok("SERIAL_LINE"),
    21 => Ok("ASYNCHRONOUS_TRANSMISSION_MODE_ATM_2"),
    22 => Ok("MIL_STD_188_220"),
    23 => Ok("METRICOM"),
    24 => Ok("IEEE_13941995"),
    25 => Ok("MAPOS"),
    26 => Ok("TWINAXIAL"),
    27 => Ok("EUI_64"),
    28 => Ok("HIPARP"),
    29 => Ok("IP_AND_ARP_OVER_ISO_7816_3"),
    30 => Ok("ARP_SEC"),
    31 => Ok("I_PSEC_TUNNEL"),
    32 => Ok("INFINI_BAND_TM"),
    33 => Ok("TIA_102_PROJECT_25_COMMON_AIR_INTERFACE_CAI"),
    34 => Ok("WIEGAND_INTERFACE"),
    35 => Ok("PURE_IP"),
    36 => Ok("HW_EXP1"),
    37 => Ok("HFI"),
    256 => Ok("HW_EXP2"),
    257 => Ok("A_ETHERNET"),
    _ => Err(()),
  }
}
