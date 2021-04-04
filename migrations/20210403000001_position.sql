-- Add migration script here

-- Your SQL goes here
DROP TABLE IF EXISTS position
;
CREATE TABLE position
(
    id integer NOT NULL,
    symbol character varying(8) NOT NULL,
    quantity numeric(20,8) NOT NULL,
    price numeric(20,8) NOT NULL,
    time timestamp with time zone NOT NULL,
    is_buyer_maker numeric(1,0) NOT NULL,

    pub id: i32,
    pub balance_asset: Decimal,
    pub balance_fiat: Decimal,
    pub price: Price,
    pub real_balance_fiat: Decimal,


    CONSTRAINT trade_pkey PRIMARY KEY (id)
)
