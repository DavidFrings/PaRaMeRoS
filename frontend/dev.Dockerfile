### Lockfile stage
FROM cgr.dev/chainguard/node AS lockfile
WORKDIR /app
COPY ./package.json ./
COPY ./pnpm-workspace.yaml ./
RUN npx pnpm install --lockfile-only

### Build stage
FROM cgr.dev/chainguard/node AS builder
WORKDIR /app
COPY ./package.json ./
COPY ./pnpm-workspace.yaml ./
COPY --from=lockfile /app/pnpm-lock.yaml ./
# Cache dependencies (Save time on rebuilds)
COPY --chown=65532:65532 ./tsconfig.json ./tsconfig.node.json ./tsconfig.app.json ./typed-router.d.ts ./env.d.ts ./eslint.config.ts ./vite.config.ts ./
RUN npx pnpm install --frozen-lockfile
# Docker starts here on rebuilds
COPY ./index.html ./
COPY ./public ./public
COPY ./src ./src
RUN npx pnpm run build

### Production stage
FROM cgr.dev/chainguard/nginx:latest-dev
USER nginx
WORKDIR /usr/share/nginx/html
COPY --from=builder /app/dist ./
COPY ./nginx.conf ./mime.types /etc/nginx/
COPY --chown=nginx:nginx  ./docker-env.sh ./docker-env.sh
COPY --chown=nginx:nginx  ./blank ./config.js
RUN chmod u+x ./docker-env.sh
RUN ./docker-env.sh
