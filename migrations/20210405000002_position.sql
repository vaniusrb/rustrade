;
DROP TABLE IF EXISTS position
;
CREATE TABLE position
(
    id integer NOT NULL,
    balance_asset numeric(20,8) NOT NULL,
    balance_fiat numeric(20,8) NOT NULL,
    price numeric(20,8) NOT NULL,
    real_balance_fiat numeric(20,8) NOT NULL,
    description varchar(100) NOT NULL,
    CONSTRAINT position_pkey PRIMARY KEY (id)
)
