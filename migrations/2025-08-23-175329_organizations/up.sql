CREATE TABLE
    organizations (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        contact_email VARCHAR(255) UNIQUE,
        contact_phone VARCHAR(20),
        address TEXT,
        gst_number VARCHAR(50) UNIQUE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

ALTER TABLE bus
ADD COLUMN organization_id INT REFERENCES organizations (id) ON DELETE CASCADE;