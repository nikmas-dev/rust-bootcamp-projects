-- Add migration script here
CREATE TABLE IF NOT EXISTS "user" (
    id            BIGSERIAL    PRIMARY KEY,
    name          VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_friends (
    user_id    BIGINT NOT NULL,
    friend_id  BIGINT NOT NULL,
    CONSTRAINT pk_user_friend PRIMARY KEY (user_id, friend_id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE,
    CONSTRAINT fk_friend FOREIGN KEY (friend_id) REFERENCES "user" (id) ON DELETE CASCADE,
    CONSTRAINT chk_user_friend_to_himself CHECK (user_id != friend_id),
    CONSTRAINT chk_user_id_order CHECK (user_id <= friend_id)
);
