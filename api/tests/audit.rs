use parameros_api::client_ip::rate_limit_key_ip;
use std::net::IpAddr;

fn key(s: &str) -> IpAddr {
    rate_limit_key_ip(s.parse().unwrap())
}

#[test]
fn ipv6_same_64_shares_key() {
    assert_eq!(
        key("2001:db8:1:2::1"),
        key("2001:db8:1:2:ffff:ffff:ffff:ffff")
    );
    assert_ne!(key("2001:db8:1:2::1"), key("2001:db8:1:3::1"));
}

#[test]
fn ipv4_and_mapped_share_key() {
    assert_eq!(key("::ffff:10.0.0.1"), key("10.0.0.1"));
    assert_eq!(key("10.0.0.1"), "10.0.0.1".parse::<IpAddr>().unwrap());
}

#[test]
fn remove_media_files_rejects_traversal() {
    let root = tempfile::tempdir().unwrap();
    let up = root.path().join("up");
    std::fs::create_dir(&up).unwrap();
    std::fs::write(root.path().join("x.webp"), b"x").unwrap();
    std::fs::write(root.path().join("x"), b"x").unwrap();
    for name in ["../x.webp", "../x", "a/b.webp", "a\\b", ""] {
        parameros_api::media::files::remove_media_files(&up, name);
    }
    assert!(root.path().join("x.webp").exists());
    assert!(root.path().join("x").exists());
}

#[actix_web::test]
async fn backfill_skips_oversized_legacy_image() {
    let dir = tempfile::tempdir().unwrap();
    let img = image::RgbImage::new(8000, 6000);
    let webp = webp::Encoder::from_rgb(&img, 8000, 6000).encode(10.0);
    std::fs::write(dir.path().join("huge.webp"), &*webp).unwrap();
    parameros_api::media::backfill::backfill(dir.path().to_path_buf()).await;
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[actix_web::test]
async fn db_error_does_not_leak_details() {
    use actix_web::web::Data;
    use diesel_async::{
        AsyncPgConnection,
        pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
    };
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        "postgres://secretuser:secretpass@127.0.0.1:1/db",
    );
    let pool = Pool::builder()
        .connection_timeout(std::time::Duration::from_millis(500))
        .build_unchecked(manager);
    let env = Data::new(parameros_api::Env {
        pool,
        jwt_secret: String::new(),
        jwt_expire: 1,
        admin_pass: String::new(),
        cors_origins: vec![],
        upload_dir: std::path::PathBuf::new(),
        redis: None,
        site_url: String::new(),
    });
    let err = parameros_api::utils::db(&env)
        .await
        .err()
        .expect("must fail");
    assert_eq!(err.to_string(), "Database error");
}
