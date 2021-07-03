CREATE TABLE dragons_heat_shield
(
  id INT GENERATED ALWAYS AS IDENTITY,
  dragon_id character varying NOT NULL,
  material character varying,
  size_meters Float,
  temp_degrees Int,
  dev_partner character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT dragons_heat_shield_pkey PRIMARY KEY (id),
  CONSTRAINT fk_dragons
    FOREIGN KEY(dragon_id)
      REFERENCES dragons(dragon_id)
)