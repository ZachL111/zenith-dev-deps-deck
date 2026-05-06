# Review Journal

The review surface for `zenith-dev-deps-deck` is deliberately narrow: one fixture, one scoring rule, and one local check.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 186, lane `ship`
- `stress`: `diagnostic quality`, score 168, lane `ship`
- `edge`: `review cost`, score 131, lane `watch`
- `recovery`: `safe rewrite`, score 223, lane `ship`
- `stale`: `change width`, score 177, lane `ship`

## Note

The useful failure mode here is a wrong decision on a named case, not a vague style disagreement.
