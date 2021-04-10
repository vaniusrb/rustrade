-- Add migration script here
DROP TABLE IF EXISTS trade_agg
;
CREATE TABLE trade_agg
(
    id bigint NOT NULL,
    symbol integer NOT NULL,
    quantity numeric(20,8) NOT NULL,
    price numeric(20,8) NOT NULL,
    time timestamp with time zone NOT NULL,
    CONSTRAINT trade_agg_pkey PRIMARY KEY (id)
)
