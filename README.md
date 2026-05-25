# skills

Personal Codex skills.

## Cài Đặt Nhanh

Từ target project directory, chạy:

```sh
curl -fsSL "https://raw.githubusercontent.com/haketienloc10/skills/main/scripts/install-skills.sh?$(date +%s)" | bash
```

Hoặc cài vào một path cụ thể:

```sh
curl -fsSL "https://raw.githubusercontent.com/haketienloc10/skills/main/scripts/install-skills.sh?$(date +%s)" | bash -s -- --directory /path/to/project
```

Installer sẽ hỏi chọn skill:

- `space`: select/deselect
- `enter`: install
- `up/down`: move

Skills được cài vào:

```text
<project>/.agents/skills
```

## Requirements

- `curl`
- `tar`
- `bash`
- `cargo`, chỉ cần khi cài `project-notes`

Khi chọn `project-notes`, installer cũng tạo:

```text
<project>/bin/pnotes
<project>/.project-notes/notes/
```

Verify:

```sh
cd /path/to/project
./bin/pnotes guide
```
