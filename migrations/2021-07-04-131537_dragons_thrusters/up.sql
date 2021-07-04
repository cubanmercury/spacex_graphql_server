CREATE TABLE dragons_thrusters
(
  id character varying NOT NULL,
  dragon_id character varying NOT NULL,
  type character varying,
  amount Int,
  pods Int,
  fuel_1 character varying,
  fuel_2 character varying,
  isp Int,
  thrust_kn Float,
  thrust_lbf Float,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT dragons_thrusters_pkey PRIMARY KEY (id),
  CONSTRAINT fk_dragons
    FOREIGN KEY(dragon_id)
      REFERENCES dragons(dragon_id)
)