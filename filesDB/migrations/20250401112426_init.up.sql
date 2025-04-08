-- Add up migration script here
DROP TYPE IF EXISTS status;

CREATE TYPE status AS ENUM ('pending', 'completed', 'failed');

DROP TABLE IF EXISTS "files";

CREATE TABLE "files" (
  id SERIAL NOT NULL PRIMARY KEY,
  file_name VARCHAR(255) NOT NULL UNIQUE,
  status status NOT NULL,
  compressed_file VARCHAR(255) NOT NULL UNIQUE,
  created_at timestamp with time zone NOT NULL
);
