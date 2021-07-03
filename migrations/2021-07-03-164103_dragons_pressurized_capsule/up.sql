CREATE TABLE dragons_pressurized_capsule
(
  id INT GENERATED ALWAYS AS IDENTITY,
  dragon_id character varying NOT NULL,
  payload_volume_cubic_meters Int,
  payload_volume_cubic_feet Int,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT dragons_pressurized_capsule_pkey PRIMARY KEY (id),
  CONSTRAINT fk_dragons
    FOREIGN KEY(dragon_id)
      REFERENCES dragons(dragon_id)
)