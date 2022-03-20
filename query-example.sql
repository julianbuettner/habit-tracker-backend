SELECT q.date as "time", coalesce(positive, 0) FROM
(
  SELECT
    calendar::date AS "date", tt.positive
  FROM
    generate_series(to_timestamp(1637780877), to_timestamp(1647880877), '1 day'::interval) calendar
  LEFT JOIN
  (
    SELECT
      date("date") AS "date",
      sum("posunits") AS "positive"
    FROM habitdays
    WHERE name = 'Push Ups'
    GROUP BY "date"
    ORDER BY 1
  ) AS tt
  ON calendar.date = tt.date
) AS q;
