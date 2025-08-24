CREATE TABLE
    ratings (
        id SERIAL PRIMARY KEY,
        booking_id INT NOT NULL REFERENCES booking (id) ON DELETE CASCADE,
        user_id INT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
        bus_id INT NOT NULL REFERENCES bus (id) ON DELETE CASCADE,
        driver_id INT REFERENCES driver (id) ON DELETE SET NULL,
        operator_id INT REFERENCES organizations (id) ON DELETE SET NULL,
        rating INT NOT NULL CHECK (rating BETWEEN 1 AND 5),
        review TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        UNIQUE (booking_id, user_id)
    );

CREATE TABLE
    organization_ratings (
        id SERIAL PRIMARY KEY,
        organization_id INT REFERENCES organizations (id) ON DELETE CASCADE,
        user_id INT REFERENCES users (id) ON DELETE CASCADE,
        rating INT CHECK (rating BETWEEN 1 AND 5),
        review TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        UNIQUE (organization_id, user_id)
    );