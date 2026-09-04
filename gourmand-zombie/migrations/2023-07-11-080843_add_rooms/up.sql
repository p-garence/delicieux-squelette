CREATE TABLE rooms
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    rules TEXT,
    is_public BOOL NOT NULL,
    password_protected BOOL NOT NULL,
    hashed_password TEXT,
    colors TEXT [] NOT NULL,
    resolution INTEGER NOT NULL,
    number_of_dessin INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    hashed_admin TEXT NOT NULL
);

-- Should add foreign key like so next time 
-- ALTER TABLE dessins ADD COLUMN parent_room SERIAL NOT NULL;
-- UPDATE dessins SET parent_room = room_id WHERE parent_room = 0;
-- ALTER TABLE dessins ADD CONSTRAINT constraint_parent_room FOREIGN KEY (parent_room) REFERENCES rooms(id);
