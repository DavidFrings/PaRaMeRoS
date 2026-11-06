UPDATE posts SET content = content_html_backup WHERE content_html_backup IS NOT NULL;
ALTER TABLE posts DROP COLUMN content_html_backup;
DROP FUNCTION html_to_md(text);
