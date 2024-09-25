CREATE TYPE type_user_kbn AS ENUM ('Admin', 'Normal');

CREATE TABLE public.users (
  uuid UUID NOT NULL
  ,user_name TEXT NOT NULL
  ,user_email TEXT NOT NULL
  ,user_kbn type_user_kbn NOT NULL
  ,PRIMARY KEY (uuid) 
);