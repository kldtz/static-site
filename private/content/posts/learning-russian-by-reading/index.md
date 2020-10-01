---
title: "Learning Russian by Reading"
date: 2018-10-03T10:07:52+02:00
description: "Introduction of Chrome extension 'Read Russian' for translating and learning Russian vocabulary."
---

I like picking up words and grammatical skills while reading texts in a new language. Since I'm learning Russian at the moment, I developed a little Chrome extension that supports this style of learning for the Russian language. The extension is available at the [Chrome Web Store](https://chrome.google.com/webstore/detail/read-russian/bfiedbggkbnefgngmdkhofddgpfjahcd).

## Motivation

Reading a text in a new foreign language usually involves looking up a lot of words in a dictionary. One of the most comprehensive and free online dictionaries is the [Wiktionary](https://www.wiktionary.org/). While we don't need to search heavy physical dictionaries anymore, to look up a word on Wiktionary we still need to open another Browser tab, type in or copy the unknown word, possibly follow redirections and skim through a long page with information about etymology, pronunciation, word senses, inflection tables, lists of lexically related words etc. for several languages (at least the Slavic languages have a large shared vocabulary). By the time I find the information I'm looking for, I usually forgot the sentence that I was trying to understand by looking up the unknown word. I'm forced to reread the sentence and maybe the preceding sentences. If my vocabulary is very restricted and I need to look up several words in a sentence, this is getting tiresome.

Once I read and understood a text, I would like to retain at least some share of the new vocabulary. One popular method to this end are flashcards -- today often in the form of spaced repetition software. But again, I'm a lazy learner and writing flashcards isn't fun.

## Features

The extension displays a short summary of word information aggregated from one or more Wiktionary pages at the top of the current tab and automatically generates flashcards that can be exported in CSV format and imported into Anki or another spaced repetition system.

The displayed information consists of the stress pattern (important for correct pronunciation), a list of translations (so far no disambiguation is performed) and some linguistic information (part of speech, morpho&shy;syn&shy;tactic features like number, gender, person, case, tense, aspect, as well as the aspect partner for verbs) if available.

The generated flashcards have three fields: The front is simply the lemma, the back has the pronunciation and a list of translations. A third field contains linguistic features and the word form surrounded by its immediate context in the source text.

## Usage

Select a Russian word in a text, open the context menu (right-click the selection) and click on the '*Get info for ...*' item provided by the extension. A small info box will appear at the top of the screen:

<figure class="middle">
    <img src="/img/read-russian-screenshot.png"
         alt="Extension screenshot"/> <figcaption>
            <p>Read Russian screenshot for понимал</p>
        </figcaption>
</figure>

In this example the info box displays the selected word and the lemma with stress marks, morpho&shy;syn&shy;tactic features (masculine, past indicative, imperfective), aspect partner (the perfective lemma *понять*) and a list of translations.

To generate a flashcard, click on a translation in the info box. A counter at the blue toolbar icon in the upper-right corner tracks the number of saved flashcards. These flashcards can be exported in CSV format via the extension's popup window and imported into Anki via *Files > Import...* (make sure that the comma is used as field separator and *Allow HTML in fields* is checked).


## Implementation

The extension uses the MediaWiki API to send a search request for the word selected by the user. From the list of pages in the response, the best match is determined heuristically. The best matching page itself and pages linked from the first page (e.g. via *inflection-of* templates) are processed. There are two alternatives for processing WikiMedia content: Parsing the HTML that is displayed to the readers of Wiktionary articles or parsing Wikitext, the markup language used by authors of WikiMedia content. Some pros and cons are collected on this [Wiktionary project page](https://en.wiktionary.org/wiki/Wiktionary:Parsing). For me there are two main points:

1. Wikitext contains complex [templates](https://www.mediawiki.org/wiki/Help:Templates) that are notoriously difficult to parse. There is no predefined, finite set of templates, people make them up as they go, not all of them are documented, Wiktionary authors use them a lot.
2. The HTML is generated from Wikitext. Since I'm not running my own server with Wiktionary content, I have no control over this generation. Any change as trivial as a different class name may break my extension.

I went for the Wikitext option, as I realized that English Wiktionary authors of articles on Russian words have a preferred set of templates that I can cover without too much effort and I figured that skipping some obscure templates is better than running the risk of a complete loss of functionality after minor WikiMedia updates. Since I didn't want to pull any WikiMedia template parser code into the project, I implemented my own shallow parser, following a 'good enough' approach. It deals with a fixed set of frequently used templates and supports nested templates. To keep things simple (both for me as a developer and for the user), I only extract a subset of the available information. The code is available on [GitHub](https://github.com/kldtz/read-russian).

