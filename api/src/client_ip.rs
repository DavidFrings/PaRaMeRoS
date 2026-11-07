use actix_web::HttpRequest;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// `X-Real-IP` (set by the reverse proxy) if valid, else the peer address, else `0.0.0.0`.
pub fn client_ip(req: &HttpRequest) -> IpAddr {
    req.headers()
        .get("X-Real-IP")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse().ok())
        .or_else(|| req.peer_addr().map(|a| a.ip()))
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
}

/// Rate-limit key: IPv4 as-is, IPv4-mapped IPv6 as IPv4, other IPv6 masked to its /64
/// (a single client usually owns a whole /64, so per-address keys would be trivially rotated).
pub fn rate_limit_key_ip(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V4(_) => ip,
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => IpAddr::V4(v4),
            None => {
                let s = v6.segments();
                IpAddr::V6(Ipv6Addr::new(s[0], s[1], s[2], s[3], 0, 0, 0, 0))
            }
        },
    }
}
