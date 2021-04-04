DROP TABLE IF EXISTS symbol
;
CREATE TABLE symbol
(
    id integer NOT NULL,
    symbol varchar(8) NOT NULL,
    CONSTRAINT symbol_pkey PRIMARY KEY (id)
)
;
DROP TABLE IF EXISTS candle
;
CREATE TABLE candle
(
    id integer NOT NULL,
    symbol integer NOT NULL,
    minutes integer NOT NULL,
    open numeric(20,8) NOT NULL,
    high numeric(20,8) NOT NULL,
    low numeric(20,8) NOT NULL,
    close numeric(20,8) NOT NULL,
    volume numeric(20,8) NOT NULL,
    open_time timestamp with time zone NOT NULL,
    close_time timestamp with time zone NOT NULL,
    CONSTRAINT candle_pkey PRIMARY KEY (id),
    CONSTRAINT start_time UNIQUE (symbol, minutes, open_time)
)
;
DROP TABLE IF EXISTS trade
;
CREATE TABLE trade
(
    id integer NOT NULL,
    symbol integer NOT NULL,
    quantity numeric(20,8) NOT NULL,
    price numeric(20,8) NOT NULL,
    time timestamp with time zone NOT NULL,
    is_buyer_maker boolean NOT NULL,
    CONSTRAINT trade_pkey PRIMARY KEY (id)
)
