use actix_web::HttpRequest;
use std::net::{IpAddr, Ipv4Addr};

/// `X-Real-IP` (set by the reverse proxy) if valid, else the peer address, else `0.0.0.0`.
pub fn client_ip(req: &HttpRequest) -> IpAddr {
    req.headers()
        .get("X-Real-IP")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse().ok())
        .or_else(|| req.peer_addr().map(|a| a.ip()))
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
}
