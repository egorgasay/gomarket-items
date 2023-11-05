CREATE TABLE orders (
  id VARCHAR PRIMARY KEY
);

CREATE TABLE goods (
  id VARCHAR PRIMARY KEY,
  price DOUBLE PRECISION NOT NULL
);

CREATE TABLE orders_goods (
    order_id VARCHAR PRIMARY KEY,
    good_id VARCHAR,
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (good_id) REFERENCES goods(id)
);
