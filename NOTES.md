# Notes

## Idea

Tool that lets you run through a defined checklist via CLI.

It shows all the items in the checklist and lets you tick them off.

It keeps track of what was ticked off the list, but we can reuse the checklist over and over.

Furthermore, it can be run as a git hook so if not everything is ticked of we fail.

Some Checklist items can be optional.

A checklist is compatible with Markdown, so we can use them in GitHub `pull_request_template.md` as well. Allowing us to use one file to define the PR's checklist and the checklist we run during git hook.

## File Specs

See [SPEC.md](./SPEC.md)

## To-do

- [x] Read in an MD file and find the checklist
- [x] Save a temporary hidden file with Saved Progress recorded
- [x] CLI option to not save progress
- [x] Implement Verbosity and proper logging
- [x] Implement UI to display the checklist
- [x] Parse Checklist SPEC
- [x] Parse Saved Progress SPEC
- [x] Generate Saved Progress SPEC
- [x] on exit we return a ExitCode = to number of unresolved items
 