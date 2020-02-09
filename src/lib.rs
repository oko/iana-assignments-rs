#![no_std]
pub mod dhcpv6;
pub mod arp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
