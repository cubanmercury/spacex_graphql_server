CREATE TABLE crew
(
  id character varying NOT NULL,
  name character varying,
  agency character varying,
  image character varying,
  wikipedia character varying,
  launches text[],
  status character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT crew_pkey PRIMARY KEY (id)
)