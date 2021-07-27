CREATE TABLE launch_cores
(
  id character varying NOT NULL,
  launch_id character varying NOT NULL,
  core character varying,
  flight Int,
  gridfins Boolean,
  legs Boolean,
  reused Boolean,
  landing_attempt Boolean,
  landing_success Boolean,
  landing_type character varying,
  landpad character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT launch_cores_pkey PRIMARY KEY (id),
  CONSTRAINT fk_launches
    FOREIGN KEY(launch_id)
      REFERENCES launches(launch_id)
)