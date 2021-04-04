-- Add migration script here

-- Your SQL goes here
DROP TABLE IF EXISTS trade
;
CREATE TABLE trade
(
    id numeric(8,0) NOT NULL,
    symbol character varying(8) NOT NULL,
    quantity numeric(20,8) NOT NULL,
    price numeric(20,8) NOT NULL,
    time timestamp with time zone NOT NULL,
    is_buyer_maker numeric(1,0) NOT NULL,
    CONSTRAINT trade_pkey PRIMARY KEY (id)
)
