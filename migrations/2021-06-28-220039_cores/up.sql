CREATE TABLE cores
(
  id character varying NOT NULL,
  block integer,
  reuse_count integer,
  rtls_attempts integer,
  rtls_landings integer,
  asds_attempts integer,
  asds_landings integer,
  last_update character varying NULL,
  launches text[],
  serial character varying,
  status character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT cores_pkey PRIMARY KEY (id)
)