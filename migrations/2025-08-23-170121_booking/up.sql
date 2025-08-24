CREATE TYPE booking_status AS ENUM ('PENDING', 'CONFIRMED', 'CANCELLED');

CREATE TABLE
    booking (
        id SERIAL PRIMARY KEY,
        user_id INT REFERENCES users (id) ON DELETE CASCADE,
        trip_id INT REFERENCES trips (id) ON DELETE CASCADE,
        seat_id INT REFERENCES seats (id) ON DELETE CASCADE,
        booking_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        status booking_status NOT NULL DEFAULT 'PENDING'
    )