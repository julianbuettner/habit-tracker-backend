
CREATE TABLE habitname (
  id SERIAL PRIMARY KEY,
  "name" VARCHAR UNIQUE NOT NULL
);

CREATE TABLE habitmetric (
  id SERIAL PRIMARY KEY,
  habitname_id INT NOT NULL,
  "datetime" TIMESTAMP NOT NULL DEFAULT NOW(),
  units INT NOT NULL,
  CONSTRAINT contraint_habitname FOREIGN KEY (habitname_id)
    REFERENCES habitname(id)
    ON DELETE CASCADE
);

CREATE VIEW habitdays AS
SELECT
  habitname.name,
  date("datetime") AS "date",
  coalesce(sum(units) FILTER (WHERE units > 0), 0) AS "posunits",
  coalesce(sum(units) FILTER (WHERE units < 0), 0) AS "negunits"
FROM habitmetric
LEFT JOIN habitname ON habitname_id = habitname.id
GROUP BY ("date", habitname.name);
