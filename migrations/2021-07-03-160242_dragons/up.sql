CREATE TABLE dragons
(
  dragon_id character varying NOT NULL,
  launch_payload_mass_kg Int,
  launch_payload_mass_lbs Int,
  launch_payload_vol_cubic_meters Int,
  launch_payload_vol_cubic_feet Int,
  return_payload_mass_kg Int,
  return_payload_mass_lbs Int,
  return_payload_vol_cubic_meters Int,
  return_payload_vol_cubic_feet Int,
  height_w_trunk_meters Float,
  height_w_trunk_feet Float,
  diameter_meters Float,
  diameter_feet Float,
  first_flight character varying,
  flickr_images text[],
  name character varying,
  type character varying,
  active Boolean,
  crew_capacity Int,
  sidewall_angle_deg Int,
  orbit_duration_yr Int,
  dry_mass_kg Int,
  dry_mass_lbs Int,
  wikipedia character varying,
  description text,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT dragons_pkey PRIMARY KEY (dragon_id)
)