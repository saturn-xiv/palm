-- migrate:up
CREATE table users(
    id INT PRIMARY KEY,
    name VARCHAR(31) NOT NULL,
    password VARCHAR(255) NOT NULL,
    profile BLOB NOT NULL,
    deleted_at DATETIME,
    updated_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_users_name ON users(name);

-- migrate:down
DROP TABLE users;
