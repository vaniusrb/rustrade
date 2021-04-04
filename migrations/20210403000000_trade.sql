-- Add migration script here

-- Your SQL goes here
DROP TABLE IF EXISTS trade
;
CREATE TABLE trade
(
    id integer,
    symbol integer,
    quantity numeric(20,8) NOT NULL,
    price numeric(20,8) NOT NULL,
    time timestamp with time zone NOT NULL,
    is_buyer_maker boolean NOT NULL,
    CONSTRAINT trade_pkey PRIMARY KEY (id)
)
