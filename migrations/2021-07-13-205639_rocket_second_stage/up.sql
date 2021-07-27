CREATE TABLE rocket_second_stage
(
  id INT GENERATED ALWAYS AS IDENTITY,
  rocket_id character varying NOT NULL,
  thrust_kn Float,
  thrust_lbf Float,
  payloads_composite_fairing_height_meters Float,
  payloads_composite_fairing_height_feet Float,
  payloads_composite_fairing_diameter_meters Float,
  payloads_composite_fairing_diameter_feet Float,
  payloads_option_1 character varying,
  reusable Boolean,
  engines Int,
  fuel_amount_tons Float,
  burn_time_sec Int,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT rocket_second_stage_pkey PRIMARY KEY (id),
  CONSTRAINT fk_rockets
    FOREIGN KEY(rocket_id)
      REFERENCES rockets(rocket_id),
  UNIQUE(rocket_id)
)