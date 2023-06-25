CREATE SCHEMA IF NOT EXISTS user;

CREATE TABLE IF NOT EXISTS user.users (
  id SERIAL PRIMARY KEY,
  id_name VARCHAR(20) NOT NULL,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  email_verified TIMESTAMP,
  image VARCHAR(255),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user.accounts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES user.users(id),
  type VARCHAR(255) NOT NULL,
  provider VARCHAR(255) NOT NULL,
  provider_account_id VARCHAR(255) NOT NULL,
  refresh_token VARCHAR(255),
  access_token VARCHAR(255),
  expires_at INTEGER,
  token_type VARCHAR(255),
  scope VARCHAR(255),
  id_token VARCHAR(255),
  session_state VARCHAR(255),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user.sessions (
  id SERIAL PRIMARY KEY,
  expires TIMESTAMP NOT NULL,
  session_token VARCHAR(255) NOT NULL,
  user_id INTEGER NOT NULL REFERENCES user.users(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user.verification_tokens (
  identifier VARCHAR(255) NOT NULL,
  token VARCHAR(255) NOT NULL,
  expire TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS email ON user.users(email);
CREATE INDEX IF NOT EXISTS user_id ON user.accounts(user_id);
CREATE INDEX IF NOT EXISTS provider_account_id ON user.accounts(provider_account_id);
CREATE UNIQUE INDEX IF NOT EXISTS session_token ON user.sessions(session_token);
CREATE UNIQUE INDEX IF NOT EXISTS access_token ON user.sessions(access_token);
CREATE UNIQUE INDEX IF NOT EXISTS token ON user.verification_tokens(token);
