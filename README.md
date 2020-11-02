# Tinysearch for languages without word boundaries

Tinysearch doesn't support languages without space delimiters, e.g. Japanese.
In this repo, I use `icu::BreakIterator` as a tokenizer and make tinysearch compatible with those languages.

Nightly rustc is required to compile:

```
$ cd fixtures 
$ cargo +nightly run index.json
$ python -m http.server
```
