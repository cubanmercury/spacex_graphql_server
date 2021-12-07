CREATE TABLE starlinks
(
  starlink_id character varying NOT NULL,
  version character varying,
  launch character varying,
  longitude Float,
  latitude Float,
  height_km Float,
  velocity_kms Float,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT starlinks_pkey PRIMARY KEY (starlink_id)
)