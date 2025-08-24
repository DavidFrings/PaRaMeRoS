# PaRaMeRoS

## Project Setup

```sh
pnpm install
```

### Compile and Hot-Reload for Development

```sh
pnpm dev
```

### Type-Check, Compile and Minify for Production

```sh
pnpm build
```

### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```

| Methode | Pfad | Erwarteter Request | Erwartete Response | Auth | Verwendungsort im Frontend |
|---|---|---|---|---|---|
| POST | /auth | JSON: `{ username: string, password: string }` | String-Token (JWT) oder `{ token: string }` – der Wert wird als Cookie `auth_token` gesetzt | Nein | `frontend/src/pages/login.vue` (handleSubmit) |
| POST | /auth/verify | JSON: `{ token: string }` | JSON: `{ user: boolean, admin: boolean }` | Nein (Token im Body) | `frontend/src/components/Navbar.vue` (onMounted) |
| GET | /progress-logs | — | JSON: `Post[]` | Optional (öffentlich) | `frontend/src/pages/progress-logs/index.vue` via `<Cards name="progress-logs" />` |
| GET | /team | — | JSON: `Post[]` | Optional (öffentlich) | `frontend/src/pages/team/index.vue` via `<Cards name="team" />` |
| GET | /forest-project | — | JSON: `Post[]` | Optional (öffentlich) | `frontend/src/pages/prmrs/forest-project/index.vue` via `<Cards name="forest-project" />` |
| POST | /newPost | multipart/form-data: `name`: "progress-logs" | "team" | "forest-project"; `token`: string; `heading`: string; `content`: string; `mediaName`?: string; `media`?: File | JSON: `{ id: string }` (wird für Redirect auf `/post/{id}` genutzt) | Ja (token erforderlich) | `frontend/src/components/createPost.vue`; genutzt in den `.../create`-Seiten |
