CREATE TABLE demo.prueba (
  interview_id text NOT NULL,
  pk text NULL,
  workspace text NULL,
  gallo int NULL,
  vivo bool NULL DEFAULT true,
  tel text NULL,
  CONSTRAINT prueba_interview_id_workspace_pk PRIMARY KEY (interview_id,workspace)
);