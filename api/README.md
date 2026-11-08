# API Routes

| Handler module | Description                   | Method | Path          | Auth required | Request                                                                        | Response (what is returned)                                                                   |
|----------------|-------------------------------|--------|---------------|---------------|--------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------|
| register       | Register a new user           | POST   | /register     | Admin         | { "username", "password", Option(role), "admin_pass" } (role defaults to `normal`; other fields are rejected) | 201: Created (400 `Unknown role`)                                                                                  |
| auth           | Authenticate user, return JWT | POST   | /auth         | user & pass   | { "username", "password" }                                                     | 200: Header: Authorization - Bearer                                                           |
| verify auth    | Verify JWT / session          | GET    | /auth/verify  | Token         | -                                                                              | 200: { role, permissions(string[]), can_set_created_at(bool) } (permissions derived from the role)                             |
| posts          | List posts                    | GET    | /posts/{name} | No            | -                                                                              | 200: Array of ⬇                                                                               |
| post           | Get a single post by id       | GET    | /post/{uuid}  | No            | -                                                                              | 200: { "name", "heading", "content", "media_type", "media_name", "media_creator", "created_at", "updated_at", Option("heading_en"), Option("content_en"), Option("season") } |
| new post       | Create a new post             | POST   | /post         | Section         | Multipart: { "name", "heading", "content", Option(heading_en), Option(content_en), Option(season), Option(created_at), Option(media_type + media + media_creator) } | 200: { "id" }                                                                               |
| update post    | Update a post by id           | PUT    | /post/{uuid}  | Section         | { Option("name"), Option("heading"), Option("content"), Option("heading_en"), Option("content_en"), Option("season"), Option("created_at") } ("" clears heading_en/content_en/season) | 202: Accepted (404 if unknown) |
| delete post    | Delete a post by id           | DELETE | /post/{uuid}  | Section         | -                                                                              | 202: Accepted                                                                                 |
| replace media  | Replace a post's media       | PUT    | /post/{uuid}/media | Section    | Multipart: { media_type ("img"/"vid"), media_creator (required, non-empty), media } (image ≤ 15 MiB JPEG/PNG/WebP ≤ 40 MP, video ≤ 100 MiB MP4/WebM) | 200: { "media_name" } |
| delete media   | Remove a post's media        | DELETE | /post/{uuid}/media | Section    | -                                                                              | 204: No Content |
| tournament     | Get the tournament           | GET    | /tournament   | No            | -                                                                              | 200: { name, place, starts_at, ends_at } or 204: No Content (none set)                        |
| set tournament | Create/replace the tournament | PUT   | /tournament   | `tournament`         | { "name", Option(place), "starts_at", Option(ends_at) }                         | 200: stored tournament                                                                        |
| delete tournament | Delete the tournament     | DELETE | /tournament   | `tournament`         | -                                                                              | 204: No Content                                                                               |
| sitemap        | Sitemap with hreflang        | GET    | /sitemap.xml  | No            | -                                                                              | 200: XML                                                                                      |

`/posts/progress-logs` is an alias of `/posts/logbuch`.

Roles (`users.role`) decide what a user may write:

| role     | may write                                               |
|----------|---------------------------------------------------------|
| `normal` | `logbuch`, `team`, `coaches`, `tournament`              |
| `forest` | everything `normal` may, plus `forest-project`          |
| `super`  | everything, and may set a post's `created_at`           |

Internally the role is mapped to the permissions `logbuch`, `team`, `coaches`, `forest-project`, `tournament` (section permissions equal post `name` values; `progress-logs` maps to `logbuch`). "Section" writes need the permission of the post's section: on POST the new `name`, on PUT/DELETE the post's current section, and on a PUT that changes `name` also the new one. A missing permission returns 403 ("Missing permission"); an unknown uuid stays 404, checked before the permission. Reads are public.

`created_at` (POST /post and PUT /post/{uuid}, `super` only, else 403) is RFC 3339 with an offset, e.g. `2024-01-28T10:00:00+01:00`, and is stored as UTC. Invalid values, years before 2017 and times more than one day ahead return 400 (`Invalid created_at`). An empty value is ignored (POST: now). Changing only `created_at` leaves `updated_at` untouched.

To change a user's role, update the row directly; it applies to the user's next request, even with an existing token:

```sql
UPDATE users SET role = 'normal' WHERE username = '…';
```

Migration `w06-user-roles` replaces `admin`/`permissions` with `role` and makes **every existing account `super`**; narrow them after deploy.

`media_creator` is required together with media (POST /post and PUT /post/{uuid}/media: 400 if missing). On POST /post a creator without media is ignored.
