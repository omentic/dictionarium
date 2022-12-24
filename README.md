dictionarium
============

A speedy, offline, command-line dictionary based on Wiktionary.

Inspired by the excellent [thesauromatic](https://github.com/cjrh/thesauromatic).

Building
--------
The environment variables `index_path` and `dictionary_path` must be set to the locations of a complete multistream bz2 index and archive from the [Wiktionary dumps](https://dumps.wikimedia.org/enwiktionary/), not included here as they're several gigabytes.
They are by default set to the last dumps from 2022 ([2022-12-20](https://dumps.wikimedia.org/enwiktionary/20221220/)), located in a top-level `data/` folder.
