---
title: "Demo: Fretboard Diagram Creator"
date: 2020-12-22T08:30:57+01:00
description: "Online tool for creating fretboard diagrams, e.g., for scale charts with fingerings."
link:
  - "/apps/fretboard-diagram-creator/fretboard.css?version=2"
---

<figure id="fretboard-diagram-creator" class="half-full">
  <svg xmlns="http://www.w3.org/2000/svg" id="fretboard" width=900 height=280 style="background-color: white;">

  </svg>
</figure>
<div class="menu">
  <p>
  <button id="visibility" class="button">Toggle</button>
  <button id="save-svg" class="button">Save</button>
  <button id="reset" class="button">Reset</button>
  <a id="svg-link" download="fretboard-diagram.svg"></a>
  </p>

  <p>
  <label>Start:</label>
  <input id="start-fret" type="number" class="num-input" value="1" name="startFret" min=1 max=22 maxlength="2" style="width: 5ch" autocomplete=off>
  <label>End:</label>
  <input id="end-fret" type="number" class="num-input" value="12" name="startFret" min=1 max=22 maxlength="2" style="width: 5ch" autocomplete=off>
  </p>
</div>

### Interactions

* Click one of the circles to **select** a note. The note is now highlighted by a dashed circle and will show up in your exported diagram.
* Type *r*, *g*, *b*, *d*, or *w* to **color** the selected note (red, green, blue, black, white).
* Press the *Delete* or *Backspace* key to **delete** the selected note.
* Hold *CTRL* and click into a note to **edit** its label. Hit *Enter* or click outside of the note being edited to save the changes.
* Click *Toggle* to **hide** all notes that you have not selected or make them appear again. Do this before you save the diagram to save the version you prefer.
* Click *Save* to **save** the diagram as SVG.
* Click *Reset* to **reset** your diagram and start from scratch.
* Enter *start* fret and *end* fret to **display a different part of the fretboard**. Changes outside of the displayed part are kept until the diagram is reset.

Remember that all of this only runs in your browser, so be careful about refreshing the page. Implemented in plain JavaScript, without dependencies. Example diagrams can be found [here](/posts/fretboard-diagram-creator/modes.html). Comments are on [Reddit](https://www.reddit.com/r/musictheory/comments/kows26/a_simple_online_tool_for_creating_guitar/).

<script src="/apps/fretboard-diagram-creator/fretboard.js?version=2"></script>