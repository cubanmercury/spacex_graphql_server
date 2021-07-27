CREATE TABLE launch_links
(
  id INT GENERATED ALWAYS AS IDENTITY,
  launch_id character varying NOT NULL,
  patch_small character varying,
  patch_large character varying,
  reddit_campaign character varying,
  reddit_launch character varying,
  reddit_media character varying,
  reddit_recovery character varying,
  flickr_small text[],
  flickr_original text[],
  presskit character varying,
  webcast character varying,
  youtube_id character varying,
  article character varying,
  wikipedia character varying,
  row_updated timestamp with time zone NOT NULL,
  CONSTRAINT launch_links_pkey PRIMARY KEY (id),
  CONSTRAINT fk_launches
    FOREIGN KEY(launch_id)
      REFERENCES launches(launch_id),
  UNIQUE(launch_id)
)