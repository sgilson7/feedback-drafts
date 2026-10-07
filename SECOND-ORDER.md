# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). **Status** is *done*, *open*, or *person*.

| # | Kind | What was assumed or found | What is true | What changed | Status |
| --- | --- | --- | --- | --- | --- |
| 1 | worklist | The brief says "a whole number of points" without a range. | I chose 0 to 1000 (0 counts as whole; 1000 keeps totals small and catches typos). | PLAN decision 3. | person |
| 2 | worklist | The brief says the opening line carries the name "if one was given" without saying where. | I chose a `{name}` marker; without a name the marker and the space before it are removed; with no marker the name is put in front ("Jordan, ..."). | PLAN decision 10. | person |
| 3 | worklist | The draft box is editable, but the draft is recomposed when an input changes. | Edits in the box are replaced when the composed draft changes. The page says so beside the box. | PLAN decision 15. | person |
| 4 | finding | Empty texts: the brief does not say whether a criterion name or level label may be empty. | Allowed, with "Criterion N" / "Level N" shown in the draft and missing list. A new criterion starts with an empty name. | PLAN decision 4. | done |
| 5 | finding | Draft wording ("Total:", "point"/"points", "Choose a level for:") is reader-visible text in the core, not in `data/`. | D5 puts reader-visible text in `data/`; the template's own convention keeps rule-dependent wording beside the rule. I kept it in the core so the draft is fully tested there. | PLAN decision 17. | done |
