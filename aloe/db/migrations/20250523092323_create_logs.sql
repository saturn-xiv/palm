-- migrate:up
CREATE table logs(
    id INT PRIMARY KEY,
    user_id INT NOT NULL,
    message TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- migrate:down
DROP TABLE logs;
