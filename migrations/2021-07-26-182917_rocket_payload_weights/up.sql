CREATE TABLE rocket_payload_weights
(
  id character varying NOT NULL,
  rocket_id character varying NOT NULL,
  payload_id character varying,
  name character varying,
  kg Int,
  lb Int,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT rocket_payload_weights_pkey PRIMARY KEY (id),
  CONSTRAINT fk_rockets
    FOREIGN KEY(rocket_id)
      REFERENCES rockets(rocket_id)
)