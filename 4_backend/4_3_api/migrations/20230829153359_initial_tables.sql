-- Add migration script here
CREATE TABLE IF NOT EXISTS "user" (
    id   BIGSERIAL    PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS role (
    slug        VARCHAR(255) PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    permissions VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_role (
    user_id   BIGINT,
    role_slug VARCHAR(255),
    CONSTRAINT pk_user_role PRIMARY KEY (user_id, role_slug),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
    CONSTRAINT fk_role FOREIGN KEY (role_slug) REFERENCES role(slug) ON DELETE CASCADE
)
