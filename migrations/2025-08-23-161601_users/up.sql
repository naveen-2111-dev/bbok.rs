-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    email VARCHAR(20) NOT NULL,
    phone VARCHAR(12) NOT NULL,
    password VARCHAR(8) NOT NULL,
    CONSTRAINT password_check CHECK (
        password ~ '^(?=.*[A-Za-z])(?=.*[0-9])(?=.*[!@#$%^&*]).{8,}$'
    )
)