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

- [] Read in an MD file and find the checklist
- [] Save a temporary hidden file with progress recorded
- [] CLI option to not save progress
- [] Implement Verbosity and proper logging