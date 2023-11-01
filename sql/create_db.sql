CREATE DATABASE tron;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE wallet_address(
  id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  address text,
  public_key text,
  private_key text
);

CREATE TABLE transactions(
  id text PRIMARY KEY NOT NULL,
  block integer REFERENCES blocks,
  "from" text,
  "to" text,
  raw_amount numeric(72),
  contract_amount numeric(72, 25)
);

