CREATE TABLE guilds (
    id                   BIGSERIAL PRIMARY KEY,
    guild_id             varchar(19) NOT NULL UNIQUE,
    disabled_providers   text[] NOT NULL
);
