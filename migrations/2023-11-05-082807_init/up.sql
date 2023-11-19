CREATE TABLE items (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    price DOUBLE PRECISION NOT NULL
);

CREATE TABLE sizes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE items_sizes (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGSERIAL NOT NULL,
    size_id SERIAL NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items(id),
    FOREIGN KEY (size_id) REFERENCES sizes(id)
);
