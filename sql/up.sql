CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  username VARCHAR(255) NOT NULL,
  email VARCHAR(260) UNIQUE NOT NULL,
  password TEXT NOT NULL
);

-- always search email first and then password
CREATE INDEX IF NOT EXISTS user_creds_idx ON users (email, password);