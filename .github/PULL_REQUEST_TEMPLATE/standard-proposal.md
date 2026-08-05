## What changes

<!-- The suite, schema field, or resolver/precedence behavior this changes. -->

## Why it's universal enough for `recommended@1`

<!--
Godharness ships a small, curated default suite so its context stays useful rather than
generic noise (see README "Recommended suite"). State why this belongs in the universal
default rather than being left to repository-specific standards.
-->

## Compatibility

<!--
Does an existing godharness.yaml or standard document stop parsing, change meaning, or
change precedence because of this? Say so plainly, including "no".
-->

## Validation

<!-- The checks you ran and their results. -->

## Checklist

- [ ] The `context` JSON contract is unchanged, or the change is called out as breaking.
- [ ] Existing `godharness.yaml` examples in documentation still parse.
- [ ] Documentation describes what exists today; planned work is labeled as planned.
