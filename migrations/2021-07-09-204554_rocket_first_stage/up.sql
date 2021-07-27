CREATE TABLE rocket_first_stage
(
  id INT GENERATED ALWAYS AS IDENTITY,
  rocket_id character varying NOT NULL,
  thrust_sea_level_kn Float,
  thrust_sea_level_lbf Float,
  thrust_vacuum_kn Float,
  thrust_vacuum_lbf Float,
  reusable Boolean,
  engines Int,
  fuel_amount_tons Float,
  burn_time_sec Int,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT rocket_first_stage_pkey PRIMARY KEY (id),
  CONSTRAINT fk_rockets
    FOREIGN KEY(rocket_id)
      REFERENCES rockets(rocket_id),
  UNIQUE(rocket_id)
)