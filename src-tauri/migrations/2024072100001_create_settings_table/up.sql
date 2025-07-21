-- Create settings table for application configuration
CREATE TABLE settings (
    id INTEGER PRIMARY KEY,
    theme TEXT NOT NULL DEFAULT 'light',
    language TEXT NOT NULL DEFAULT 'en',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert default settings record
INSERT INTO settings (id, theme, language) VALUES (1, 'light', 'en');