PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE servers (
    server_id INTEGER PRIMARY KEY,
    server_name TEXT NOT NULL
);
CREATE TABLE channel_id (
    channel_id INTEGER PRIMARY KEY,
    server_id INTEGER NOT NULL,
    channel_name TEXT NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(server_id)
);
CREATE TABLE role_id (
    role_id INTEGER PRIMARY KEY,
    channel_id INTEGER NOT NULL,
    role_name TEXT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channel_id(channel_id)
);
CREATE TABLE voice_channel_id (
    voice_channel_id INTEGER PRIMARY KEY,
    channel_id INTEGER NOT NULL,
    voice_channel_name TEXT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channel_id(channel_id)
);
COMMIT;
