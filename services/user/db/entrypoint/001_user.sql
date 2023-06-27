CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE SCHEMA IF NOT EXISTS "user";

CREATE TABLE IF NOT EXISTS "user"."users" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "id_name" VARCHAR(20) UNIQUE NOT NULL,
  "name" VARCHAR(255) NOT NULL,
  "email" VARCHAR(255) UNIQUE NOT NULL,
  "email_verified" TIMESTAMP,
  "image" VARCHAR(255),
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "user"."accounts" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "user_id" UUID NOT NULL REFERENCES "user"."users"("id"),
  "type" VARCHAR(255) NOT NULL,
  "provider" VARCHAR(255) NOT NULL,
  "provider_account_id" VARCHAR(255) NOT NULL,
  "refresh_token" VARCHAR(255) UNIQUE,
  "access_token" VARCHAR(255) UNIQUE,
  "expires_at" INTEGER,
  "token_type" VARCHAR(255),
  "scope" VARCHAR(255),
  "id_token" VARCHAR(255),
  "session_state" VARCHAR(255),
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "user"."sessions" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "expires" TIMESTAMP NOT NULL,
  "session_token" VARCHAR(255) NOT NULL UNIQUE,
  "user_id" UUID NOT NULL REFERENCES "user"."users"("id"),
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "user"."verification_tokens" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "identifier" VARCHAR(255) NOT NULL,
  "token" VARCHAR(255) NOT NULL UNIQUE,
  "expire" TIMESTAMP NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "user_id" ON "user"."accounts"("user_id");
CREATE INDEX IF NOT EXISTS "provider_account_id" ON "user"."accounts"("provider_account_id");
