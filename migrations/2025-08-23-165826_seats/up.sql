CREATE TABLE
    seats (
        id SERIAL PRIMARY KEY,
        trip_id INT REFERENCES trips (id) ON DELETE CASCADE,
        bus_id INT REFERENCES bus (id) ON DELETE CASCADE,
        seat_number INT NOT NULL,
        is_booked BOOLEAN NOT NULL DEFAULT FALSE
    )