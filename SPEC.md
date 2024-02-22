# Spec

## Checklist SPEC

- Items can be nested
- list marker can be `- | +| * | [0-9].`
- Checkbox `[ ]` is not required but allowed
- Optional item is marked by `[OPTIONAL]`
- `<!-- checklist = 'name' -->` delimits the start and end of a checklist. End is optional
- `checklist = 'name'` has to be valid toml. And key must be `checklist`
- if no name is passed or its invalid name will be set to `checklist`
- Items must be unique !

```markdown
<!-- checklist = 'name' -->
- [ ] My checklist item
- [ ] My optional checklist item [OPTIONAL]
    - [ ] My nested checklist item
<!-- checklist = 'name' -->
```

## Saved Progress SPEC

- TOML file
- `text` is the text of the item
- `optional` is whether the item is optional or not
- `resolved` is whether we completed the item or not
- file is saved as `.<name>.kb.toml`
  - the `<name>` here is the name of the checklist defined by `<!-- checklist = 'name' -->`

```toml
name = '<name>'

[[items]]
text = 'test checklist item 1'
optional = false
resolved = false

[[items]]
text = 'test checklist item 2'
optional = false
resolved = false

```
