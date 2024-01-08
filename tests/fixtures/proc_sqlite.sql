CREATE SCHEMA my_schema AUTHORIZATION rw_user;
GRANT ALL ON SCHEMA my_schema TO rw_user;
GRANT USAGE ON SCHEMA my_schema TO upd_user;
GRANT USAGE ON SCHEMA my_schema TO ro_user;
CREATE TABLE users (
  workspace text NULL,
  is_archived integer NULL,
  creation_date text NOT NULL,
  user_id text NOT NULL,
  user_name text NULL,
  full_name text NULL,
  "role" text NOT NULL,
  is_locked integer NOT NULL,
  email text NULL,
  phone_number text NULL,
  supervisor_id text NULL,
  is_locked_by_supervisor integer NOT NULL DEFAULT false,
  is_locked_by_headquarters integer NOT NULL DEFAULT false,
  CONSTRAINT users_workspace_user_id_pk PRIMARY KEY (workspace,user_id),
  CONSTRAINT users_workspace_user_name_uk UNIQUE (workspace,user_name)
);
ALTER TABLE users OWNER TO rw_user;
GRANT ALL ON TABLE users TO rw_user;
GRANT ALL ON TABLE users TO upd_user;
GRANT SELECT ON TABLE users TO ro_user;
CREATE TABLE cache (
  id autoincrement NOT NULL,
  pk text NULL,
  responsible text NOT NULL,
  responsible_id text NOT NULL,
  workspace text NOT NULL,
  survey_id integer NOT NULL,
  questionnaire_id text NOT NULL,
  date_created text NOT NULL,
  date_updated text NOT NULL,
  otro integer NULL,
  CONSTRAINT cache_workspace_survey_id_pk PRIMARY KEY (workspace,survey_id)
);
ALTER TABLE cache OWNER TO rw_user;
GRANT ALL ON TABLE cache TO rw_user;
GRANT ALL ON TABLE cache TO upd_user;
GRANT SELECT ON TABLE cache TO ro_user;
ALTER SEQUENCE asignaciones_cache_id_seq OWNER TO rw_user;
GRANT ALL ON SEQUENCE asignaciones_cache_id_seq TO rw_user;
GRANT ALL ON SEQUENCE asignaciones_cache_id_seq TO upd_user;
GRANT SELECT ON SEQUENCE asignaciones_cache_id_seq TO ro_user;