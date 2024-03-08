CREATE TABLE perms (
  id integer primary key autoincrement NOT NULL,
  perm_name text NOT NULL,
  CONSTRAINT perms_perm_name_uk UNIQUE (perm_name)
);
CREATE TABLE roles_perms (
  id_role integer NOT NULL,
  id_perm integer NOT NULL,
  CONSTRAINT roles_perms_perms_id_perm_fk FOREIGN KEY (id_perm) REFERENCES perms (id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT roles_perms_id_role_id_perm_pk PRIMARY KEY (id_role,id_perm)
);
CREATE TABLE roles (
  id integer primary key autoincrement NOT NULL,
  role_name text NOT NULL,
  level integer NOT NULL,
  CONSTRAINT roles_role_name_uk UNIQUE (role_name)
);
CREATE TABLE provincias (
  nprvnc text NOT NULL,
  cdscrpcn text NOT NULL,
  CONSTRAINT provincias_nprvnc_pk PRIMARY KEY (nprvnc),
  CONSTRAINT provincias_nprvnc_cdscrpcn_uk UNIQUE (nprvnc,cdscrpcn)
);
CREATE TABLE users (
  id integer primary key autoincrement NOT NULL,
  username text NOT NULL,
  first_name text NOT NULL,
  last_name text NOT NULL,
  password text NOT NULL,
  email text NULL,
  "role" text NOT NULL,
  provincia text NULL,
  survey_user text NULL,
  is_locked integer NOT NULL DEFAULT false,
  created_at text NOT NULL DEFAULT "CURRENT_TIMESTAMP",
  CONSTRAINT users_roles_role_fk FOREIGN KEY (role) REFERENCES roles (role_name) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT users_provincias_provincia_fk FOREIGN KEY (provincia) REFERENCES provincias (nprvnc) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT users_username_uk UNIQUE (username)
);