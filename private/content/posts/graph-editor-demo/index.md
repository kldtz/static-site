---
title: "Demo: Graph Editor"
date: 2020-11-21T10:24:00+01:00
description: "Live demo of an improved visual graph editor built with D3.js"
link:
  - "/apps/graph-editor/graph.css"
---

[Graph-editor](https://github.com/kldtz/graph-editor) is an improved reimplementation of Colorado Reed's [directed-graph-creator](https://github.com/cjrd/directed-graph-creator).

<div style="margin-bottom: -80px;">
<svg id="graph" width=100% height=auto style="border: 1px solid" viewbox="0 0 800 600"></svg>
<div id="toolbox" style="top: -100px; position: relative;">
    <input type="file" id="select-file">
    <input id="upload-input" type="image" title="upload graph" src="/img/graph-editor/upload-icon.png" alt="upload graph">
    <input type="image" id="download-input" title="download graph" src="/img/graph-editor/download-icon.png" alt="download graph">
    <input type="image" id="delete-graph" title="delete graph" src="/img/graph-editor/trash-icon.png" alt="delete graph">
</div>
</div>

You can try the following actions (not optimized for mobile devices):

* Scroll to zoom in or out
* Drag whitespace to move graph  
* Shift-click on whitespace to create a node
* Shift-click on a node and drag to another node to connect them with a directed edge
* Shift-click on node or edge to change its label
* Click on node or edge and press backspace/delete to delete
* Upload/download graph in JSON format
* Delete whole graph

### Why?

I was looking for a tool with which non-technical folks could create and edit directed graphs, something that is intuitive to use and extensible, as I would need to add support for arbitrary attributes on nodes and edges and some other custom functionality. After reading a bit online and trying different options, I liked Colorado Reed's [directed-graph-creator](https://github.com/cjrd/directed-graph-creator) the most. However, the last commit was from 2014, the code used an old D3 version, and I noticed a few bugs. So I decided to reimplement the graph with modern JavaScript syntax and the latest version of D3. I was able to reuse large parts of the original code, however, for some problems I had to make adjustments or find different solutions. I fixed bugs I stumbled upon and added optional edge labels and arrow heads that match the color of the arrow's path element.



<script src="https://d3js.org/d3.v6.min.js"></script>
<script src="/lib/FileSaver.min.js"></script>
<script src="/apps/graph-editor/graph.js"></script>