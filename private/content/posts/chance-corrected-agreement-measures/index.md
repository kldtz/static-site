---
title: "Chance-Corrected Agreement Measures for a Fixed Number of Units"
date: 2019-11-07T19:38:14+02:00
description: "Comparison of inter-rater agreement measures for a fixed number of nominal ratings on a fixed number of units."
features:
    - MathJax
---

In this post I introduce and discuss three common 'chance-corrected' measures of inter-rater reliability/agreement for a fixed number of nominal ratings on a fixed number of units. All definitions are based on either continguency or coincidence matrices as defined by [Krippendorff (2004)](#krippendorff-2004) to keep the formulas simple and comparable.

## Preliminaries

Assume we have $r$ raters, each assigning one out of $c$ classes of nominal labels to each of $u$ units. We get the following label matrix:

<div>$$
L = \begin{pmatrix}
l_{11} & l_{12} & l_{13} & \ldots & l_{1u} \\
l_{21} & l_{22} & l_{23} & \ldots & l_{2u} \\
l_{31} & l_{32} & l_{33} & \ldots & l_{3u} \\
\vdots & \vdots & \vdots & \vdots & \vdots \\
l_{r1} & l_{r2} & l_{r3} & \ldots & l_{ru} \\
\end{pmatrix}
$$</div>

Where $l_{ij}$ is the label assigned by the $i$th rater to the $j$th unit.

### Special Case: Two Raters

For $r=2$ we can compute a $(c \times c)$ [continguency matrix](https://en.wikipedia.org/wiki/Contingency_table) $A$ from our label matrix $L$, where $a_{ij}$ is the number of units for which the first annotator chose label $i$ while the second annotator chose label $j$. In general, this matrix is asymmetric. The sum of all its entries equals the number of units:

<div>$$
n_A = \sum_{i=1}^c \sum_{j=1}^c a_{ij} = u
$$</div>

The coincidence matrix $B$ is a symmetric version of $A$, where $b_{ij}$ gives us the number of times the first annotator chose label $i$ while the second annotator chose label $j$ or *vice versa*. Thus, we have $B = A + A^T$ and the sum of all entries of $B$ equals twice the number of units (or the total number of labels assigned by both annotators).

### General Case: Multiple Raters

Since $A$ is asymmetric, there is no simple way of generalizing it to multiple raters. 

To generalize $B$, we can define it as the sum of the coincidence matrices between all 2-combinations of annotators normalized by $r-1$.<a id="fn-1"></a><sup>[1](#1)</sup> Each coincidence matrix between two annotators contributes $2u$ ratings, thus after multiplication with the binomial coefficient and normalization we arrive at the number of labels assigned by all annotators ($ru$). 

<div>$$
n_B = \sum_{i=1}^c \sum_{j=1}^c b_{ij} = \frac{1}{r-1}{r \choose 2} 2u = ru
$$</div>

Equivalently, we can say that we add up the contingency matrices between all ordered pairs of annotators and normalize the result by $r-1$. Since we have $r(r-1)$ ordered pairs of annotators, normalization again ensures that the sum over all elements of $B$ equals $ru$.

<div>$$
n_B = \sum_{i=1}^c \sum_{j=1}^c b_{ij} = \frac{1}{r-1} r(r-1) u  = ru
$$</div>

For $r=2$ there is only a single 2-combination and $r - 1 = 1$, thus the previous definition of $B$ remains unchanged.

For convencience we define $m_{i\cdot} = \sum_{j=1}^c m_{ij}$ as the sum over the $i$th row of a matrix $M$ and $m_{\cdot j} = \sum_{i=1}^c m_{ij}$ as the sum over the $j$th column of $M$. Since $B$ is symmetric, row sums equal column sumns $b_{i \cdot} = b_{\cdot i}$ (the number of assignments of label $i$ across all raters and units).

## Agreement Measures

All the agreement measures in the following can be stated in the general form

<div>$$
\kappa / \pi / \alpha = \frac{p_o - p_e} {1 - p_e}
$$</div>

where $p_o$ is the observed agreement and $p_e$ is the expected or chance agreement.

### Cohen's Kappa

Cohen's Kappa $\kappa_C$ [(Cohen, 1960)](#cohen-1960) is only defined for two raters and can be computed from the continguency matrix $A$. All items on which both annotators agree can be found on the diagonal of $A$ and are normalized by the sum over all elements of $A$ to obtain the observed agreement.

<div>$$
p_o = \frac{tr(A)}{n_A}
$$</div>

For the expected agreement we sum over the product of row and column totals for each label category and normalize by the squared sum over all entries of $A$.

<div>$$
p_e = \frac{1}{n^2_A} \sum_{i=1}^c a_{i \cdot} a_{\cdot i}
$$</div>

From the definition of $p_e$ we can see that the expected agreement will be higher (and hence $\kappa_C$ will be lower) if the row and column totals (or marginal totals) of $A$ have a similar distribution. Furthermore, we can see that imbalanced class distributions will affect $\kappa_C$. For details about problems with $\kappa_C$, see [Feinstein (1990)](#feinstein-1990) and [Byrt et al. (1993)](#byrt-et-al-1993).


### Scott's Pi and Fleiss' Kappa

The observed agreement of Fleiss' Kappa $\kappa_F$ ([Fleiss, 1971](#fleiss-1971)) is the trace of the coincidence matrix $B$ normalized by the sum over all its elements.

<div>$$
p_o = \frac{tr(B)}{n_B}
$$</div>

To obtain the chance agreement for the $i$th category, we calculate the square of the proportion of all assignments to the $i$th category. The sum over the chance agreement for all $c$ categories gives us $p_e$.<a id="fn-2"></a><sup>[2](#2)</sup>

<div>$$
p_e = \sum_{i=1}^c \left( \frac{b_{i \cdot}}{n_B} \right)^2 = \frac{1}{n^2_B} \sum_{i=1}^c \left( b_{i \cdot} \right)^2
$$</div>

This definition of $p_e$ assumes a *multinomial distribution* (*binomial* for the special case of $c=2$): a label $l$ is selected by two raters in two independent trials with a fixed probability. Note that both $p_o$ and $p_e$ in $\kappa_F$ are invariant to scaling of $B$, hence we can ignore the normalization factor $\frac{1}{r-1}$.

Despite the name suggesting a relation between Fleiss' Kappa and Cohen's Kappa, and Fleiss (1971) claiming to generalize Cohen's Kappa, $\kappa_F$ is actually the generalization of Scott's Pi $\pi_S$ ([Scott, 1955](#scott-1955)) to multiple raters. Or inversely, $\pi_S$ is the special case of $\kappa_F$ where $r=2$.

### Krippendorff's Alpha

Observed agreement for Krippendorff's Alpha $\alpha_K$ (Krippendorff, 2004) is identical to that of Fleiss' Kappa. 

<div>$$
p_o = \frac{tr(B)}{n_B}
$$</div>

The definition of $p_e$ is very similar to Fleiss' chance agreement, however, here a *(multivariate) hypergeometric distribution* is assumed: the label choice of the second rater depends on the choice of the first rater. For each category, we calculate the probability of drawing two times a label of this category from the multiset of all observed labels without replacement and add up the results (since these are mutually exclusive events).

<div>$$
p_e = \frac{\sum_{i=1}^c b_{i\cdot} (b_{i\cdot} - 1)}{n_B (n_B - 1)}
$$</div>

Alternatively, we may also define $p_e$ for the unnormalized matrix $B' = (r-1) B$:

<div>$$
p_e = \frac{\sum_{i=1}^c b'_{i\cdot} (b'_{i\cdot} - r + 1)}{n_{B'} (n_{B'} - r + 1)}
$$</div>


## Discussion

The values of all three measures usually lie between 0 and 1, although negative values may occur in cases of systematic disagreement (if $p_e > p_o$). If we only observe labels from a single category, all three expect perfect agreement by chance ($p_e = 1$) and none of them is defined due to divison by zero. $\kappa_C$'s sensitiveness to different data distributions makes its values difficult to compare and interpret across different tasks, and penalizing raters who agree on marginal distributions seems counter-intuitive ([Brennan & Prediger, 1981](#brennan-prediger-1981)). For smaller samples, $\alpha_K$ will be slightly higher than for larger samples (ceteris paribus) due to the computation of expected agreement without replacement. In many cases, the difference between $\kappa_F$ and $\alpha_K$ will be negligible ($\alpha_K$ approaches $\kappa_F$ as the sample size approaches infinity).

All three measures compute expected agreement based purely on 'chance' given the observed frequencies, treating these observations as given. This can be misleading. Depending on the annotation process, it may be better to compute a simple agreement measure without 'chance correction' and compare it to several different baselines that explicitly model chance or possible annotation biases (e.g., annotators choosing a label that requires minimal effort in the UI when in doubt). In general, a single measure that summarizes information about multiple classes and incorporates some form of chance agreement is not very helpful in case of low agreement, where we would like to know how we can improve the annotation process. Finally, we might ask [why we need to correct for chance agreement](https://john-uebersax.com/stat/kappa2.htm) only when measuring agreement between human annotators. Why not when we compute accuracy or F-score between classifier predictions and a gold standard? (Here we usually compare against different baselines.)

<hr>

<a id="1" href="#fn-1"><sup>1</sup></a> Here I use the definition of Krippendorff (2004, p. 231). He normalizes by the number of ratings per unit, but since we deal with a fixed number $r$ of ratings, this is equivalent.

<a id="2" href="#fn-2"><sup>2</sup></a> Fleiss (1971) defines $p_o = \frac{1}{u} \sum_{i=1}^u \frac{1}{r(r-1)} \sum_{j=1}^c n_{ij} (n_{ij} - 1)$ and $p_e = \sum_{j=1}^c \left( \frac{1}{ur} \sum_{i=1}^u n_{ij} \right)^2$, where $n_{ij}$ denotes the number of raters who assigned the $i$th unit to the $j$th category. It's easy to show that these definitions are equivalent to the ones given above ($\frac{1}{r-1} \sum_{i=1}^u n_{ij} (n_{ij} - 1) = b_{jj}$ and $\sum_{i=1}^u n_{ij} = b_{j \cdot} = b_{\cdot j}$).

<hr>

<a id="brennan-prediger-1981"></a> Brennan, Robert L., & Prediger, Dale J. (1981). Coefficient kappa: Some uses, misuses, and alternatives. Educational and Psychological Measurement, 41, 687-699.

<a id="byrt-et-al-1993"></a> Byrt, T., Bishop, J., & Carlin, J. B. (1993). Bias, prevalence and kappa. Journal of clinical epidemiology, 46(5), 423-429.

<a id="cohen-1960"></a> Cohen, J. (1960). A coeffcient of agreement for nominal scales. Educational and psychological measurement, 20(1):37-46.

<a id="feinstein-1990"></a> Feinstein, A.R, Cicchetti, D.V. (1990). High agreement but low kappa: I. The problems of two paradoxes. Journal of Clinical Epidemiology; 43: 543-548.

<a id="fleiss-1971"></a>Fleiss, J. L. (1971). Measuring nominal scale agreement among many raters. Psychological
bulletin, 76(5):378.

<a id="krippendorff-2004"></a> Krippendorff, K. (2004). Content Analysis, An introduction to its methodology. 2nd Edition. Thousand Oaks, CA: Sage Publications.

<a id="scott-1955"></a> Scott, W. A. (1955). Reliability of content analysis: The case of nominal scale coding.
Public Opinion Quarterly, 19, 321-325.