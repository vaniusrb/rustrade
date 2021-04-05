;
DROP TABLE IF EXISTS flow
;
CREATE TABLE flow
(
    id integer NOT NULL,
    position integer NOT NULL,
    is_buyer_maker boolean NOT NULL,
    time timestamp with time zone NOT NULL,
    price numeric(20,8) NOT NULL,
    quantity numeric(20,8) NOT NULL,
    total numeric(20,8) NOT NULL,
    real_balance_fiat_old numeric(20,8) NOT NULL,
    real_balance_fiat_new numeric(20,8) NOT NULL,
    CONSTRAINT flow_pkey PRIMARY KEY (id)
)
;
