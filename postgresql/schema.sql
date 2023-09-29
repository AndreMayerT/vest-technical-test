CREATE TABLE IF NOT EXISTS holdings (
    holding_id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    quantity INT NOT NULL CHECK (quantity >= 0),
    average_cost_per_share FLOAT NOT NULL CHECK (average_cost_per_share >= 0),
    total_cost FLOAT NOT NULL CHECK (total_cost >= 0)
);
