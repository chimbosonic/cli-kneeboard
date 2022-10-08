## Checklist SPEC

- Items can be nested
- list marker can be `- | +| * | [0-9].`
- Checkbox `[ ]` is not required but allowed
- Optional item is marked by `[OPTIONAL]`
- `<!-- checklist = 'name' -->` delimits the start and end of a checklist. End is optional
- `checklist = 'name'` has to be valid toml. And key must be `checklist` 

```markdown
<!-- checklist = 'name' -->
- [ ] My checklist item
- [ ] My optional checklist item [OPTIONAL]
    - [ ] My nested checklist item
<!-- checklist = 'name' -->
```

## Saved Progress SPEC

- TOML file
- One table called `[checklist]`
- Key value pairs where value is a `bool` and key is a `string`
- Key is the text of the checklist item
- Value is whether we completed the item or not
- file is saved as `.<name>.kb.toml`
  - the `<name>` here is the name of the checklist defined by `<!-- checklist = 'name' -->`

```toml
[checklist]
"My uncompleted checklist item" = false
"My completed checklist item" = true
```