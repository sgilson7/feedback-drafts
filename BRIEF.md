# Brief

Stage 1, written by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The request, word for word

> You are building a static browser tool for a teacher who writes individual feedback on student projects. This directory is a copy of the Builder template: read `CLAUDE.md` and `BUILDER-SPEC.md` first, and keep to them.
>
> The tool has two parts on one page.
>
> **Rubric.** The teacher opens a rubric file (JSON) or starts from the sample rubric the page loads by default. A rubric has a title, an opening line, a closing line, a setting for whether the draft shows points, and one or more criteria; each criterion has a name and two to six levels; each level has a label, a whole number of points, and the comment the teacher wants a student at that level to read. In the page the teacher can edit any of those texts and numbers, add a criterion (it starts with copies of the first criterion's level labels and points, and empty comments), remove a criterion (not the last one), and save the rubric as a file to reuse.
>
> **Draft.** The teacher types an optional student name, picks one level for each criterion, and can type a personal note. When every criterion has a level, the tool composes a feedback draft: the opening line (with the name if one was given), one paragraph per criterion with its name, its level label, its points if the rubric shows points, and the level's comment; then the total ("N of M points") if the rubric shows points; then the personal note if any; then the closing line. The draft appears in an editable text box. The teacher edits it and copies it, or saves it as a text file. "Next student" clears the name, the levels and the note and keeps the rubric. Until every criterion has a level, the tool names the criteria still missing and shows no draft.
>
> Nothing is sent anywhere and nothing is stored between visits: the page uses no browser storage. Student names and notes stay in the page until the teacher clears them or closes it.
>
> Keep to the template's architecture and its checks: every rule in `crates/core`, a shim in `crates/shim` that decides nothing, a page in `web/` that draws what the core returns, content in `data/`, `./scripts/check.sh` as the full check.
>
> Rules:
> (a) The core depends only on serde and serde_json.
> (b) The shim decides nothing.
> (c) The page keeps no copy of a rule: composing the draft, the total, which criteria are missing, and every rubric validation and edit happen in the core.
> (d) Nothing runs anywhere but static hosting, and the page makes no request to another origin.
> (e) The core uses no floats, hash maps, or clocks, so the same inputs always give the same draft.
> (f) Every new test is broken once and watched failing before it is kept.
>
> What you may not decide alone: adding any network request, any third-party script or service, any model, any browser storage, or any way for student information to leave the teacher's device. If something seems to need one, stop and say so.
>
> Write the sample rubric yourself as clearly marked sample content: a project-report rubric with four criteria and four levels each (points 1 to 4), with a short comment for every level. The page must say on screen that the rubric is sample content to check or replace.
>
> The plan is approved in advance: write `PLAN.md`, then continue through the agent's stages without waiting. Commit after each working step with a message that says what changed. Do not push and do not deploy.

The request continues with instructions for the plan (B2), the notebook (B3), corrective dialogue (B4), checks (C4) and handoff (C5). Those are followed in `PLAN.md`, `SECOND-ORDER.md`, the tests and `HANDOFF.md`.

## The tool

A one-page static tool for a teacher writing individual feedback on student projects: edit a rubric, pick a level per criterion for one student, get a feedback draft to edit, copy or save. Used in a desktop browser (Chromium, Firefox or WebKit).

## Done means

- `./scripts/check.sh` passes, including the browser check in Chromium, Firefox and WebKit.
- Core tests with hand-computed expected values: the full draft for one set of choices on the sample rubric (with and without name, note and points), its total, the missing list for a partial set, the rubric after adding and after removing a criterion.
- Core tests that a rubric with a missing field, one or seven levels, negative or non-integer points, or removal of the last criterion is rejected with a message, never a crash.
- `tests/browser/interactions.py` drives the tool as a teacher would (see `PLAN.md`).
- `README.md` says what the tool does, how to build, test and serve it, and that the sample rubric is sample content.

## Decisions that stay with the person

- The sample rubric's wording (written by the agent, marked as sample).
- Any network request, third-party script or service, model, browser storage, or route for student information to leave the device.

## Autonomy

The agent may plan, build, test and commit without waiting; the plan is approved in advance. It must stop and say so before adding a network request, a third-party script or service, a model, browser storage, or any way for student information to leave the device. It may never push or deploy.

Publish grant, 2026-10-19, in the person's words: "ok execute all of the changes you recommend to increase feasabilitiy, i approve make deploy in all builder made repos, and make deploy in the website, keep track of all changes made in the design log here" This lifts "It may never push or deploy" for T1 (`TRIAGE.md`) only: push the commit, publish `dist/` to gh-pages, and run the browser check against the live address. Everything else above still stands.

## Allowed core dependencies

`serde`, `serde_json` (reference default). No others.
