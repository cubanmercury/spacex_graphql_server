CREATE TABLE payloads
(
  payload_id character varying NOT NULL,
  name character varying,
  type character varying,
  reused Boolean,
  launch character varying,
  customers text[],
  norad_ids Int[],
  nationalities text[],
  manufacturers text[],
  mass_kg Float,
  mass_lbs Float,
  orbit character varying,
  reference_system character varying,
  regime character varying,
  longitude Float,
  semi_major_axis_km Float,
  eccentricity Float,
  periapsis_km Float,
  apoapsis_km Float,
  inclination_deg Float,
  period_min Float,
  lifespan_years Float,
  epoch character varying,
  mean_motion Float,
  raan Float,
  arg_of_pericenter Float,
  mean_anomaly Float,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT payloads_pkey PRIMARY KEY (payload_id)
)