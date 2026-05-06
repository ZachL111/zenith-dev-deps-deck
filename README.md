# zenith-dev-deps-deck

`zenith-dev-deps-deck` keeps a focused Rust implementation around developer tools. The project goal is to build a Rust toolkit that studies deps behavior through capacity fixtures, with allocation and spill reports and explicit failure cases.

## Use Case

The point is to make a small domain rule concrete enough that a reader can change it and immediately see what broke.

## Zenith Dev Deps Deck Review Notes

`recovery` and `edge` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Highlights

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/zenith-dev-deps-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `safe rewrite` and `review cost`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Code Layout

The implementation keeps the scoring rule plain: reward signal and confidence, preserve slack, penalize drag, then classify the result into a review lane.

The Rust addition stays small enough to inspect in one sitting.

## Run The Check

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Regression Path

The check exercises the source code and the review fixture. `recovery` is the high score at 223; `edge` is the low score at 131.

## Future Work

The fixture set is small enough to audit by hand. The next useful expansion is malformed input coverage, not extra surface area.
