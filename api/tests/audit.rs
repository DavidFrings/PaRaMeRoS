use parameros_api::client_ip::rate_limit_key_ip;
use std::net::IpAddr;

fn key(s: &str) -> IpAddr {
    rate_limit_key_ip(s.parse().unwrap())
}

#[test]
fn ipv6_same_64_shares_key() {
    assert_eq!(key("2001:db8:1:2::1"), key("2001:db8:1:2:ffff:ffff:ffff:ffff"));
    assert_ne!(key("2001:db8:1:2::1"), key("2001:db8:1:3::1"));
}

#[test]
fn ipv4_and_mapped_share_key() {
    assert_eq!(key("::ffff:10.0.0.1"), key("10.0.0.1"));
    assert_eq!(key("10.0.0.1"), "10.0.0.1".parse::<IpAddr>().unwrap());
}
