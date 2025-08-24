CREATE TYPE bus_type AS ENUM ('AC', 'NON-AC', 'SLEEPER', 'SEMI-SLEEPER');

CREATE TABLE
    bus (
        id SERIAL PRIMARY KEY,
        bus_number VARCHAR(9) UNIQUE NOT NULL,
        bus_type bus_type NOT NULL DEFAULT 'AC',
        bus_driver INT REFERENCES driver (id) ON DELETE CASCADE,
        capacity INT NOT NULL,
        org_name VARCHAR(20) DEFAULT 'MP-Travels',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )