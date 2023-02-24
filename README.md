dictionarium
============

A speedy, offline, command-line dictionary based on Wiktionary.

Inspired by the excellent [thesauromatic](https://github.com/cjrh/thesauromatic).

Usage
-----

```bash
[apropos@arch ~]$ dictionarium dictionarium
Latin
Etymology
Renaissance Latin, from noun of action dictiō ("speaking") + ārium, from dīcō ("say, speak"). First attested in 1481.
Noun
dictiōnārium -1. dictionary
Usage notes
• Used especially in book titles, normally with adjective like Dictionarium Latino Lusitanicum ("Latin-Portuguese Dictionary"), Dictionarium Latinogermanicum/Latino-Germanicum ("Latin-German Dictionary")
```

Building
--------
The environment variables `index_path` and `dictionary_path` must be set to the locations of a complete multistream bz2 index and archive from the [Wiktionary dumps](https://dumps.wikimedia.org/enwiktionary/), not included here as they're several gigabytes.
They are by default set to the last dumps from 2022 ([2022-12-20](https://dumps.wikimedia.org/enwiktionary/20221220/)), located in a top-level `data/` folder.
