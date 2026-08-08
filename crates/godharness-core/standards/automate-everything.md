---
id: automate-everything
title: Automate Everything You Can
keywords: [automate, automation, manual step, repeated task]
paths: []
must-read: false
supersedes: []
relates-to: []
---

## Rule

If a task will be repeated, automate it. Preference order: CI (runs on every push or schedule,
can't be skipped or forgotten), then a hook that fires inline during a session for things CI
can't reach, then a standalone script as a last resort, since that still requires someone to
remember to run it.

## Why

Manual repeated tasks get skipped under time pressure. CI automation runs itself regardless of
who's working or how rushed they are; hooks depend on the right event firing; scripts depend on
a human or agent remembering they exist and running them - the weakest guarantee of the three.

## How to apply

Noticing the same manual step done more than once is the signal to automate it, not just note it
down. Default to CI for anything that should run regardless of who's working locally. Reach for a
hook only when the task must happen inline during a session and CI can't reach it. Reach for a
standalone script only when neither of the above can trigger it, and treat that as a gap to close
later rather than the final answer.
