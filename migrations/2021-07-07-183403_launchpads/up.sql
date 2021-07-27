CREATE TABLE launchpads
(
  id character varying NOT NULL,
  name character varying,
  full_name character varying,
  locality character varying,
  region character varying,
  timezone character varying,
  latitude Float,
  longitude Float,
  launch_attempts Int,
  launch_successes Int,
  rockets text[],
  launches text[],
  status character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT launchpads_pkey PRIMARY KEY (id)
)