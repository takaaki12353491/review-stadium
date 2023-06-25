# ---- Base Node ----
FROM node:18-alpine AS base
WORKDIR /app

# Install pnpm
RUN npm install -g pnpm

# ---- Dependencies ----
FROM base AS dependencies
COPY .npmrc ./package.json ./pnpm-lock.yaml ./pnpm-workspace.yaml ./
COPY ./common/nodejs ./common/nodejs
COPY ./front/web ./front/web
RUN pnpm install --frozen-lockfile --prod

# ---- Build ----
FROM dependencies AS build
WORKDIR /app/front/web
RUN pnpm build

# ---- Release ----
FROM base AS release
WORKDIR /app/front/web

# Copy from build image
COPY --from=build /app /app

# Expose port and define CMD
EXPOSE 3000
CMD ["pnpm", "start"]
