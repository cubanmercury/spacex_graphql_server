CREATE TABLE landpads
(
  id character varying NOT NULL,
  name character varying,
  full_name character varying,
  status character varying,
  type character varying,
  locality character varying,
  region character varying,
  latitude Float,
  longitude Float,
  landing_attempts Int,
  landing_successes Int,
  wikipedia character varying,
  details text,
  launches text[],
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT landpads_pkey PRIMARY KEY (id)
)