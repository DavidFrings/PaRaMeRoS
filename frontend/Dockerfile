### Lockfile stage
FROM cgr.dev/chainguard/node AS lockfile
WORKDIR /app
COPY ./package.json ./
COPY ./pnpm-workspace.yaml ./
RUN npx pnpm install --lockfile-only

### Build stage
FROM cgr.dev/chainguard/node AS builder
WORKDIR /app

ARG VITE_NAME
ARG VITE_DESC
ARG VITE_KEYWORDS
ARG VITE_API
ENV VITE_NAME=${VITE_NAME}
ENV VITE_DESC=${VITE_DESC}
ENV VITE_KEYWORDS=${VITE_KEYWORDS}
ENV VITE_API=${VITE_API}

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
FROM cgr.dev/chainguard/nginx
LABEL org.opencontainers.image.source=https://github.com/DavidFrings/PaRaMeRoS
LABEL org.opencontainers.image.description="This is the frontend service for PaRaMeRoS"
LABEL org.opencontainers.image.licenses=MPL-2.0
WORKDIR /usr/share/nginx/html
COPY --from=builder /app/dist ./
COPY ./nginx.conf ./mime.types /etc/nginx/
