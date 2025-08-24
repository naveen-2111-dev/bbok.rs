CREATE TYPE refund_status AS ENUM ('PENDING', 'SUCCESS', 'FAILED');

CREATE TABLE
    cancellations (
        id SERIAL PRIMARY KEY,
        booking_id INT REFERENCES booking (id) ON DELETE CASCADE,
        user_id INT REFERENCES users (id) ON DELETE CASCADE,
        reason TEXT,
        refund_amount DECIMAL(10, 2),
        refund_status refund_status DEFAULT 'PENDING',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );