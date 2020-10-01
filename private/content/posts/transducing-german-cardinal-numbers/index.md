---
title: "Transducing German Cardinal Numbers"
date: 2020-09-06T15:25:42+02:00
description: "Converting spelled-out German cardinal number to numerals and vice versa. Exercise in writing finite-state transducers."
features:
    - Highlight
---

This is a little excercise in writing finite-state transducers. The goal is to convert spelled-out German cardinal numbers from 1 to 9999 to numerals and vice versa. I will construct the transducer by going from written-out numbers to numerals, e.g., from *zweiunddreißig* to *32*. The implementation is done with the finite-state toolkit [*foma*](https://fomafst.github.io/).

## Intermediate representation

In a first step, we convert each written-out number into an intermediate representation by transducing compound parts to numerals in linear order, for example, *neuntausend-zweihundert-acht-und-vierzig* (hyphens added for better readability) should become `9000+200+8&40`. Let's start with two-digit numbers.

```plaintext
TwoToNine = zwei:2  | drei:3   | vier:4  | fünf:5  |
            sechs:6 | sieben:7 | acht:8  | neun:9  ;

Teens = zehn:     [1 %0]   | elf:      [1 1]    | 
        zwölf:    [1 2]    | dreizehn: [1 3]    |
        vierzehn: [1 4]    | fünfzehn: [1 5]    |
        sechzehn: [1 6]    | siebzehn: [1 7]    |
        achtzehn: [1 8]    | neunzehn: [1 9]    ;

TwentyToNinety = zwanzig:[2 %0]  | dreißig:[3 %0]  |
                 vierzig:[4 %0]  | fünfzig:[5 %0]  |
                 sechzig:[6 %0]  | siebzig:[7 %0]  |
                 achtzig:[8 %0]  | neunzig:[9 %0]  ;

TwoDigitNumber = eins:1 | TwoToNine | Teens | TwentyToNinety |
                 [ein:1 | TwoToNine] und:%& TwentyToNinety;
```

The digit *1* is special, since it sometimes appears as *ein* and sometimes *eins*, that's why it's not part of the first transducer.  Some `Teens` could be decomposed further, but this would only complicate things. Because *0* has special semantics in *foma* regex, we need to make sure to escape *0* literals with the percentage symbol. The ampersand that replaces *und* indicates that the numbers around it need to be swapped later.

For larger numbers, we need hundreds and thousands, so we define two more transducers.
```plaintext
Hundred  = [ein:1 | TwoToNine] hundert:[%0 %0 %+];
Thousand = [ein:1 | TwoToNine] tausend:[%0 %0 %0 %+];
```

Now we can put everything together. For the sake of simplicity, we ignore some variants: we recognize *einhundert* and *eintausend*, but not *hundert* and *tausend*; we recognize *einhundertfünf*, but not *einhundertundfünf*. These variants can be added easily, but the expressions become a bit messier.

```plaintext
Intermediate = (Thousand) (Hundred) (TwoDigitNumber);
```

If we check the `lower-words`<sup id="fn-1">[1](#1)</sup> of this transducer, we see the following:

```plaintext
foma[1]: lower-words

1000+
1000+100+
1000+100+1&20
...
```
Now we need to transform these intermediate forms into the desired output.

## Cascaded transducers

We'll move from right to left. In the end there may be a trailing plus (introduced by `Hundred` or `Thousand`). We don't need it, so we add a cleanup rule.

```plaintext
CleanPlus = %+ -> 0 || _ .#.;
```

Then we remove any trailing zero after an ampersand and swap the digits around the ampersand.

```plaintext
Trunc = %0 -> 0 || %& ? _ .#.;

def Ins(X) [..] -> X || X %& ? _ .#.;
Copy = Ins(1) .o. Ins(2) .o. Ins(3) .o. 
       Ins(4) .o. Ins(5) .o. Ins(6) .o.
       Ins(7) .o. Ins(8) .o. Ins(9) ;

Delete = ? %& -> 0 ;
Swap   = Trunc .o. Copy .o. Delete;
```

Maybe there is some more elegant syntax to do this, but I define a helper template that inserts a digit in the context of itself, the ampersand and some other digit. By applying this template for each non-zero digit, we copy the digit before the ampersand to the end of the string. Finally we delete the original digit and the ampersand, completing the swap.

Now we can move further to the left and add our two digit number to the hundreds.

```plaintext
N = [%0|1|2|3|4|5|6|7|8|9];

FixHundred = %0 %+     -> 0 || %0 _ N .#. ,,
             %0 %0 %+  -> 0 || _ N N .#.  ;
```

We simply delete as many zeros as there are digits after the plus symbol. The same is repeated with the thousands to the left.

```plaintext
FixThousand = %0 %+       -> 0  || %0 %0 _ N .#. ,,
              %0 %0 %+    -> 0  || %0 _ N N .#.  ,,
              %0 %0 %0 %+ -> 0  || _ N N N .#.    ;
```

We compose the pieces to obtain our transducer for cardinal numbers.

```plaintext
Card = Intermediate .o. CleanPlus .o. Swap .o. 
       FixHundred .o. FixThousand;
```

## Result

The final transducer has 10,000 paths: 9,999 numbers and the empty string. FSTs are closed under inversion (input and output labels are simply switched), so we can apply our cardinal number FST in both directions, as generator and parser.

```plaintext
foma[0]: regex Card;
4.8 kB. 37 states, 212 arcs, 10000 paths.

foma[1]: up
apply up> 2345
zweitausenddreihundertfünfundvierzig

foma[1]: down
apply down> fünftausenddreiundzwanzig
5023
```

<hr>

<a id="1" href="#fn-1"><sup>1</sup></a> Words of the *lower* or *second* projection of `Intermediate`, that is, words *o* for which an input *i* exists such that `Intermediate` maps *i* to *o*.