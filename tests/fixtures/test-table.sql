CREATE TABLE demo.prueba (
  id text NOT NULL,
  pk text NULL,
  ws text NULL,
  gallo int NULL,
  vivo bool NULL DEFAULT true,
  tel text NULL,
  CONSTRAINT prueba_id_ws_pk PRIMARY KEY (id,ws)
);