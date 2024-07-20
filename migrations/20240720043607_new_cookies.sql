CREATE TABLE IF NOT EXISTS cookiesv2 (
    key NVCHAR PRIMARY KEY NOT NULL,
    value NVCHAR NOT NULL
);

INSERT OR REPLACE INTO cookiesv2 (key, value)
-- `,` cannot be used in cookies; so, it is safe to use here as a delimiter
SELECT site || ',' || key, value
FROM cookies
WHERE EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='cookies');

DROP TABLE IF EXISTS cookies;
