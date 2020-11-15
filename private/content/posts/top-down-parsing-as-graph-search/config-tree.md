---
title: "Configuration Tree"
date: 2018-10-05T17:50:22+02:00
link:
  - "/css/graph.css"
---

Expanding on a footnote in the post [Top-Down Parsing as Graph Search](/posts/top-down-parsing-as-graph-search): 

If each configuration has not only an index, but a full analysis stack, our graph is a tree, as one node cannot have exactly one analysis stack and more than one parent:

<figure class="half-full">
<svg id="tree" width="1000" height="600"></svg>
<figcaption><p>Tree of configurations with full analysis stack.</p></figcaption>
</figure>

With this definition we get more configurations, meaning that the parser has to perform more steps for the same result. For example, *4:aB* and its descendants appear twice in the tree above, but only once in the directed acyclic graph presented in the post.

<script src="/lib/d3/d3.min.js"></script>
<script src="/js/config_tree.js"></script>
