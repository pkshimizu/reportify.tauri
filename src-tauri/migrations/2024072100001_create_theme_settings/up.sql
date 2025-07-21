CREATE TABLE theme_settings (
    id INTEGER PRIMARY KEY,
    theme_name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert default theme
INSERT INTO theme_settings (id, theme_name) VALUES (1, 'light');