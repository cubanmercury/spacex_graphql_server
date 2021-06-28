CREATE TABLE capsules
(
  id character varying NOT NULL,
  reuse_count integer,
  water_landings integer,
  land_landings integer,
  last_update character varying NULL,
  launches text[],
  serial character varying,
  status character varying,
  type character varying,
  CONSTRAINT capsules_pkey PRIMARY KEY (id)
)