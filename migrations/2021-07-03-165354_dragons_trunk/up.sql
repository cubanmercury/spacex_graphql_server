CREATE TABLE dragons_trunk
(
  id INT GENERATED ALWAYS AS IDENTITY,
  dragon_id character varying NOT NULL,
  trunk_volume_cubic_meters Int,
  trunk_volume_cubic_feet Int,
  cargo_solar_array Int,
  cargo_unpressurized_cargo Boolean,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT dragons_trunk_pkey PRIMARY KEY (id),
  CONSTRAINT fk_dragons
    FOREIGN KEY(dragon_id)
      REFERENCES dragons(dragon_id)
)