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
