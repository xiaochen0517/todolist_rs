CREATE TABLE IF NOT EXISTS "user"
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(255) UNIQUE NOT NULL,
    email      VARCHAR(255) UNIQUE NULL,
    password   VARCHAR(255)        NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO "user" (username, email, password)
-- Password is "todolist" hashed with bcrypt
VALUES ('admin', NULL, '$argon2id$v=19$m=19456,t=2,p=1$mD4ecTLFSdVqJS5GZk/7kQ$TQtucK69KSoccUAclGMvLY+avgjuCXHTiAmRwnajk9U');