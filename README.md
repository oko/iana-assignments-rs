# `iana-assignments-rs`

Library containing generated constants from various IANA numeric registries.

## Usage

Simple:

```
extern crate iana_assignments_rs;

use iana_assignments_rs::dhcpv6::duidtypes;

fn main() {
  println!("Hello, world! {}", duidtypes::DUID_UUID);
}
```
