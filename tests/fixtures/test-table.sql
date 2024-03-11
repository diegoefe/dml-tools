CREATE TABLE demo.prueba (
  id text NOT NULL,
  pk text NULL,
  ws text NULL,
  gallo int NULL,
  vivo bool NULL DEFAULT true,
  tel text NULL,
  CONSTRAINT prueba_id_ws_pk PRIMARY KEY (id,ws),
  CONSTRAINT prueba_cache_ws_id_user_id_fk FOREIGN KEY (ws_id,user_id) REFERENCES demo.cache (ws,user) ON DELETE RESTRICT ON UPDATE RESTRICT
);