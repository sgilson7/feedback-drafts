// The page draws what the core returns and decides nothing.
// Every call goes through the shim, which returns {"ok": ...} or {"error": "..."}.
// The page keeps the core's state object as it came back and hands it to the
// next call; it never reads a rule out of it or changes it.
import init, { load, start, act, rubric_file, draft_filename } from "./pkg/tool_shim.js";

const el = (id) => document.getElementById(id);

let strings = null;
let state = null;
let view = null;
let lastDraft = null;
let lastUrl = null;
// References to the editor and choice elements, rebuilt when the rubric's
// shape changes (start, open, add, remove) and updated in place otherwise.
let editorRefs = [];
let choiceRefs = [];

function call(fn, ...args) {
  return JSON.parse(fn(...args));
}

function showMessage(text) {
  el("message").textContent = text;
}

// Puts a value in a box unless the teacher is typing in it.
function sync(input, value) {
  const text = String(value);
  if (document.activeElement !== input && input.value !== text) {
    input.value = text;
  }
}

function accept(result, rebuild) {
  if (result.error) {
    showMessage(result.error);
    return;
  }
  showMessage("");
  state = result.ok.state;
  view = result.ok.view;
  draw(rebuild);
}

function send(action, rebuild) {
  accept(call(act, JSON.stringify(state), JSON.stringify(action)), rebuild);
}

function make(tag, className) {
  const node = document.createElement(tag);
  if (className) {
    node.className = className;
  }
  return node;
}

function labelled(caption, input) {
  const label = make("label", "field");
  const span = make("span");
  span.textContent = caption;
  label.append(span, input);
  return label;
}

function textInput(onInput) {
  const input = make("input");
  input.type = "text";
  input.autocomplete = "off";
  input.addEventListener("input", () => onInput(input.value));
  return input;
}

function buildEditor() {
  const box = el("criteria");
  box.replaceChildren();
  editorRefs = view.criteria.map((criterion, i) => {
    const set = make("fieldset", "criterion");
    const legend = make("legend");
    const name = textInput((text) => send({ kind: "set_criterion_name", criterion: i, text }, false));
    set.append(legend, labelled(strings.criterion_name, name));
    const levels = criterion.levels.map((level, j) => {
      const row = make("div", "level");
      const label = textInput((text) => send({ kind: "set_level_label", criterion: i, level: j, text }, false));
      const points = make("input", "points");
      points.type = "text";
      points.inputMode = "numeric";
      points.autocomplete = "off";
      points.addEventListener("change", () => send({ kind: "set_level_points", criterion: i, level: j, text: points.value }, false));
      const comment = make("textarea");
      comment.addEventListener("input", () => send({ kind: "set_level_comment", criterion: i, level: j, text: comment.value }, false));
      row.append(
        labelled(strings.level_label, label),
        labelled(strings.level_points, points),
        labelled(strings.level_comment, comment),
      );
      set.append(row);
      return { label, points, comment };
    });
    const remove = make("button", "remove");
    remove.type = "button";
    remove.textContent = strings.remove_criterion;
    remove.addEventListener("click", () => send({ kind: "remove_criterion", criterion: i }, true));
    set.append(remove);
    box.append(set);
    return { legend, name, levels, remove };
  });
}

function buildChoices() {
  const box = el("choices");
  box.replaceChildren();
  choiceRefs = view.criteria.map((criterion, i) => {
    const set = make("fieldset", "choice");
    const legend = make("legend");
    set.append(legend);
    const options = criterion.levels.map((level, j) => {
      const label = make("label", "option");
      const radio = make("input");
      radio.type = "radio";
      radio.name = "choice-" + i;
      radio.addEventListener("change", () => send({ kind: "choose", criterion: i, level: j }, false));
      const text = make("span");
      label.append(radio, text);
      set.append(label);
      return { radio, text };
    });
    box.append(set);
    return { legend, options };
  });
}

function draw(rebuild) {
  if (rebuild) {
    buildEditor();
    buildChoices();
  }
  el("sample-banner").hidden = !view.sample;
  sync(el("student-name"), view.name);
  sync(el("note"), view.note);
  sync(el("rubric-title"), view.title);
  sync(el("opening"), view.opening);
  sync(el("closing"), view.closing);
  el("show-points").checked = view.show_points;

  view.criteria.forEach((criterion, i) => {
    const editor = editorRefs[i];
    editor.legend.textContent = criterion.display_name;
    sync(editor.name, criterion.name);
    editor.remove.disabled = !view.can_remove;
    criterion.levels.forEach((level, j) => {
      sync(editor.levels[j].label, level.label);
      sync(editor.levels[j].points, level.points);
      sync(editor.levels[j].comment, level.comment);
      const option = choiceRefs[i].options[j];
      option.text.textContent = level.choice_text;
      option.radio.checked = level.chosen;
    });
    choiceRefs[i].legend.textContent = criterion.display_name;
  });

  const hasDraft = view.draft !== null;
  el("missing").textContent = view.missing_message || "";
  el("missing").hidden = hasDraft;
  el("draft-area").hidden = !hasDraft;
  if (hasDraft && view.draft !== lastDraft) {
    el("draft").value = view.draft;
    el("copy-status").textContent = "";
  }
  lastDraft = view.draft;
}

function saveFile(filename, text, type) {
  if (lastUrl) {
    URL.revokeObjectURL(lastUrl);
  }
  lastUrl = URL.createObjectURL(new Blob([text], { type }));
  const link = make("a");
  link.href = lastUrl;
  link.download = filename;
  document.body.append(link);
  link.click();
  link.remove();
}

function wire() {
  el("student-name").addEventListener("input", () => send({ kind: "set_name", text: el("student-name").value }, false));
  el("note").addEventListener("input", () => send({ kind: "set_note", text: el("note").value }, false));
  el("rubric-title").addEventListener("input", () => send({ kind: "set_title", text: el("rubric-title").value }, false));
  el("opening").addEventListener("input", () => send({ kind: "set_opening", text: el("opening").value }, false));
  el("closing").addEventListener("input", () => send({ kind: "set_closing", text: el("closing").value }, false));
  el("show-points").addEventListener("change", () => send({ kind: "set_show_points", on: el("show-points").checked }, false));
  el("add-criterion").addEventListener("click", () => send({ kind: "add_criterion" }, true));
  el("next-student").addEventListener("click", () => send({ kind: "next_student" }, true));

  el("open-file").addEventListener("change", async () => {
    const file = el("open-file").files[0];
    el("open-file").value = "";
    if (file) {
      send({ kind: "open_rubric", text: await file.text() }, true);
    }
  });

  el("save-rubric").addEventListener("click", () => {
    const result = call(rubric_file, JSON.stringify(state));
    if (result.error) {
      showMessage(result.error);
    } else {
      saveFile(result.ok.filename, result.ok.text, "application/json");
    }
  });

  el("save-draft").addEventListener("click", () => {
    const result = call(draft_filename, JSON.stringify(state));
    if (result.error) {
      showMessage(result.error);
    } else {
      saveFile(result.ok, el("draft").value, "text/plain");
    }
  });

  el("copy").addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(el("draft").value);
      el("copy-status").textContent = strings.copied;
    } catch (refused) {
      el("draft").focus();
      el("draft").select();
      el("copy-status").textContent = strings.copy_failed;
    }
  });
}

async function startPage() {
  await init();
  const loaded = call(load, await (await fetch("data/strings.json")).text());
  if (loaded.error) {
    showMessage(loaded.error);
    document.body.dataset.ready = "true";
    return;
  }
  strings = loaded.ok;
  document.title = strings.title;
  el("title").textContent = strings.title;
  document.querySelectorAll("[data-text]").forEach((node) => {
    node.textContent = strings[node.dataset.text];
  });
  const sample = await (await fetch("data/sample-rubric.json")).text();
  accept(call(start, sample), true);
  if (state) {
    wire();
  }
  document.body.dataset.ready = "true";
}

startPage();
