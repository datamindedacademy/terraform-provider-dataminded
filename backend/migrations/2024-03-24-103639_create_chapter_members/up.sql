CREATE TABLE chapter_members (
  id INTEGER PRIMARY KEY,
  chapter_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (chapter_id) REFERENCES chapters(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);

