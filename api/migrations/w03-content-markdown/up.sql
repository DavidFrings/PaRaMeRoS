ALTER TABLE posts ADD COLUMN content_html_backup VARCHAR;

-- Regex flags: g = all matches, i = case-insensitive. Postgres ARE takes its
-- greediness from the first quantifier, so the link pattern starts with lazy \s+?.
CREATE FUNCTION html_to_md(input text) RETURNS text
LANGUAGE sql IMMUTABLE AS $f$
    SELECT btrim(
        replace(replace(replace(replace(replace(replace(
            regexp_replace(
            regexp_replace(
            regexp_replace(
            regexp_replace(
            regexp_replace(
            regexp_replace(
            regexp_replace(
                input,
                '<br\s*/?>', E'\n', 'gi'),
                '</p\s*>', E'\n\n', 'gi'),
                '<p(\s[^>]*)?>', '', 'gi'),
                '<(b|strong)>(.*?)</\1>', '**\2**', 'gi'),
                '<(i|em)>(.*?)</\1>', '*\2*', 'gi'),
                '<a\s+?[^>]*?href="([^"]*)"[^>]*?>(.*?)</a>', '[\2](\1)', 'gi'),
                '<[a-zA-Z/][^>]*>', '', 'g'),
            '&lt;', '<'), '&gt;', '>'), '&quot;', '"'), '&#39;', ''''), '&nbsp;', ' '),
            '&amp;', '&'),
        E'\n ')
$f$;

UPDATE posts SET content_html_backup = content, content = html_to_md(content)
WHERE content ~ '<[a-zA-Z/]';

UPDATE posts SET content_en = html_to_md(content_en)
WHERE content_en ~ '<[a-zA-Z/]';
