-- Initial migration
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  display_name VARCHAR(255) NOT NULL
);

CREATE TABLE stories (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  text TEXT,
  url TEXT,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  author_id INTEGER NOT NULL REFERENCES users(id)
);

INSERT INTO users (display_name) VALUES ('LambdaFunction');
