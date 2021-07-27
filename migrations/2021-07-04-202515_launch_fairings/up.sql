CREATE TABLE launch_fairings
(
  id INT GENERATED ALWAYS AS IDENTITY,
  launch_id character varying NOT NULL,
  reused Boolean,
  recovery_attempt Boolean,
  recovered Boolean,
  ships text[],
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT launch_fairings_pkey PRIMARY KEY (id),
  CONSTRAINT fk_launches
    FOREIGN KEY(launch_id)
      REFERENCES launches(launch_id),
  UNIQUE(launch_id)
)