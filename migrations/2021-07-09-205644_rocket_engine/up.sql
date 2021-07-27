CREATE TABLE rocket_engine
(
  id INT GENERATED ALWAYS AS IDENTITY,
  rocket_id character varying NOT NULL,
  isp_sea_level Int,
  isp_vacuum Int,
  thrust_sea_level_kn Float,
  thrust_sea_level_lbf Float,
  thrust_vacuum_kn Float,
  thrust_vacuum_lbf Float,
  number Int,
  type character varying,
  version character varying,
  layout character varying,
  engine_loss_max Int,
  propellant_1 character varying,
  propellant_2 character varying,
  thrust_to_weight Float,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT rocket_engine_pkey PRIMARY KEY (id),
  CONSTRAINT fk_rockets
    FOREIGN KEY(rocket_id)
      REFERENCES rockets(rocket_id),
  UNIQUE(rocket_id)
)