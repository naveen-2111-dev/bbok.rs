CREATE TYPE license AS ENUM ('HEAVY', 'LIGHT');

CREATE TABLE
    driver (
        id SERIAL PRIMARY KEY,
        driver_name VARCHAR(20) NOT NULL,
        driver_age INT NOT NULL CHECK (
            driver_age > 23
            AND driver_age < 40
        ),
        driver_phone VARCHAR(12) NOT NULL,
        driver_license_number VARCHAR(15) NOT NULL,
        license_type license NOT NULL DEFAULT 'LIGHT'
    )