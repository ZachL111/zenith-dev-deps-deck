# Zenith Dev Deps Deck Walkthrough

This note is the quickest way to read the extra review model in `zenith-dev-deps-deck`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 186 | ship |
| stress | diagnostic quality | 168 | ship |
| edge | review cost | 131 | watch |
| recovery | safe rewrite | 223 | ship |
| stale | change width | 177 | ship |

Start with `recovery` and `edge`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`recovery` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
