CREATE TABLE IF NOT EXISTS "todolist"
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER      NOT NULL,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE
);

INSERT INTO "todolist" (user_id, title, description)
VALUES (1, 'Buy groceries', 'Milk, Bread, Eggs, Butter'),
       (1, 'Finish project', 'Complete the project by the end of the week'),
       (1, 'Call mom', 'Check in with mom and see how she is doing');