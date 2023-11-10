CREATE TABLE items (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    price DOUBLE PRECISION NOT NULL
);

CREATE TABLE sizes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE items_sizes (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGSERIAL,
    size_id SERIAL,
    quantity INTEGER,
    FOREIGN KEY (item_id) REFERENCES items(id),
    FOREIGN KEY (size_id) REFERENCES sizes(id)
);
