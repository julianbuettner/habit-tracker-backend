# Habit tracking for quantifiable habits


## Okay what?
This is meant to be a backend maintaining a postgres
database which can be used by Grafana to display your development of habits.

It is supposed to track things like
- productive hours (studying, working)
- healthy meals (e.g. salad made with effort)
- push ups

with counterparts like
- guilty pleasure time hours (Netflix)
- unhealthy meals (table of chocolate)


## Screenshots
Here is a demo of food habit tracking.
One plus unit could mean one vegetable and one minus unit could
mean one piece of cake.  
You could also choose e.g. one plus unit to be 10g of Protein and one minus
unit to be 5g of sugar.  
Choose what you think makes sense for you. Go wild.

![](https://github.com/julianbuettner/habit-tracker-backend/raw/main/screenshots/grafana-food.png)


## Grafana Dashboard example
Check out the Grafana dashboard example `grafana-dashboard.json`.  


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


