# Triage

Stage 7, written by the person after using the build. `C4 Verify against hand-computed cases`.

One row per concrete change. Say what you did, what you expected, and what happened. Compare at least one result with a case you worked out by hand.

| # | What I did | Expected | What happened | Change wanted | Status |
| --- | --- | --- | --- | --- | --- |
| T1 | 2026-10-19. Sam, after a K-12 classroom feasibility review of the tool, wrote: "ok execute all of the changes you recommend to increase feasabilitiy, i approve make deploy in all builder made repos, and make deploy in the website, keep track of all changes made in the design log here" | A school can see on what terms it may use, copy and adapt the tool. | The repository had no licence file, so by default no one else had permission to reuse the code. | Add an MIT licence, the same text as Sam's other tools ("Copyright (c) 2026 Sam Gilson"), and say so in the README. | done: `LICENSE`, README "Licence". Notebook rows 21 and 22. |
