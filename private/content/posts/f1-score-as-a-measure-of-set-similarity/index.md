---
title: "F1 Score as a Measure of Set Similarity"
date: 2019-03-26T18:17:46+01:00
description: "F1 score defined in terms of two sets (Dice coefficient)."
features:
    - MathJax
---

In machine learning the F1 score is often used to evaluate the performance of classifiers. In this context it is usually thought of in terms of true positives, false positives and false negatives from a binary contingency table. However, thinking of the F1 score as a simple measure of set similarity may give us better intuitions about its properties and limitations.

## F1 Score from Two Sets

Given a set $R$ of expected or **real elements** and a set $P$ of **predicted elements**, we can define the sets of **true positives** $TP$, **false positives** $FP$ and **false negatives** $FN$ via basic set operations.

<div>
$$\begin{align}
TP &= R \cap P\\
FP &= P \setminus R\\
FN &= R \setminus P
\end{align}$$
</div>

Based on these sets, we can define **precision** $p$, aka *confidence* or *true positive accuracy*, and **recall** $r$, aka *sensitivity* or *true positive rate*.

<div>
$$\begin{align}
p &= \frac{|R \cap P|}{|P|} = \frac{|TP|}{|TP| + |FP|}\\
r &= \frac{|R \cap P|}{|R|} = \frac{|TP|}{|TP| + |FN|}
\end{align}$$
</div>

The **F1 score** is usually introduced as the harmonic mean of precision and recall or in terms of true positives, false positives and false negatives. The latter avoids division by zero if $|TP| = 0$ as long as $|FP| \neq 0$ or $|FN| \neq 0$. 

<div>
$$\begin{align}
F_1 &= \frac{2pr}{p + r}\\
    &= \frac{2|TP|}{2|TP| + |FP| + |FN|}
\end{align}$$
</div>

## Dice Coefficient

If we don't care about precision and recall, we can state $F_1$ most easily in terms of the two sets $R$ and $P$ directly, namely as the cardinality of the intersection of $R$ and $P$ (the number of true positives) normalized by the arithmetic mean of $|R|$ and $|P|$.

$$
F_1 = \frac{2|R \cap P|}{|R| + |P|} = \frac{|R \cap P|}{\frac{1}{2}(|R| + |P|)}
$$

In this form the $F_1$ score is known under the name of **(Sørensen–)Dice coefficient**. It is symmetric, thus the semantics attached to $R$ and $P$ don't really matter. Since we have $0 \leq F_1 \leq 1$, we can define the **Dice distance** $d_D$ as a semimetric of set distance within the same range by subtracting $F_1$ from one:

$$
d_D = 1 - F_1
$$

It is a semimetric because it satisfies non-negativity, identity of indiscernibles and symmetry, which is easy to see from the definition of $F_1$ in terms of $R$ and $P$, but does not satisfy the triangle inequality, e.g. for the three sets $A = \\{a\\}$, $B = \\{b\\}$, and $AB = \\{a, b\\}$ we get $d_D(A, B) = 1$, $d_D(A, AB) = \frac{1}{3}$ and $d_D(AB, B) = \frac{1}{3}$, thus
$$
d_D(A,B) \not\leq d_D(A, AB) + d_D(AB, B).
$$

## Digression: Jaccard Coefficient

The Dice coefficient is not the only measure of set similarity. Another (earlier) example of this kind of statistic is the **Jaccard coefficient** $J$, aka *Intersection over Union (IoU)*: the cadinality of the intersection of $R$ and $P$ divided by the cardinality of their union. It may also be stated in terms of true positives, false negatives and false positives, or in terms of $F_1$.

<div>$$
\begin{align}
J &= \frac{|R \cap P|}{|R \cup P|}\label{intersection_over_union}\\
&= \frac{|TP|}{|TP| + |FN| + |FP|}\label{jaccard_tp_fn_fp}\\
&= \frac{F_1}{2 - F_1}\label{jaccard_f1}\\
\end{align}
$$</div>

The **Jaccard distance** $d_J = 1 - J$ is a metric, as it satisfies not only non-negativity, identity of indiscernibles and symmetry, but also the triangle inequality (in contrast to $d_D$).


## Set Similarity vs. Classifier Performance

Assume that $R$ and $P$ are two subsets of another set $S$, that is, we no longer have only two sets $R$ and $P$, but two binary partitions of $S$, nameley the expected or real partition $X = \\{R, S \setminus R\\}$ and the predicted partition $Y = \\{P, S \setminus P\\}$. With this superset $S$, we can define the set of **true negatives** $TN$ as the intersection of the relative complements of $R$ and $P$ in $S$:

<div>$$
TN = (S \setminus R) \cap (S \setminus P)
$$</div>

At this point we have all the information necessary for filling out the traditional binary contingency table.

|      | R  | S \ R | S |
|-----|:-----:|:-----:|:-----:|
| **P**  | \|TP\| | \|FP\| | \|P\| |
| **S \ P** | \|FN\| | \|TN\| | \|S \ P\|
| **S** | \|R\| | \|S \ R\| | \|S\|

Note that $TN$ is not defined if we have only two sets and thus does not affect the F score. For some tasks this is a desired property, e.g., in Information Retrieval (IR) or Named Entity Recognition (NER), where the set of true negatives can be ill-defined, uncountable or just extremely large. 

If we have a fixed set of items and our classifier assigns exactly one label per item, we have $S$. When we calculate F1 over all classes, for each item with matching labels we get a true positive, otherwise a false positive and a false negative, thus by definition $|TN| = 0$. In this case (micro-)F1 equals accuracy (the number of items where labels match divided by the total number of items) with all its limitations ([accuracy paradox](https://en.wikipedia.org/wiki/Accuracy_paradox)).


