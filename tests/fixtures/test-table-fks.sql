ALTER TABLE demo.prueba
  ADD CONSTRAINT prueba_cache_ws_id_user_id_fk
  FOREIGN KEY (ws_id,user_id)
  REFERENCES demo.cache (ws,user)
  ON DELETE RESTRICT ON UPDATE RESTRICT;