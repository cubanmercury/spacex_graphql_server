CREATE TABLE launch_failures
(
  id character varying NOT NULL,
  launch_id character varying NOT NULL,
  time Int,
  altitude Int,
  reason character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT launch_failures_pkey PRIMARY KEY (id),
  CONSTRAINT fk_launches
    FOREIGN KEY(launch_id)
      REFERENCES launches(launch_id)
)