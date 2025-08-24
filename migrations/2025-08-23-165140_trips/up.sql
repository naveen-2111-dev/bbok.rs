CREATE TYPE trip_status AS ENUM ('SCHEDULED', 'CONFIRMED', 'CANCELLED');

CREATE TABLE
    trips (
        id SERIAL PRIMARY KEY,
        bus_id INT REFERENCES bus (id) ON DELETE CASCADE,
        route_id INT REFERENCES busroutes (id) ON DELETE CASCADE,
        arrival_time TIMESTAMP NOT NULL,
        departure_time TIMESTAMP NOT NULL,
        starting_location VARCHAR(20) NOT NULL DEFAULT 'chennai',
        price DECIMAL(10, 2) NOT NULL,
        status trip_status NOT NULL DEFAULT 'SCHEDULED'
    )