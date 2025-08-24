-- Your SQL goes here
CREATE TABLE
    busroutes (
        id SERIAL PRIMARY KEY,
        route_number VARCHAR(10) NOT NULL,
        start_location VARCHAR(100) NOT NULL,
        end_location VARCHAR(100) NOT NULL,
        distance INT NOT NULL, --in KM
        travel_time INT NOT NULL
    )