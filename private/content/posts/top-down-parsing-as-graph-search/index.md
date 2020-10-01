---
title: "Top-Down Parsing as Graph Search"
date: 2018-09-30T16:35:56+02:00
description: "Visualization of top-down parsing as graph search with D3 and dagre."
link:
  - "/css/graph.css"
---

The workings of a directional top-down parser can be nicely explained in terms of a graph search problem. In the past days I played a bit with [D3](https://d3js.org/) and [dagre](https://github.com/dagrejs/dagre) and generated some graph visualizations that provide an intuitive understanding of the algorithm.

<!--more-->

## The Basics

Throughout this post I will use the following grammar taken from [Grune & Jacobs (2008, ch.&nbsp;6)](#grune-jacobs-2008):

```
S -> aB | bA        (1, 2)
A -> a | aS | bAA   (3, 4, 5)
B -> b | bS | aBB   (6, 7, 8)
```
This context-free grammar (CFG) is in [**Greibach normal form**](https://en.wikipedia.org/wiki/Greibach_normal_form) and has eight rules (vertical bars indicate alternative right-hand sides). Its language is the set of all strings with an equal number of <em>a</em>s and <em>b</em>s. 

The parser that I describe in this post is a directional top-down parser that processes its input from left to right and performs **leftmost derivations**. In a leftmost derivation we always expand the leftmost nonterminal symbol of the current **sentential form** (a combination of terminals and/or nonterminals derivable from the start symbol) with a matching rule from the grammar to create a new sentential form. Here is an example:

```
S => aB         (1)
  => aaBB       (7)
  => aabB       (5)
  => aabbbA     (2)
  => aabbba     (3)
```
We begin with the start symbol *S* and expand it using the first rule of the grammar. We obtain the sentential form *aB* and expand its first nonterminal *B* using the eighth rule of the grammar, which results in the sentential form *aaBB*. We keep expanding the first nonterminals with matching rules from the grammar. After five steps we end up with a **sentence**, a sentential form consisting only of terminals.

## Match and Predict

In our parser the expansion of a nonterminal is called the **predict** step. For each nonterminal there might be multiple different possible predictions, depending on the grammar. For now, we are not concerned with choosing among these possible predictions, we'll simply try all possibilities in no particular order.

In the example derivation above we randomly chose among all rules with the required left-hand side to derive new sentential forms until we found some sentence. To find the derivation of a specific input sentence, we need to relate our top-down predictions to the input, which our parser does in the **match** step:  If the grammar is not left-recursive and describes a language that consists of more than just the empty string, successive predictions from the start symbol eventually result in a sentential form that starts with one or more terminal symbols. At this point we can check whether the predicted terminals match the first terminal symbols of the input. If they do, we increment an index that tracks the number of matched terminals and points to the input symbol that needs to be matched next. If the predicted terminals don't match the input, we must abandon the current derivation (or at least **backtrack** to yet unexplored predictions). This succession of predict and match steps repeats until a derivation for the input is found or until all possibilities have been explored.

## Configuration Graph

We can state the parsing process above in terms of a graph search problem: The nodes of the graph are parser **configurations**<a id="fn-1"></a><sup>[1](#1)</sup> consisting of a sentential form and an index into the input. Since the matched symbols of a sentential form are just prefixes of the input string, a configuration can be reduced to its index and the unmatched part of the sentential form (aka the **prediction stack** from which we pop the first symbol as input to match/predict and to which we push the output of match/predict). At the beginning we have one configuration consisting of the start symbol and index 0. By matching or predicting, we explore all direct successors of a configuration. For the grammar above and the sentence *aabbab* we get the following graph:

<figure class="half-full">
<svg id="static-graph" width="1000" height="600"></svg>
<figcaption><p>Configuration graph for the string <em>aabbab</em>.</p></figcaption>
</figure>

Blue nodes are the result of predictions<a id="fn-2"></a><sup>[2](#2)</sup>, orange nodes come from matching. We see an alternating color pattern because our grammar is in Greibach normal form (each right-hand side starts with a terminal). Filled nodes are configurations that actually lead to a sentence, all other nodes lead to dead ends. Because our grammar doesn't have epsilon rules, each match result is an only child. Predictions, however, are siblings as our grammar provides more than one rule for each nonterminal.

There is one final configuration that covers the whole input sentence and has an empty prediction stack (6), which means that the parser recognized the input string as being in our language. Two different paths lead to this goal configuration, which tells us that the grammar is **ambiguous**. To construct the parse trees, we can simply start with the goal configuration set (in this case it has only one element) and follow the parents up to the root node.

## Graph Traversal

The graph above contains 20 configurations that don't lead to the input sentence. Of course, we would like to avoid predicting useless configurations as much as possible. For certain grammar types we can indeed build a top-down parser that only explores useful configurations (given a lookahead of *k* input symbols). For other grammars we can at least exclude some predictions by incorporating bottom-up information. In the general case and without any additional information, however, we will simply traverse the full configuration graph using either **depth-first search (DFS)** or **breadth-first search (BFS)**.

Here is another visualization of the same configuration graph:

<div id="dynamic-graph-buttons"><button id="dfs" title="Perform depth-first search">DFS</button><button id="bfs" title="Perform breadth-first search">BFS</button></div>

<figure class="half-full">
<svg id="dynamic-graph" width="1000" height="600"></svg>
<figcaption><p>DFS and BFS on the configuration graph. Siblings are visited in reverse lexicographical order.</p></figcaption>
</figure>

When you click on one of the buttons, you can watch a DFS or BFS search on this grap where children of the same parent node (predictions) are visited in reverse lexicographical order. Obviously, this order is far from ideal. With *4:aSB*, DFS runs into a long dead end and BFS picks up a lot of useless configurations on the way. With a simple lookahead of one symbol we could already avoid *some* traps, e.g. *0:bA*, *1:bS* and  *1:b* because we know that the input starts with *aa*, but not all, compare for example *4:aSB* and *4:aB*. Considering these examples we can guess what a grammar needs to look like that allows a top-down parser to run in linear time given a lookahead of one or *k* tokens. But that would be another post.

<hr>

<a id="1" href="#fn-1"><sup>1</sup></a> I adopt the term *configuration* from Grune and Jacobs (2008, ch.&nbsp;6), but according to their definition, a configuration has an analysis stack instead of a simple index, which leads to more configurations (our example graph would be a [tree](config-tree.html)!).

<a id="2" href="#fn-2"><sup>2</sup></a> You can think of the configuration *0:S* as the prediction result of a special rule *S' -> S*.

<a id="grune-jacobs-2008"></a>Grune, Dick, & Jacobs, Ceriel J. H. (2008). Parsing Techniques - A Practical Guide (2nd ed.). Monographs in Computer Science. New York: Springer.

<script src="/lib/d3/d3.v4.min.js"></script>
<script src="/lib/dagre/dagre-d3.min.js"></script>
<script src="/lib/dagre/dagre.min.js"></script>
<script src="/js/config_dag.js"></script>

