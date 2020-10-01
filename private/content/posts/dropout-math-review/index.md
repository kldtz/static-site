---
title: "The Dropout Learning Algorithm: Math Review"
date: 2020-03-14T09:20:45+01:00
description: "Stochastic functions and some combinatorics for Baldi and Sadowski (2014)."
features: 
       - MathJax
---

Review of combinatorics and probability theory for Baldi & Sadowski (2014).


## Combinatorics

We have a set $I$ of $n$ elements. The set of all subsets of $I$, the powerset $\mathcal{P}(I)$, has $2^n$ elements (including the empty subset), which can be seen 

1. **algebraically**, by grouping all subsets of $I$ by cardinality, and noting that for $0 \leq k \leq n$ we have ${n \choose k}$ subsets of cardinality $k$. The sum of these terms is the left side of the binomial theorem $\sum_{k=0}^n {n \choose k} x^{n-k} y^k = (x + y)^n$ with $x = y = 1$, so $\mathcal{P}(I) = 2^n$.
2. **combinatorially**, simply by observing that we can form a subset of $I$ by deciding for each element, whether to include it in the set or not. Thus, there are $2^n$ ways of forming a subset.

How often does each element of $I$ appear in all of the $2^n$ subsets of $I$? Again, consider each group of subsets with equal cardinality: We can form all subsets of $I$ of cardinality $k > 0$ that contain the same element $e$ by choosing element $e$ out of $n$ elements and combining it with each subset of carndinality $k-1$ of the remaining $n-1$ elements. Thus, each element of $I$ appears $\sum_{k=1}^{n} {n-1 \choose k-1} = 2^{n-1}$ times.


## Stochastic functions

### Expectation

For a discrete random variable $X$ with $k$ possible outcomes $x_1, x_2, \ldots, x_k$ occuring with probabilities $P(X = x_i) = p_i$, $1 \leq i \leq k$, the expectation (**first moment**) of $X$ is defined as

<div>$$
E[X] = \sum_{i=1}^k x_i p_i
$$</div>

So, in the special case of a Bernoulli random variable with $x_1 = 1$, $x_2 = 0$ and $p_1 = p$, $p_2 = 1-p = q$, we have $E[X] = p$. 

From the general definition, we can prove the **linearity of expectation**: (1) The expected value of the sum of two (or any finite number of) random variables equals the sum of the expected values of the individual random variables.

<div class="math-left-align">$$\begin{align}
E[X + Y] &= \sum_x \sum_y \left[ (x + y) \cdot P(X = x, Y = y)\right] \\
         &= \sum_x \sum_y \left[ x \cdot P(X = x, Y = y)\right] + \sum_x \sum_y \left[ y \cdot P(X = x, Y = y)\right] \\
         &= \sum_x x \sum_y P(X = x, Y = y) + \sum_y y \sum_{x} P(X = x, Y = y) \\
         &= \sum_x x \cdot P(X = x) + \sum_y y \cdot P(Y = y) \\
         &= E[X] + E[Y]
\end{align}$$</div>


(2) The expected value scales linearly with a multiplicative constant.


<div class="math-left-align">$$\begin{align}
E[aX] &= \sum_x a x \cdot P(X = x) \\
      &= a \sum_x x \cdot P(X = x) \\
      &= a \cdot E[X] \\
\end{align}$$</div>


By definition, two discrete random variables are independent iff $P(X=x, Y=y) = P(X=x) P(Y=y)$ for all $x, y$. Thus for **independent random variables**, the expectation of the product equals the product of the expectations.

<div class="math-left-align">$$\begin{align}
E[XY] &= \sum_x \sum_y xy \cdot P(X=x, Y=y) \\
      &= \sum_x x P(X=x) \sum_y y P(Y=y) \\
      &= E[X] E[Y] \\
\end{align}$$</div>


### Variance

Variance (the **second central moment**) is defined as the expected squared deviation of a random variable $X$ from its mean $\mu = E[X]$.

<div>$$\begin{align}
Var(X) &= E \left[ (X - \mu)^2 \right] \\
       &= E \left[ X^2 - 2 X E[X] + E[X]^2 \right] \\ 
       &= E[X^2] - 2 \cdot E[X]^2 + E[X]^2 \\ 
       &= E[X^2] - E[X]^2
\end{align}$$</div>

For the special case of a **Bernoulli random variable** we can simplify as follows:

<div>$$\begin{align}
Var(X) &= E[X^2] - E[X]^2 \\ 
       &= p \cdot 1^2 + q \cdot 0^2 - p^2 \\ 
       &= p - p^2 \\ 
       &= p(1 - p) = pq
\end{align}$$</div>

If its argument is scaled by a constant, variance is **scaled by the square** of that constant. This follows from the definition of variance and linearity of expectation:

<div>$$\begin{align}
Var(aX) &= E \left[ (a X - a \mu)^2 \right] \\ 
       &= E \left[ a^2 (X - \mu)^2 \right] \\ 
       &= a^2 E \left[ (X - \mu)^2 \right] \\ 
       &= a^2 Var(X)
\end{align}$$</div>


### Covariance

Covariance is defined as the expected value of the product of the squared deviations of two jointly distributed random variables from their means.

<div>$$\begin{align}
Cov(X, Y) &= E\left[ (X - E[X]) (Y - E[Y])  \right] \\ 
          &= E\left[ XY - X E[Y] - E[X]Y + E[X]E[Y]  \right] \\ 
          &= E[XY] - E[X]E[Y] - E[X]E[Y] + E[X]E[Y] \\ 
          &= E[XY] - E[X]E[Y]
\end{align}$$</div>

Variance is a special case of covariance, where $X = Y$. If $X$ and $Y$ are independent random variables, then $E[XY] = E[X]E[Y]$, that is, $Cov(X, Y) = 0$ (this implication does not hold in the opposite direction in general).

The **variance of the sum** of two random variables equals the sum of their individual variances plus two times their covariance.

<div>$$\begin{align}
Var(X + Y) &= E \left[ ((X+Y) - E[X + Y])^2 \right] \\ 
           &= E \left[ ((X+Y)^2 - 2 (X+Y) E[X + Y] + E[X+Y]^2 \right] \\ 
           &= E [ (X^2 + 2XY + Y^2 \\ 
           &\qquad - 2 (X E[X] + X E[Y] + Y E[X] + Y E[Y]) \\ 
           &\qquad + E[X]^2 + 2 E[X] E[Y] + E[Y]^2 ] \\ 
           &= E[ X^2 - 2 X E[X] + E[X]^2 \\ 
           &\qquad + Y^2 - 2 Y E[Y] + E[Y]^2 \\ 
           &\qquad + 2 ( XY - X E[Y] - Y E[X] + E[X] E[Y])] \\ 
           &= E[ (X - E[X])^2 ] + E[ ( Y - E[Y])^2 ] \\  
           &\qquad + 2 \cdot E [ (X - E[X] ) ( Y - E[Y] )] \\ 
           &= Var(X) + Var(Y) + 2 \cdot Cov(X, Y)
\end{align}$$</div>

Because $Cov(X, Y) = 0$ for independent random variables, variance is linear if $X$ and $Y$ are independent.


---

Baldi, P., & Sadowski, P. (2014). The dropout learning algorithm. Artificial intelligence, 210, 78-122.
