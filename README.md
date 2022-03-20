# Habit tracking for quantifiable habits

## Screenshots

## Okay what?
This is meant to be a backend maintaining a postgres
database which can be used by grafana to display your development of habits.

It is supposed to track things like
- productive hours (studying, working)
- healthy meals (e.g. salad made with effort)
- push ups

with counterparts like
- guilty pleasure time hours (Netflix)
- unhealthy meals (table of chocolate)

## Grafana Dashboard example
Check out Grafana dashboard example `grafana-dashboard.json`.


## Setup

Create a configuration at `~/.habit-tracker.yaml`.

```yaml
---

postgres: "postgres://habit:habit@localhost:5432/habit"

auth-whitelist:
  - this-is-like-an-unhashed-password
  - too-lazy-for-user-management

```

Then run the release binary.


