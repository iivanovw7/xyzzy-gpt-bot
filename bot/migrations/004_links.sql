CREATE TABLE IF NOT EXISTS link_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    UNIQUE(user_id, name)
);

CREATE TABLE IF NOT EXISTS links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    title TEXT,
    description TEXT,
    thumbnail_url TEXT,
    category_id INTEGER,
    tags TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    FOREIGN KEY(category_id) REFERENCES link_categories(id)
);

CREATE VIRTUAL TABLE IF NOT EXISTS links_fts USING fts5(
    title,
    description,
    tags,
    category,
    content='links',
    content_rowid='id'
);

CREATE TRIGGER IF NOT EXISTS links_ai AFTER INSERT ON links BEGIN
    INSERT INTO links_fts(rowid, title, description, tags, category)
    VALUES (new.id, new.title, new.description, new.tags, (SELECT name FROM link_categories WHERE id = new.category_id));
END;

CREATE TRIGGER IF NOT EXISTS links_ad AFTER DELETE ON links BEGIN
    INSERT INTO links_fts(links_fts, rowid, title, description, tags, category)
    VALUES('delete', old.id, old.title, old.description, old.tags, (SELECT name FROM link_categories WHERE id = old.category_id));
END;

CREATE TRIGGER IF NOT EXISTS links_au AFTER UPDATE ON links BEGIN
    INSERT INTO links_fts(links_fts, rowid, title, description, tags, category)
    VALUES('delete', old.id, old.title, old.description, old.tags, (SELECT name FROM link_categories WHERE id = old.category_id));
    INSERT INTO links_fts(rowid, title, description, tags, category)
    VALUES (new.id, new.title, new.description, new.tags, (SELECT name FROM link_categories WHERE id = new.category_id));
END;
