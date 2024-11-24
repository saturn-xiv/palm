-- migrate:up
CREATE TABLE bbs_forums(
    id SERIAL PRIMARY KEY,
    lang VARCHAR(15) NOT NULL,
    slug VARCHAR(31) NOT NULL,
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
CREATE UNIQUE INDEX idx_bbs_forums_slug ON bbs_forums(slug);
CREATE INDEX idx_bbs_forums_lang ON bbs_forums(lang);
CREATE INDEX idx_bbs_forums_title ON bbs_forums(title);
CREATE INDEX idx_bbs_forums_status ON bbs_forums(status);

CREATE TABLE bbs_topics(
    id SERIAL PRIMARY KEY,
    forum_id INT NOT NULL,
    slug VARCHAR(127) NOT NULL,
    subject VARCHAR(255) NOT NULL,
    "body" TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL, 
    author_id INT NOT NULL,  
    status VARCHAR(15) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bbs_topics_slug ON bbs_topics(slug);
CREATE INDEX idx_bbs_topics_subject ON bbs_topics(subject);
CREATE INDEX idx_bbs_topics_status ON bbs_topics(status);


CREATE TABLE bbs_posts(
    id SERIAL PRIMARY KEY,
    forum_id INT NOT NULL,
    topic_id INT NOT NULL,
    parent_id INT,    
    "body" TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL,
    author_id INT NOT NULL,  
    status VARCHAR(15) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_bbs_posts_status ON bbs_posts(status);

-- migrate:down
DROP TABLE bbs_posts;
DROP TABLE bbs_topics;
DROP TABLE bbs;
