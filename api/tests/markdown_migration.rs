#![allow(clippy::redundant_field_names)] // diesel QueryableByName expansion
mod common;

use diesel::sql_types::Text;
use diesel_async::{AsyncPgConnection, RunQueryDsl, SimpleAsyncConnection};

const UP: &str = include_str!("../migrations/w03-content-markdown/up.sql");
const DOWN: &str = include_str!("../migrations/w03-content-markdown/down.sql");

#[derive(diesel::QueryableByName)]
struct Out {
    #[diesel(sql_type = Text, column_name = r)]
    value: String,
}

async fn md(conn: &mut AsyncPgConnection, html: &str) -> String {
    diesel::sql_query("SELECT html_to_md($1) AS r")
        .bind::<Text, _>(html)
        .get_result::<Out>(conn)
        .await
        .unwrap()
        .value
}

async fn content(conn: &mut AsyncPgConnection, name: &str) -> String {
    diesel::sql_query("SELECT content AS r FROM posts WHERE name = $1")
        .bind::<Text, _>(name)
        .get_result::<Out>(conn)
        .await
        .unwrap()
        .value
}

async fn insert(conn: &mut AsyncPgConnection, name: &str, content: &str) {
    diesel::sql_query(
        "INSERT INTO posts (uuid,name,heading,content,created_at) \
         VALUES (gen_random_uuid(),$1,'h',$2,now())",
    )
    .bind::<Text, _>(name)
    .bind::<Text, _>(content)
    .execute(conn)
    .await
    .unwrap();
}

#[actix_web::test]
async fn html_to_md_cases() {
    let Some((_app, env)) = common::test_app().await else {
        return;
    };
    let mut conn = env.pool.get().await.unwrap();
    let c: &mut AsyncPgConnection = &mut conn;
    let cases = [
        (
            "Wir sind <b>PaRaMeRoS</b>, <i>das</i> <a href=\"https://prmrs.de\">Team</a>",
            "Wir sind **PaRaMeRoS**, *das* [Team](https://prmrs.de)",
        ),
        ("a<br>b<br/>c", "a\nb\nc"),
        (
            "<span class=\"x\">Hallo</span> <script>x</script>",
            "Hallo x",
        ),
        ("Kein HTML & so", "Kein HTML & so"),
        ("<p>Eins</p><p>Zwei</p>", "Eins\n\nZwei"),
        ("<b>x\ny</b>", "**x\ny**"),
        (
            "<a href=\"1\">a</a> und <a href=\"2\">b</a>",
            "[a](1) und [b](2)",
        ),
        ("&amp;lt; &lt;b&gt; &quot;&#39;&nbsp;x", "&lt; <b> \"' x"),
        ("<b>1 < 2 > 0</b>", "**1 < 2 > 0**"),
    ];
    for (i, (inp, want)) in cases.iter().enumerate() {
        assert_eq!(&md(c, inp).await, want, "case {i}");
    }
    assert_eq!(
        md(c, "<B>a</B><BR/><A HREF=\"u\">l</A>").await,
        "**a**\n[l](u)"
    );
}

#[actix_web::test]
async fn plain_rows_untouched_and_unbacked() {
    let Some((_app, env)) = common::test_app().await else {
        return;
    };
    let mut conn = env.pool.get().await.unwrap();
    let c: &mut AsyncPgConnection = &mut conn;
    insert(c, "plain", "Kein HTML & so\n&amp; bleibt").await;
    c.batch_execute(DOWN).await.unwrap();
    c.batch_execute(UP).await.unwrap();
    assert_eq!(content(c, "plain").await, "Kein HTML & so\n&amp; bleibt");
    let n: Out = diesel::sql_query(
        "SELECT count(*)::text AS r FROM posts WHERE content_html_backup IS NOT NULL",
    )
    .get_result(c)
    .await
    .unwrap();
    assert_eq!(n.value, "0");
}

#[actix_web::test]
async fn markdown_migration_down_restores_original() {
    let Some((_app, env)) = common::test_app().await else {
        return;
    };
    let mut conn = env.pool.get().await.unwrap();
    let c: &mut AsyncPgConnection = &mut conn;
    let html = "<p>A &amp; B</p>\n<b>x\ny</b><br>&lt;z&gt; <a href=\"u\">l</a>  \n\n";
    c.batch_execute(DOWN).await.unwrap();
    insert(c, "html", html).await;
    insert(c, "plain", "nur Text  \n").await;
    c.batch_execute(UP).await.unwrap();
    assert_ne!(content(c, "html").await, html);
    assert_eq!(content(c, "plain").await, "nur Text  \n");
    c.batch_execute(DOWN).await.unwrap();
    assert_eq!(content(c, "html").await, html);
    assert_eq!(content(c, "plain").await, "nur Text  \n");
    c.batch_execute(UP).await.unwrap(); // leave schema as migrated
}
