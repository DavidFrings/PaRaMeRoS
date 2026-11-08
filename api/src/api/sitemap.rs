use crate::db::schema::posts::dsl::{created_at, posts, updated_at, uuid};
use crate::redis_store::{CACHE_TTL_SECS, cache_get, cache_set};
use crate::utils::{HttpError, db};
use crate::{Env, query};
use actix_web::{HttpResponse, get, web::Data};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::fmt::Write as _;

const FIXED: [&str; 7] = [
    "/",
    "/About",
    "/team",
    "/logbuch",
    "/prmrs/forest-project",
    "/Imprint",
    "/Privacy-Policy",
];
const CACHE_KEY: &str = "cache:sitemap";

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn url_entry(out: &mut String, base: &str, path: &str, lastmod: Option<&str>) {
    let de = esc(&format!("{base}{path}"));
    let en = esc(&format!(
        "{base}/en{}",
        if path == "/" { "/" } else { path }
    ));
    for loc in [&de, &en] {
        // Writing to a String cannot fail, so the fmt::Result is ignored.
        let _ = writeln!(out, "  <url>\n    <loc>{loc}</loc>");
        if let Some(d) = lastmod {
            let _ = writeln!(out, "    <lastmod>{d}</lastmod>");
        }
        for (lang, href) in [("de", &de), ("en", &en), ("x-default", &de)] {
            let _ = writeln!(
                out,
                "    <xhtml:link rel=\"alternate\" hreflang=\"{lang}\" href=\"{href}\"/>"
            );
        }
        out.push_str("  </url>\n");
    }
}

#[get("/sitemap.xml")]
pub async fn sitemap(env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let xml = if let Some(xml) = cache_get(&env, CACHE_KEY).await {
        xml
    } else {
        let mut conn = db(&env).await?;
        let rows: Vec<(
            ::uuid::Uuid,
            chrono::NaiveDateTime,
            Option<chrono::NaiveDateTime>,
        )> = query!(
            posts
                .select((uuid, created_at, updated_at))
                .order((created_at.desc(), uuid.asc()))
                .load(&mut conn)
        );

        let base = env.site_url.strip_suffix('/').unwrap_or(&env.site_url);
        let mut xml = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
        );
        for p in FIXED {
            url_entry(&mut xml, base, p, None);
        }
        for (id, created, updated) in rows {
            let date = updated.unwrap_or(created).format("%Y-%m-%d").to_string();
            url_entry(&mut xml, base, &format!("/post/{id}"), Some(&date));
        }
        xml.push_str("</urlset>\n");
        cache_set(&env, CACHE_KEY, &xml, CACHE_TTL_SECS).await;
        xml
    };
    Ok(HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(xml))
}
