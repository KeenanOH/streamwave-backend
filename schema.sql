CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sessions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL UNIQUE,
    session_id VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS shows (
    id INT AUTO_INCREMENT PRIMARY KEY,
    show_id VARCHAR(255) NOT NULL,
    translation_type ENUM('dub', 'sub') NOT NULL,
    name TEXT NOT NULL,
    image_url TEXT NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    episodes INT NOT NULL,

    UNIQUE KEY (show_id, translation_type)
);


CREATE TABLE IF NOT EXISTS episodes (
    id     INT AUTO_INCREMENT PRIMARY KEY,
    name   TEXT NOT NULL,
    number INT  NOT NULL,
    season INT  NOT NULL,
    show_id INT NOT NULL,
    url TEXT NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (show_id) REFERENCES shows(id)
);

CREATE TABLE IF NOT EXISTS users_shows (
    id INT AUTO_INCREMENT PRIMARY KEY,
    show_id INT NOT NULL UNIQUE,
    user_id INT NOT NULL UNIQUE,
    FOREIGN KEY (show_id) REFERENCES shows(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
