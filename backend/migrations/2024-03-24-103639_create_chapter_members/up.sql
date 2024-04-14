CREATE TABLE chapter_members (
  chapter_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (chapter_id) REFERENCES chapters(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
  PRIMARY KEY(chapter_id, user_id)
);

