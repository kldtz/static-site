---
title: "Demo: Fretboard Diagram Creator"
date: 2020-12-22T08:30:57+01:00
description: "Online tool for creating fretboard diagrams, e.g., for scale charts with fingerings."
link:
  - "/fretboard-diagram-creator/fretboard.css"
---

<figure id="fretboard-diagram-creator" class="half-full">
  <svg xmlns="http://www.w3.org/2000/svg" id="fretboard" width=900 height=290 style="background-color: white;">

  </svg>
</figure>
<div class="menu">
    <button id="visibility" class="button">Toggle</button>
    <button id="save-svg" class="button">Save</button>
    <button id="reset" class="button">Reset</button>
    <a id="svg-link" download="fretboard-diagram.svg"></a>
</div>

### Interactions

* Click one of the circles to **select** a note. The note is now highlighted by a dashed circle and will show up in your exported diagram.
* Type *r*, *g*, *b*, or *w* to **color** the selected note (red, green, blue, white).
* Press the *Delete* or *Backspace* key to **delete** the selected note.
* Hold *CTRL* and click into a note to **edit** its label. Hit *Enter* or click outside of the note being edited to save the changes.
* Click *Toggle* to **hide** all notes that you have not selected or make them appear again. Do this before you save the diagram to save the version you prefer.
* Click *Save* to **save** the diagram as SVG.
* Click *Reset* to **reset** your diagram and start from scratch.

Remember that all of this only runs in your browser, so be careful about refreshing the page. Implemented in plain JavaScript, without dependencies.

<script src="/fretboard-diagram-creator/fretboard.js"></script>