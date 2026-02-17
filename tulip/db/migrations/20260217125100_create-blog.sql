-- migrate:up
CREATE TABLE blog_posts(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    title VARCHAR(127) NOT NULL,
    body TEXT NOT NULL,
    body_editor INT NOT NULL,
    permalink VARCHAR(63) NOT NULL,
    location VARCHAR(255),
    published_at TIMESTAMP WITHOUT TIME ZONE,
    reader_comments INT NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_blog_posts_permalink ON blog_posts(permalink);
CREATE INDEX idx_blog_posts_title ON blog_posts(title);
CREATE INDEX idx_blog_posts_location ON blog_posts(location) WHERE location IS NOT NULL;

CREATE TABLE blog_post_labels(
    id BIGSERIAL PRIMARY KEY,
    post_id BIGINT NOT NULL,
    name VARCHAR(31) NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_blog_post_labels ON blog_post_labels(post_id, name);
CREATE INDEX idx_blog_post_labels_name ON blog_post_labels(name);

CREATE TABLE blog_post_comments(
    id BIGSERIAL PRIMARY KEY,    
    post_id BIGINT NOT NULL,
    comment_id BIGINT,
    user_id BIGINT,
    body TEXT NOT NULL,
    body_editor INT NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- migrate:down
DROP TABLE blog_post_labels;
DROP TABLE blog_post_comments;
DROP TABLE blog_posts;
