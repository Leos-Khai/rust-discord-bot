PRAGMA foreign_keys = OFF;

BEGIN TRANSACTION;

-- Existing tables
CREATE TABLE
    servers (
        server_id INTEGER PRIMARY KEY,
        server_name TEXT NOT NULL
    );

CREATE TABLE
    channel_id (
        channel_id INTEGER PRIMARY KEY,
        server_id INTEGER NOT NULL,
        channel_name TEXT NOT NULL,
        FOREIGN KEY (server_id) REFERENCES servers (server_id)
    );

CREATE TABLE
    role_id (
        role_id INTEGER PRIMARY KEY,
        channel_id INTEGER NOT NULL,
        role_name TEXT NOT NULL,
        FOREIGN KEY (channel_id) REFERENCES channel_id (channel_id)
    );

CREATE TABLE
    voice_channel_id (
        voice_channel_id INTEGER PRIMARY KEY,
        channel_id INTEGER NOT NULL,
        voice_channel_name TEXT NOT NULL,
        FOREIGN KEY (channel_id) REFERENCES channel_id (channel_id)
    );

-- New table for storing linked channels
CREATE TABLE
    voice_text_link (
        link_id INTEGER PRIMARY KEY,
        voice_channel_id INTEGER NOT NULL,
        text_channel_id INTEGER NOT NULL,
        FOREIGN KEY (voice_channel_id) REFERENCES voice_channel_id (voice_channel_id),
        FOREIGN KEY (text_channel_id) REFERENCES channel_id (channel_id)
    );

COMMIT;