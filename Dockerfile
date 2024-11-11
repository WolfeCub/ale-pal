FROM rust:latest AS rbuilder
WORKDIR /api
COPY api .
ENV DATABASE_URL=sqlite://alepal.db
RUN cargo install sqlx-cli
RUN sqlx database create
RUN sqlx migrate run
RUN cargo build --release
RUN strip target/release/ale-pal

FROM node:20-slim AS jbuilder
WORKDIR /ui
COPY ui .
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
RUN pnpm install
RUN pnpm run build

FROM gcr.io/distroless/cc-debian12:latest AS release
WORKDIR /app
COPY --from=rbuilder /api/target/release/ale-pal .
COPY --from=rbuilder /api/alepal.db .
COPY --from=jbuilder /ui/dist/ dist/

EXPOSE 8080

CMD ["./ale-pal"]
