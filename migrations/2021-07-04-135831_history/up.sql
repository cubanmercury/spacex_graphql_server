CREATE TABLE history
(
  id character varying NOT NULL,
  title character varying,
  event_date_utc character varying,
  event_date_unix Int,
  details text,
  links_article character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT history_pkey PRIMARY KEY (id)
)