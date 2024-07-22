-- Add up migration script here

CREATE TABLE "user" (
  "id" serial PRIMARY KEY NOT NULL,
  "name" varchar NOT NULL
);