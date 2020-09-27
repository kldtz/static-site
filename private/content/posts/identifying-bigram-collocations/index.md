---
title: "Identifying Bigram Collocations"
date: 2020-09-27T10:11:59+02:00
features:
    - MathJax
---

After two posts on FSTs, this text deals with another piece of old (or *classic*) technology from computational linguistics. It's my digestion of [Manning and Schütze's (1999)](#manning-schütze-1999) chapter on collocations and some cited/related papers.

By the most simple definition, a collocation is a sequence of words that occur together more often then expected by chance. For a bigram $w_1 w_2$, we can formalize this using conditional probabilities as $P(w_1) P(w_2|w_1) > P(w_1) P(w_2)$, or simply $P(w_2|w_1) > P(w_2)$. We can estimate the probabilties from a corpus by counting unigram and bigram types: 

<div>$$
P(w_1) = \frac{C(w_1)}{N}\\
P(w_2) = \frac{C(w_2)}{N}\\
P(w_2|w_1) = \frac{C(w_1, w_2)}{C(w_1)}
$$</div>

where $C(...)$ is a count from our corpus and $N$ is the number of unigrams/bigrams in the corpus<a name="fn-1"></a><sup>[1](#1)</sup>. However, if we want to use these probabilities to find collocations in a corpus, we face two problems:

1. $P(w_2|w_1) > P(w_2)$ could hold by chance, especially if our corpus is small or if we are looking at infrequent pairs of words.
2. Language is not random, so most sequences of words occur more often than we would expect by the frequency of their individual words. That is, our definition doesn't really capture what linguists understand under the term *collocation*.

The first problem is a question of statistical significance, so an obvious idea is to apply a statistical test that tells us whether the difference between the conditional probability and the probability of two independent events is significant.  Problem two tells us that significance is not enough, we also need to have a look at effect size. We will use the test statistic to rank bigrams from most likely to least likely collocation and then need to find a threshold for classification (usually this is done empirically by inspecting the ranked bigrams of a corpus and choosing a cutoff).


## Hypothesis Testing

Before picking a test statistic, we need to think about the distribution we are dealing with. We can view a text as a sequence of *N* (overlapping) bigrams. For a given bigram $b = (w_1, w_2)$, each bigram in this sequence either equals *b* (1) or not (0). In other words, we have *N* binary random variables. Assuming independence between these random variables and identical distribution<a name="fn-2"></a><sup>[2](#2)</sup>, we can model text as a **Bernoulli process**, a sequence of independent Bernoulli trials. The number of successes (bigram equals *b*) in these N trials has a **binomial distribution**. The probability mass function (PMF) for the binomial distribution is defined as

<div>$$
b(k; n, p) = {n \choose k} p^k (1-p)^{n-k}
$$</div>

where $n \in \mathbb{N}$ is the number of trials ($N$ bigrams), $k$ (nonnegative integer less or equal $n$) is the number of successes (bigrams that equal *b*) and $p$ is the success probability of each Bernoulli trial. We have ${n \choose k}$ ways of selecting $k$ out of $n$ trials, and each way adds a probability of $p^k (1-p)^{n-k}$.

### T-test

According to the [De Moivre-Laplace theorem](https://en.wikipedia.org/wiki/De_Moivre%E2%80%93Laplace_theorem), the normal distribution can be used to approximate the binomial distribution for large enough $n$ and/or $p$ not close to 0 or 1, more specifically, if $np (1-p) > 5$ ([Dunning, 1993](#dunning-1993)). Under this precondition, we could use a one-sample t-test to determine whether the population mean (the bigram probability estimated from our text sample, $P(w_1, w_2) = \frac{C(w_1, w_2)}{N}$) is significantly different (or larger if we use a one-sided test) from the mean we would expect given the null hypothesis (the product of the probabilties of the individual words, $H_0: P(w_1, w_2) = P(w_1) P(w_2)$). However, [we know](https://proceed-to-decode.com/posts/vocabulary-of-russian-news/) that the distribution of words in natural language follows Zipf's law, that is, a few words occur with very high frequency, while many words occurs just once or a few times, even in a large corpus. So in many cases $p$ will be close to zero and $np (1-p) < 1$, prohibiting the use of the t-test, which assumes a normal distribution.

### Likelihood ratios

[Dunning (1993)](#dunning-1993) proposed likelihood ratios as more appropriate test for collocations. His method does not require normally distributed data and produces more interpretable numbers than the t-test statistic. The computed statistic is asymptotically $\chi ^2$ distributed, so we can consult the same table as we do for Pearson's chi-squared test to determine confidence levels. Furthermore, the statistic is more appropriate for sparse data than Pearson's $X^2$.


We start by defining the hypotheses we want to test. I copy the definitions from [Manning and Schütze's (1999)](#manning-schütze-1999).

<div>$$
H_0: P(w_2|w_1) = p = P(w_2|\neg w_1)\\
H_A: P(w_2|w_1) = p_1 \neq p_2 = P(w_2|\neg w_1)
$$</div>

Note that this does not follow directly from the definition above. The test is two-sided because the null hypothesis is an equality (independence of $w_1$ and $w_2$). If it is rejected, we could have $P(w_2|w_1) > P(w_2|\neg w_1)$ or $P(w_2|w_1) < P(w_2|\neg w_1)$. In the latter case, we wouldn't be dealing with a collocation according to our definition. We can solve this problem by additionally testing for the direction.

Now we compute the logarithm of the likelihood ratios, the ratio between the different likelihoods of obtaining the observed frequencies under the two hypotheses.

<div>$$\begin{align}
\log \lambda &= \log \frac{b(c_{12}, c_1, p) \cdot b(c_2 - c_{12}, N - c_1, p)}{b(c_{12}, c_1, p_1) \cdot b(c_2 - c_{12}, N - c_1, p_2)} \\
             &= \log L(c_{12}, c_1, p) + \log L(c_2 - c_{12}, N - c_1, p)\\ 
             &\quad - \log L(c_{12}, c_1, p_1) - \log L(c_2 - c_{12}, N - c_1, p_2)
\end{align}$$</div>

where $L(k; n, x) = x^k (1-x)^{n-k}$ because the binomial coefficients cancel out. Under the null hypothesis, we compute the likelihood of choosing $C(w_1, w_2) = c_{12}$ out of $C(w_1) = c_1$ items and $C(\neg w_1, w_2) = c_2 - c_{12}$ out of $C(\neg w_1) = N - c_1$ items with the same probability $p = \frac{c_2}{N}$. Under the alternative hypothesis, the two PMFs have different probability parameters: $p_1 = \frac{c_{12}}{c_1}$ and $p_2 = \frac{c_2 - c_{12}}{N - c_{1}}$. It's the quantity $- 2 \log \lambda$ that is asymptotically $\chi ^2$ distributed with one degree of freedom.


|                   | w<sub>2</sub> | ¬w<sub>2</sub> |      ∑     |
|------------------:|--------------:|---------------:|-----------:|
| **w<sub>1</sub>** | c<sub>12</sub> | *c<sub>1</sub> - c<sub>12</sub>* | c<sub>1</sub> |
| **¬w<sub>1</sub>** | c<sub>2</sub> - c<sub>12</sub> |  *N - c<sub>1</sub> - c<sub>2</sub> + c<sub>12</sub>* | N - c<sub>1</sub> |
| **∑** | c<sub>2</sub> | *N - c<sub>2</sub>* |  N |


## Application: Period disambiguation

Besides the obvious applications in lexicography (identifying proper nouns, extracting typical contexts of words) or phraseology (identifying [phrasemes](https://en.wikipedia.org/wiki/Phraseme)), an example of a more creative and very successful application of collocation identification has been proposed by Kiss and Strunk ([2002](#kiss-strunk-2002), [2006](#kiss-strunk-2006)). They apply different versions of the log-likelihood ratio method introduced above to disambiguate between periods as sentence boundaries and abbreviations. Bigram collocations of truncated word and period tend to be abbreviations (which, however, could appear at the end of a sentence), collocational ties between tokens surrounding a period are evidence against a sentence boundary, whereas collocations of a potential sentence boundary and its subsequent word are evidence for a sentence boundary (frequent sentence starters). Together with a few empirically determined heuristics, these collocation tests are the backbone of the unsupervised sentence boundary detection system *Punkt* which comes with Python's [Natural Language Toolkit (NLTK)](https://www.nltk.org/). Of course, Kiss and Strunk's (2006) system cannot compete with modern supervised systems that are trained on huge amounts of annotated (domain-specific) data, but often labeled data for our language and domain of interest is unavailable and in these cases *Punkt* is still a very good choice, given that what we consider *sentences* is delimited by periods (see [Sanchez (2019)](#sanchez-2019) for a recent counter example).



<hr>

<a name="1" href="#fn-1"><sup>1</sup></a> For the sake of simplicity, we assume the same number of unigrams and bigrams *N*.

<a name="2" href="#fn-2"><sup>2</sup></a> This is a naive assumption that obviously cannot be true. All models are wrong, but some are useful, as they say.

<hr>

<a name="dunning-1993"></a> Dunning, T. E. (1993). Accurate methods for the statistics of surprise and coincidence. *Computational linguistics*, 19(1), 61-74.

<a name="kiss-strunk-2002"></a>Kiss, T., & Strunk, J. (2002). Viewing sentence boundary detection as collocation identification. In *Proceedings of KONVENS* (Vol. 2002, pp. 75-82).

<a name="kiss-strunk-2006"></a>Kiss, T., & Strunk, J. (2006). Unsupervised multilingual sentence boundary detection. *Computational linguistics*, 32(4), 485-525.

<a name="manning-schütze-1999"></a>Manning, C., & Schütze, H. (1999). Foundations of statistical natural language processing. MIT press.

<a name="sanchez-2019"></a> Sanchez, G. (2019). Sentence boundary detection in legal text. In *Proceedings of the Natural Legal Language Processing Workshop 2019* (pp. 31-38).