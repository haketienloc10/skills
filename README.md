# skills

Personal Codex skills.

## Install skills

Run the interactive installer:

```sh
./install.sh
```

Use `space` to select skills and `enter` to install them into:

```text
<project>/.agents/skills
```

By default `<project>` is the current directory. You can pass a target project path:

```sh
./install.sh /path/to/target-project
```

When `project-notes` is selected, the installer also runs its bootstrap script for the target project. This creates:

- `<project>/bin/pnotes`
- `<project>/.project-notes/notes/`

## project-notes

Install `project-notes` into a target repo from this checkout:

```sh
sh project-notes/scripts/install.sh /path/to/target-repo
```

This creates:

- `/path/to/target-repo/bin/pnotes`
- `/path/to/target-repo/.project-notes/notes/`

Verify:

```sh
cd /path/to/target-repo
./bin/pnotes guide
```
