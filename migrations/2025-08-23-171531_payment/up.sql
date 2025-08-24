CREATE TYPE payment_status AS ENUM ('PENDING', 'SUCCESS', 'FAILED');

CREATE TYPE payment_type AS ENUM ('UPI', 'CARD', 'BANK');

CREATE TABLE
    payments (
        id SERIAL PRIMARY KEY,
        booking_id INT NOT NULL REFERENCES booking (id) ON DELETE CASCADE,
        amount DECIMAL(10, 2) NOT NULL,
        payment_method payment_type NOT NULL,
        status payment_status DEFAULT 'PENDING',
        transaction_id VARCHAR(100) UNIQUE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )