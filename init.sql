DROP TABLE rappels;

CREATE TABLE rappels (
    id serial primary key,
    nom VARCHAR(30) not null,
    date_limite date,
    repetitition int,
    criticite VARCHAR(30)
);

INSERT INTO rappels (nom, date_limite, repetitition, criticite) VALUES ('Test', NOW(), 2, 'URGENT');
