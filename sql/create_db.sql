CREATE DATABASE tron;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE wallet_address (
	id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  address TEXT,
  public_key TEXT,
  private_key TEXT
);
