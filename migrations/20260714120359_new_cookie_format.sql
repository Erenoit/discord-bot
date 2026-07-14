CREATE TABLE IF NOT EXISTS cookiesv3 (
    host TEXT NOT NULL,
    host_only INTEGER NOT NULL CHECK (host_only IN (0, 1)),
    path TEXT NOT NULL DEFAULT '/',
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    expires INTEGER,
    secure INTEGER NOT NULL DEFAULT 0 CHECK (secure IN (0, 1)),
    Primary Key (host, path, name)
) WITHOUT ROWID;

-- For fast indexing with host
CREATE INDEX IF NOT EXISTS idx_cookies_host ON cookiesv3(host);

-- Don't keep any cookies from previous. Not enough information to fill the fields.

DROP TABLE IF EXISTS cookiesv2;
