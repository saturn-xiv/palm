-- migrate:up
CREATE TABLE forum(
    id SERIAL PRIMARY KEY,
    lang VARCHAR(15) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(511) NOT NULL,    
    status VARCHAR(15) NOT NULL,
    profile BYTEA NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_forum_lang ON forum(lang);
CREATE INDEX idx_forum_title ON forum(title);
CREATE INDEX idx_forum_status ON forum(status);

CREATE TABLE forum_topics(
    id SERIAL PRIMARY KEY,
    forum_id INT NOT NULL,
    subject VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL, 
    author_id INT NOT NULL,  
    status VARCHAR(15) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_forum_topics_subject ON forum_topics(subject);
CREATE INDEX idx_forum_topics_status ON forum_topics(status);


CREATE TABLE forum_posts(
    id SERIAL PRIMARY KEY,
    forum_id INT NOT NULL,
    topic_id INT NOT NULL,
    post_id INT,    
    body TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL,
    author_id INT NOT NULL,  
    status VARCHAR(15) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_forum_posts_status ON forum_posts(status);

-- migrate:down
DROP TABLE forum_posts;
DROP TABLE forum_topics;
DROP TABLE forum;
