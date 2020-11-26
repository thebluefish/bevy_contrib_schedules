# Bevy Schedule Runner

---

Please note this project is minimally maintained.

I originally wrote this as a temporary stop-gap until Bevy gets proper support for adding/removing systems during runtime. Ideally, it should serve as a base for writing your own schedules; and thus I wish to keep it to a very minimal implementation. I feel that this accomplishes what I set forth to do.

I will try to support new Bevy releases on crates.io, but I am not aiming to bring in additional features. When Bevy gets proper support for adding/removing systems, I will probably archive this repository. I whole-heartedly welcome anyone who wants to fork it and make something awesome.

---

A component for running systems at a different rate from the main schedule.

- Run systems at a fixed timestep
- Component schedules
  - Add and remove schedules as necessary
  - Run multiple schedules

See [examples](https://github.com/thebluefish/bevy_contrib_schedules/tree/master/examples)