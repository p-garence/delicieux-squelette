CREATE TABLE dessins (
  id SERIAL PRIMARY KEY,
  x Integer NOT NULL,
  y Integer NOT NULL,
  done Bool NOT NULL DEFAULT FALSE,
  room_id Integer NOT NULL,
  content Bytea,
  last_request Timestamp
);

CREATE UNIQUE INDEX position_and_room_id ON dessins(x, y, room_id);