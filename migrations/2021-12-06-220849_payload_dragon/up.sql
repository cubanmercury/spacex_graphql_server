CREATE TABLE payload_dragon
(
  id INT GENERATED ALWAYS AS IDENTITY,
  payload_id character varying NOT NULL,
  capsule character varying,
  mass_returned_kg Float,
  mass_returned_lbs Float,
  flight_time_sec Int,
  manifest character varying,
  water_landing Boolean,
  land_landing Boolean,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT payload_dragon_pkey PRIMARY KEY (id),
  CONSTRAINT fk_payloads
    FOREIGN KEY(payload_id)
      REFERENCES payloads(payload_id),
  UNIQUE(payload_id)
)-- Your SQL goes here