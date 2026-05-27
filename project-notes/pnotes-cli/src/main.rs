use chrono::Local;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

const NOTES_DIR: &str = ".project-notes/notes";
const LOG_PATH: &str = "logs/pnotes-debug.jsonl";

// Append one JSON event line to LOG_PATH. Silently ignores all errors so the
// calling command always succeeds regardless of log I/O issues.
fn write_debug_event(event: serde_json::Value) {
    let log_path = PathBuf::from(LOG_PATH);
    if let Some(parent) = log_path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }
    if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(&log_path) {
        let mut line = serde_json::to_string(&event).unwrap_or_default();
        line.push('\n');
        let _ = file.write_all(line.as_bytes());
    }
}

// ── CLI ──────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "pnotes", about = "Project-level execution notes manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize .project-notes/notes/ directory
    Init,

    /// Add a new note
    Add {
        #[command(subcommand)]
        note_type: NoteType,
    },

    /// Recall relevant notes
    Recall {
        #[arg(long)]
        area: Vec<String>,
        #[arg(long)]
        tag: Vec<String>,
        #[arg(long)]
        task: Option<String>,
        #[arg(long, default_value = "3")]
        limit: usize,
    },

    /// Show full content of a note by ID
    Show { id: String },

    /// Print workflow guide and command reference
    Guide,

    /// Generate a Change Safety Brief for an area
    Brief {
        #[arg(long)]
        area: Vec<String>,
        #[arg(long)]
        tag: Vec<String>,
        #[arg(long)]
        task: Option<String>,
        #[arg(long, default_value = "3")]
        limit: usize,
    },

    /// Quality review commands
    Quality {
        #[command(subcommand)]
        sub: QualitySubcommands,
    },
}

#[derive(Subcommand)]
enum QualitySubcommands {
    /// Check pending notes and if a quality review is required
    Status,
    /// Record a completed quality review result
    Record {
        /// Path to the JSON file containing the review result
        #[arg(long)]
        from: PathBuf,
    },
}

#[derive(Subcommand)]
enum NoteType {
    /// Add a continuity note
    Continuity {
        #[arg(long)]
        task: Option<String>,
        #[arg(long)]
        signal: Option<String>,
        #[arg(long)]
        area: Vec<String>,
        #[arg(long)]
        tag: Vec<String>,
        #[arg(long)]
        handoff: Option<String>,
        #[arg(long)]
        run: Option<String>,
        #[arg(long)]
        decision: Vec<String>,
        #[arg(long)]
        invariant: Vec<String>,
        #[arg(long)]
        risk: Vec<String>,
        #[arg(long)]
        test_command: Vec<String>,
        #[arg(long)]
        test_covers: Vec<String>,
        #[arg(long)]
        missing_test: Vec<String>,
    },
}

// ── Note schema ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum TestEntryValue {
    Structured {
        command: String,
        #[serde(default)]
        covers: Vec<String>,
    },
    Legacy(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct TestEntry {
    command: String,
    covers: Vec<String>,
}

impl<'de> Deserialize<'de> for TestEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match TestEntryValue::deserialize(deserializer)? {
            TestEntryValue::Structured { command, covers } => Ok(TestEntry { command, covers }),
            TestEntryValue::Legacy(command) => Ok(TestEntry {
                command,
                covers: vec![],
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NoteFrontmatter {
    id: String,
    #[serde(rename = "type")]
    note_type: String,
    task: String,
    created_at: String,
    signal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handoff: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    areas: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    read_when: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    supersedes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    decisions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    invariants: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    risks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tests: Vec<TestEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    missing_tests: Vec<String>,
}

// ── Frontmatter parser ───────────────────────────────────────────────────────

/// Extract YAML block between first pair of `---` delimiters.
fn extract_frontmatter(content: &str) -> Option<&str> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return None;
    }
    let after_first = &content[3..];
    // skip optional newline right after opening ---
    let after_first = after_first.trim_start_matches('\n').trim_start_matches('\r');
    let end = after_first.find("\n---")?;
    Some(&after_first[..end])
}

fn parse_frontmatter(content: &str) -> Result<NoteFrontmatter, String> {
    let yaml = extract_frontmatter(content).ok_or("No frontmatter found")?;
    serde_yaml::from_str(yaml).map_err(|e| format!("YAML parse error: {e}"))
}

// ── Path normalization ───────────────────────────────────────────────────────

fn normalize_path(p: &str) -> String {
    p.trim_end_matches('/').to_lowercase()
}

fn area_exact(note_area: &str, query: &str) -> bool {
    normalize_path(note_area) == normalize_path(query)
}

fn area_prefix(note_area: &str, query: &str) -> bool {
    let a = normalize_path(note_area);
    let b = normalize_path(query);
    a.starts_with(&b) || b.starts_with(&a)
}

// ── Scoring ──────────────────────────────────────────────────────────────────

struct RecallFilter {
    areas: Vec<String>,
    tags: Vec<String>,
    task: Option<String>,
}

fn score_note(fm: &NoteFrontmatter, filter: &RecallFilter) -> i32 {
    // Return -1 if no filters at all (handled outside)
    let has_filter = !filter.areas.is_empty() || !filter.tags.is_empty() || filter.task.is_some();
    if !has_filter {
        return 0; // will be handled as recency-only
    }

    let mut score = 0i32;

    // Area scoring: +5 exact, +4 prefix (per query area, take max match per note area)
    for q_area in &filter.areas {
        let mut area_score = 0i32;
        for n_area in &fm.areas {
            if area_exact(n_area, q_area) {
                area_score = area_score.max(5);
            } else if area_prefix(n_area, q_area) {
                area_score = area_score.max(4);
            }
        }
        score += area_score;
    }

    // Task match: +3
    if let Some(ref t) = filter.task {
        if fm.task == *t {
            score += 3;
        }
    }

    // Tag match: +2 each
    for q_tag in &filter.tags {
        if fm.tags.iter().any(|nt| nt == q_tag) {
            score += 2;
        }
    }

    // Text match in signal or read_when: +2
    let all_queries: Vec<&str> = filter
        .areas
        .iter()
        .map(|s| s.as_str())
        .chain(filter.tags.iter().map(|s| s.as_str()))
        .chain(filter.task.as_deref())
        .collect();

    for q in &all_queries {
        let q_lower = q.to_lowercase();
        let signal_lower = fm.signal.to_lowercase();
        let rw_text: String = fm
            .read_when
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");
        if signal_lower.contains(&q_lower) || rw_text.contains(&q_lower) {
            score += 2;
            break; // +2 once per note
        }
    }

    score
}

fn score_and_filter_notes(
    notes: Vec<(PathBuf, NoteFrontmatter)>,
    filter: &RecallFilter,
) -> Vec<(i32, String, PathBuf, NoteFrontmatter)> {
    let has_filter = !filter.areas.is_empty() || !filter.tags.is_empty() || filter.task.is_some();

    // Map and filter notes matching the query/filter
    let mut scored: Vec<(i32, String, PathBuf, NoteFrontmatter)> = notes
        .into_iter()
        .map(|(path, fm)| {
            let s = if has_filter {
                score_note(&fm, filter)
            } else {
                0 // recency only
            };
            let date = fm.created_at.clone();
            (s, date, path, fm)
        })
        .filter(|(score, _, _, _)| !has_filter || *score > 0)
        .collect();

    if scored.is_empty() {
        return vec![];
    }

    // Sort: score DESC, then created_at DESC
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

    // Recency boost: newest gets +1 (applied after initial sort to break ties)
    // Re-sort with recency boost included
    let max_date = scored.iter().map(|(_, d, _, _)| d.clone()).max().unwrap_or_default();
    let mut scored: Vec<(i32, String, PathBuf, NoteFrontmatter)> = scored
        .into_iter()
        .map(|(s, d, p, fm)| {
            let boost = if d == max_date { 1 } else { 0 };
            (s + boost, d, p, fm)
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

    scored
}

// ── File helpers ─────────────────────────────────────────────────────────────

fn notes_dir() -> PathBuf {
    std::env::var("PNOTES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(NOTES_DIR))
}

fn resolve_note_path(task_slug: &str, date: &str) -> PathBuf {
    let base = notes_dir();
    let mut path = base.join(format!("{date}-{task_slug}.md"));
    if !path.exists() {
        return path;
    }
    let mut suffix = 2;
    loop {
        path = base.join(format!("{date}-{task_slug}-{suffix}.md"));
        if !path.exists() {
            return path;
        }
        suffix += 1;
    }
}

fn parse_test_metadata_from_args<I, S>(args: I) -> Result<(Vec<TestEntry>, Vec<String>), String>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    let args_str: Vec<String> = args.iter().map(|s| s.to_string_lossy().into_owned()).collect();
    let is_add_continuity = args_str
        .windows(2)
        .any(|w| w[0] == "add" && w[1] == "continuity");
    if !is_add_continuity {
        return Ok((vec![], vec![]));
    }

    let mut tests: Vec<TestEntry> = vec![];
    let mut missing_tests: Vec<String> = vec![];
    let mut i = 0;
    while i < args_str.len() {
        let arg = &args_str[i];
        if arg == "--test-command" {
            let value = args_str
                .get(i + 1)
                .ok_or("--test-command requires a value")?
                .clone();
            tests.push(TestEntry {
                command: value,
                covers: vec![],
            });
            i += 2;
        } else if arg.starts_with("--test-command=") {
            let value = arg["--test-command=".len()..].to_string();
            if value.is_empty() {
                return Err("--test-command requires a value".to_string());
            }
            tests.push(TestEntry {
                command: value,
                covers: vec![],
            });
            i += 1;
        } else if arg == "--test-covers" {
            let value = args_str
                .get(i + 1)
                .ok_or("--test-covers requires a value")?
                .clone();
            let latest = tests
                .last_mut()
                .ok_or("--test-covers requires a preceding --test-command")?;
            latest.covers.push(value);
            i += 2;
        } else if arg.starts_with("--test-covers=") {
            let value = arg["--test-covers=".len()..].to_string();
            if value.is_empty() {
                return Err("--test-covers requires a value".to_string());
            }
            let latest = tests
                .last_mut()
                .ok_or("--test-covers requires a preceding --test-command")?;
            latest.covers.push(value);
            i += 1;
        } else if arg == "--missing-test" {
            let value = args_str
                .get(i + 1)
                .ok_or("--missing-test requires a value")?
                .clone();
            missing_tests.push(value);
            i += 2;
        } else if arg.starts_with("--missing-test=") {
            let value = arg["--missing-test=".len()..].to_string();
            if value.is_empty() {
                return Err("--missing-test requires a value".to_string());
            }
            missing_tests.push(value);
            i += 1;
        } else {
            i += 1;
        }
    }

    Ok((tests, missing_tests))
}

// ── Commands ─────────────────────────────────────────────────────────────────

fn cmd_init() {
    let dir = notes_dir();
    if dir.exists() {
        println!("Already initialized");
    } else {
        fs::create_dir_all(&dir).expect("Failed to create .project-notes/notes/");
        println!("Initialized .project-notes/");
    }
}

fn cmd_add_continuity(
    task: Option<String>,
    signal: Option<String>,
    areas: Vec<String>,
    tags: Vec<String>,
    handoff: Option<String>,
    run: Option<String>,
    decisions: Vec<String>,
    invariants: Vec<String>,
    risks: Vec<String>,
    tests: Vec<TestEntry>,
    missing_tests: Vec<String>,
) {
    // Validate required fields
    match (&task, &signal) {
        (None, _) | (_, None) => {
            eprintln!("Error: --task and --signal are required");
            std::process::exit(1);
        }
        _ => {}
    }
    let task = task.unwrap();
    let signal = signal.unwrap();

    let dir = notes_dir();
    if !dir.exists() {
        eprintln!("Run 'pnotes init' first");
        std::process::exit(1);
    }

    let date = Local::now().format("%Y-%m-%d").to_string();
    let file_path = resolve_note_path(&task, &date);
    let id = file_path
        .file_stem()
        .expect("resolved note path should have file stem")
        .to_string_lossy()
        .to_string();

    let has_area = !areas.is_empty();
    let has_tag = !tags.is_empty();
    let has_decision = !decisions.is_empty();

    let fm = NoteFrontmatter {
        id: id.clone(),
        note_type: "continuity".to_string(),
        task: task.clone(),
        created_at: date.clone(),
        signal: signal.clone(),
        run,
        handoff,
        areas,
        tags,
        read_when: vec![],
        supersedes: vec![],
        decisions,
        invariants,
        risks,
        tests,
        missing_tests,
    };

    let yaml = serde_yaml::to_string(&fm).expect("Failed to serialize frontmatter");
    // serde_yaml adds trailing newline already
    let body = format!(
        "---\n{yaml}---\n\n## task\n{task} — {date}\n\n## deviations\nNone\n\n## traps\nNone\n\n## dead_ends\nNone\n\n## validation_delta\nAs expected\n\n## next_agent_hint\nSee Handoff\n"
    );

    fs::write(&file_path, body).expect("Failed to write note file");
    println!("Created: {}", file_path.display());

    write_debug_event(serde_json::json!({
        "ts": Local::now().to_rfc3339(),
        "cmd": "add",
        "task_slug": task,
        "has_area": has_area,
        "has_tag": has_tag,
        "has_decision": has_decision
    }));
}

fn load_all_notes() -> Vec<(PathBuf, NoteFrontmatter)> {
    let dir = notes_dir();
    if !dir.exists() {
        return vec![];
    }

    let mut notes = vec![];
    for entry in WalkDir::new(&dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_path_buf();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: could not read {}: {e}", path.display());
                continue;
            }
        };
        match parse_frontmatter(&content) {
            Ok(fm) => notes.push((path, fm)),
            Err(e) => {
                eprintln!("Warning: skipping {} — {e}", path.display());
            }
        }
    }
    notes
}

fn cmd_recall(areas: Vec<String>, tags: Vec<String>, task: Option<String>, limit: usize) {
    let log_area = areas.first().cloned();
    let log_tag = tags.first().cloned();
    let log_task = task.clone();

    let notes = load_all_notes();
    if notes.is_empty() {
        println!("No notes found");
        write_debug_event(serde_json::json!({
            "ts": Local::now().to_rfc3339(),
            "cmd": "recall",
            "area": log_area,
            "tag": log_tag,
            "task": log_task,
            "result_count": 0
        }));
        return;
    }

    let filter = RecallFilter {
        areas,
        tags,
        task,
    };

    let scored = score_and_filter_notes(notes, &filter);

    if scored.is_empty() {
        println!("No notes found");
        write_debug_event(serde_json::json!({
            "ts": Local::now().to_rfc3339(),
            "cmd": "recall",
            "area": log_area,
            "tag": log_tag,
            "task": log_task,
            "result_count": 0
        }));
        return;
    }

    let result_count = scored.len().min(limit);
    for (_, _, _, fm) in scored.into_iter().take(limit) {
        let areas_str = if fm.areas.is_empty() {
            "(none)".to_string()
        } else {
            fm.areas.join(", ")
        };
        let tags_str = if fm.tags.is_empty() {
            "(none)".to_string()
        } else {
            fm.tags.join(", ")
        };
        println!("--- {} ---", fm.id);
        println!("signal: {}", fm.signal);
        println!("areas:  {}", areas_str);
        println!("tags:   {}", tags_str);
        println!("task:   {}", fm.task);
        println!("date:   {}", fm.created_at);
        println!();
    }
    write_debug_event(serde_json::json!({
        "ts": Local::now().to_rfc3339(),
        "cmd": "recall",
        "area": log_area,
        "tag": log_tag,
        "task": log_task,
        "result_count": result_count
    }));
}

fn build_brief(
    all_notes: Vec<(PathBuf, NoteFrontmatter)>,
    areas: Vec<String>,
    tags: Vec<String>,
    task: Option<String>,
    limit: usize,
) -> String {
    // Build filter summary for the header line
    let mut filter_parts: Vec<String> = areas.clone();
    for t in &tags {
        filter_parts.push(format!("tag:{t}"));
    }
    if let Some(ref t) = task {
        filter_parts.push(format!("task:{t}"));
    }
    let filter_summary = if filter_parts.is_empty() {
        "(all)".to_string()
    } else {
        filter_parts.join(", ")
    };

    let filter = RecallFilter { areas: areas.clone(), tags, task };

    // 1. Score & filter notes using shared helper (which applies recency boost)
    let scored = score_and_filter_notes(all_notes, &filter);

    // 2. Collect supersedes from matched notes only
    let superseded_by_matched: std::collections::HashSet<String> = scored
        .iter()
        .flat_map(|(_, _, _, fm)| fm.supersedes.iter().cloned())
        .collect();

    // 3. Remove matched notes whose id is in that matched-superseded set
    let mut current: Vec<(i32, String, PathBuf, NoteFrontmatter)> = scored
        .into_iter()
        .filter(|(_, _, _, fm)| !superseded_by_matched.contains(&fm.id))
        .collect();

    let mut out = String::new();
    out.push_str("=== Change Safety Brief ===\n");
    out.push_str(&format!("area: {filter_summary}\n"));

    if current.is_empty() {
        let area_str = areas.first().map(String::as_str).unwrap_or(&filter_summary);
        out.push_str("\nNo project memory found for this area.\n");
        out.push_str(&format!("Run 'pnotes recall --area {area_str}' to explore related notes,\n"));
        out.push_str(&format!("or 'pnotes add continuity --task <slug> --signal \"...\" --area {area_str}'\n"));
        out.push_str("to start capturing context.\n");
        return out;
    }

    // Sort: score DESC, then created_at DESC; take top `limit`
    current.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    let top: Vec<_> = current.into_iter().take(limit).map(|(_, _, path, fm)| (path, fm)).collect();

    out.push_str(&format!("notes: {} matched\n\n", top.len()));

    let sections: &[(&str, fn(&NoteFrontmatter) -> &Vec<String>)] = &[
        ("DECISIONS", |fm| &fm.decisions),
        ("INVARIANTS", |fm| &fm.invariants),
        ("RISKS", |fm| &fm.risks),
    ];

    for (name, getter) in sections {
        out.push_str(&format!("{name}\n"));
        let mut has_items = false;
        for (_, fm) in &top {
            for item in getter(fm) {
                out.push_str(&format!("- {item} (from: {}, {})\n", fm.id, fm.created_at));
                has_items = true;
            }
        }
        if !has_items {
            out.push_str("(none)\n");
        }
        out.push('\n');
    }

    out.push_str("TESTS\n");
    let mut has_tests = false;
    for (_, fm) in &top {
        for test in &fm.tests {
            if test.covers.is_empty() {
                out.push_str(&format!(
                    "- {} (from: {}, {})\n",
                    test.command, fm.id, fm.created_at
                ));
            } else {
                out.push_str(&format!(
                    "- {} covers: {} (from: {}, {})\n",
                    test.command,
                    test.covers.join("; "),
                    fm.id,
                    fm.created_at
                ));
            }
            has_tests = true;
        }
    }
    if !has_tests {
        out.push_str("(none)\n");
    }
    out.push('\n');

    out.push_str("MISSING TESTS\n");
    let mut has_missing_tests = false;
    for (_, fm) in &top {
        for item in &fm.missing_tests {
            out.push_str(&format!("- {item} (from: {}, {})\n", fm.id, fm.created_at));
            has_missing_tests = true;
        }
    }
    if !has_missing_tests {
        out.push_str("(none)\n");
    }
    out.push('\n');

    out.push_str("RECENT CONTINUITY NOTES\n");
    if top.is_empty() {
        out.push_str("(none)\n");
    } else {
        for (path, fm) in &top {
            out.push_str(&format!(
                "- {}\n  signal: {}\n",
                path.display(),
                fm.signal
            ));
        }
    }

    out
}

fn cmd_brief(areas: Vec<String>, tags: Vec<String>, task: Option<String>, limit: usize) {
    let log_area = areas.first().cloned();
    let log_tag = tags.first().cloned();
    let log_task = task.clone();

    let notes = load_all_notes();
    // Score separately to obtain result_count for logging without changing build_brief signature.
    let filter = RecallFilter { areas: areas.clone(), tags: tags.clone(), task: task.clone() };
    let result_count = score_and_filter_notes(notes.clone(), &filter).len().min(limit);

    let output = build_brief(notes, areas, tags, task, limit);
    print!("{output}");

    write_debug_event(serde_json::json!({
        "ts": Local::now().to_rfc3339(),
        "cmd": "brief",
        "area": log_area,
        "tag": log_tag,
        "task": log_task,
        "result_count": result_count
    }));
}

fn cmd_show(id: &str) {
    let path = notes_dir().join(format!("{id}.md"));
    if !path.exists() {
        eprintln!("Note not found: {id}");
        std::process::exit(1);
    }
    let content = fs::read_to_string(&path).expect("Failed to read note");
    print!("{content}");
}

// ── Quality Review commands ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LastReview {
    id: Option<String>,
    reviewed_at: Option<String>,
    reviewed_until: Option<String>,
    notes_reviewed: usize,
    average_score: Option<f64>,
    decision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReviewLogFrontmatter {
    schema_version: usize,
    last_review: LastReview,
}

#[derive(Debug, Clone, Deserialize)]
struct ReviewInput {
    id: String,
    reviewed_at: String,
    reviewed_until: String,
    notes_reviewed: usize,
    average_score: f64,
    decision: String,
}

fn review_log_path() -> PathBuf {
    PathBuf::from(".agents/skills/project-notes/self-improvement/note-quality-review-log.md")
}

fn ensure_review_log_exists() -> Result<(), String> {
    let path = review_log_path();
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directories for review log: {e}"))?;
        }
        let initial_content = r#"---
schema_version: 1
last_review:
  id: null
  reviewed_at: null
  reviewed_until: null
  notes_reviewed: 0
  average_score: null
  decision: null
---

# Note Quality Review Log

<!-- Append reviews below. Do not rewrite or remove this header. -->
"#;
        fs::write(&path, initial_content).map_err(|e| format!("Failed to write initial review log: {e}"))?;
    }
    Ok(())
}

fn split_frontmatter_and_body(content: &str) -> Option<(String, String)> {
    let content_trimmed = content.trim_start();
    if !content_trimmed.starts_with("---") {
        return None;
    }
    let after_first = &content_trimmed[3..];
    let end_idx = after_first.find("\n---")?;
    let body_idx = end_idx + 4; // skip '\n' and '---'
    let full_end_idx = 3 + body_idx;
    if full_end_idx <= content_trimmed.len() {
        let frontmatter = content_trimmed[..full_end_idx].to_string();
        let body = content_trimmed[full_end_idx..].to_string();
        Some((frontmatter, body))
    } else {
        None
    }
}

fn parse_review_log_frontmatter(content: &str) -> Result<ReviewLogFrontmatter, String> {
    let yaml = extract_frontmatter(content).ok_or("No frontmatter found in review log")?;
    serde_yaml::from_str(yaml).map_err(|e| format!("Review log YAML parse error: {e}"))
}

fn parse_datetime(s: &str, default_offset: chrono::FixedOffset) -> Result<chrono::DateTime<chrono::FixedOffset>, String> {
    use chrono::TimeZone;
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Ok(dt);
    }
    if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
            if let Some(dt) = default_offset.from_local_datetime(&ndt).single() {
                return Ok(dt);
            }
        }
    }
    Err(format!("Failed to parse datetime: {s}"))
}

// Reads last 20 events from LOG_PATH and returns a formatted `log_signals` section,
// or None if the log file does not exist.
// Thresholds are starting points; calibrate after ~50 real events.
fn compute_log_signals() -> Option<String> {
    let log_path = PathBuf::from(LOG_PATH);
    if !log_path.exists() {
        return None;
    }
    let content = fs::read_to_string(&log_path).ok()?;
    let sample: Vec<serde_json::Value> = content
        .lines()
        .rev()
        .take(20)
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let sample_size = sample.len();

    let recall_total = sample.iter()
        .filter(|e| matches!(e["cmd"].as_str(), Some("recall") | Some("brief")))
        .count();
    let recall_empty = sample.iter()
        .filter(|e| matches!(e["cmd"].as_str(), Some("recall") | Some("brief")))
        .filter(|e| e["result_count"].as_u64().unwrap_or(1) == 0)
        .count();
    let recall_empty_rate = if recall_total > 0 {
        recall_empty as f64 / recall_total as f64
    } else {
        0.0
    };

    let add_total = sample.iter().filter(|e| e["cmd"].as_str() == Some("add")).count();
    let add_no_area = sample.iter()
        .filter(|e| e["cmd"].as_str() == Some("add"))
        .filter(|e| !e["has_area"].as_bool().unwrap_or(true))
        .count();
    let area_false_rate = if add_total > 0 {
        add_no_area as f64 / add_total as f64
    } else {
        0.0
    };

    // Thresholds: recall_empty_result_rate > 0.25, note_has_area_false_rate > 0.20
    let mut concern_reasons: Vec<&str> = vec![];
    if recall_empty_rate > 0.25 {
        concern_reasons.push("recall_empty_result_rate_exceeded");
    }
    if area_false_rate > 0.20 {
        concern_reasons.push("note_has_area_false_rate_exceeded");
    }
    let concern = !concern_reasons.is_empty();

    let mut out = String::new();
    out.push_str("log_signals:\n");
    out.push_str(&format!("  sample_size: {sample_size}\n"));
    out.push_str(&format!("  recall_empty_result_rate: {recall_empty_rate:.2}\n"));
    out.push_str(&format!("  note_has_area_false_rate: {area_false_rate:.2}\n"));
    out.push_str(&format!("  concern: {concern}\n"));
    if concern_reasons.is_empty() {
        out.push_str("  concern_reasons: []\n");
    } else {
        out.push_str("  concern_reasons:\n");
        for reason in &concern_reasons {
            out.push_str(&format!("  - {reason}\n"));
        }
    }
    Some(out)
}

fn cmd_quality_status() {
    if let Err(e) = ensure_review_log_exists() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    let log_path = review_log_path();
    let content = match fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading review log: {e}");
            std::process::exit(1);
        }
    };

    let log_fm = match parse_review_log_frontmatter(&content) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("Error parsing review log frontmatter: {e}");
            std::process::exit(1);
        }
    };

    let reviewed_until = log_fm.last_review.reviewed_until;
    let notes = load_all_notes();

    let mut pending = vec![];
    use chrono::Offset;
    let local_offset = chrono::Local::now().offset().fix();

    for (path, fm) in notes {
        if fm.note_type != "continuity" {
            continue;
        }
        let is_pending = match &reviewed_until {
            None => true,
            Some(until_str) => {
                if let Ok(until_dt) = parse_datetime(until_str, local_offset) {
                    if let Ok(note_dt) = parse_datetime(&fm.created_at, *until_dt.offset()) {
                        note_dt > until_dt
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        };
        if is_pending {
            pending.push((path, fm));
        }
    }

    pending.sort_by(|a, b| {
        let a_dt = parse_datetime(&a.1.created_at, local_offset).ok();
        let b_dt = parse_datetime(&b.1.created_at, local_offset).ok();
        match (a_dt, b_dt) {
            (Some(da), Some(db)) => da.cmp(&db).then_with(|| a.0.cmp(&b.0)),
            _ => a.1.created_at.cmp(&b.1.created_at).then_with(|| a.0.cmp(&b.0)),
        }
    });

    let count = pending.len();
    let review_required = if count >= 5 { "yes" } else { "no" };
    let trigger = if count >= 5 {
        "new_notes_since_last_review >= 5"
    } else {
        "below_threshold"
    };

    let last_review_str = match &reviewed_until {
        Some(s) => s.as_str(),
        None => "null",
    };

    println!("last_review: {last_review_str}");
    println!("new_notes: {count}");
    println!("review_required: {review_required}");
    println!("trigger: {trigger}");
    println!();
    println!("notes:");
    for (path, _) in &pending {
        println!("- {}", path.display());
    }

    if let Some(signals) = compute_log_signals() {
        println!();
        print!("{signals}");
    }
}

fn cmd_quality_record(from: &PathBuf) {
    let json_content = match fs::read_to_string(from) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading JSON file '{}': {e}", from.display());
            std::process::exit(1);
        }
    };

    let input: ReviewInput = match serde_json::from_str(&json_content) {
        Ok(inp) => inp,
        Err(e) => {
            eprintln!("Error parsing JSON from '{}': {e}", from.display());
            std::process::exit(1);
        }
    };

    let decision = &input.decision;
    if decision != "keep"
        && decision != "amend"
        && decision != "rollback"
        && decision != "inconclusive"
    {
        eprintln!(
            "Error: invalid decision '{}'. Must be one of: keep, amend, rollback, inconclusive",
            decision
        );
        std::process::exit(1);
    }

    if let Err(e) = ensure_review_log_exists() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    let log_path = review_log_path();
    let old_content = match fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading review log: {e}");
            std::process::exit(1);
        }
    };

    let (frontmatter_part, body_part) = match split_frontmatter_and_body(&old_content) {
        Some(parts) => parts,
        None => {
            eprintln!("Error: could not parse frontmatter in review log");
            std::process::exit(1);
        }
    };

    let mut log_fm = match parse_review_log_frontmatter(&frontmatter_part) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("Error parsing review log frontmatter: {e}");
            std::process::exit(1);
        }
    };

    log_fm.last_review = LastReview {
        id: Some(input.id.clone()),
        reviewed_at: Some(input.reviewed_at.clone()),
        reviewed_until: Some(input.reviewed_until.clone()),
        notes_reviewed: input.notes_reviewed,
        average_score: Some(input.average_score),
        decision: Some(input.decision.clone()),
    };

    let new_yaml = match serde_yaml::to_string(&log_fm) {
        Ok(y) => y,
        Err(e) => {
            eprintln!("Error serializing review log frontmatter: {e}");
            std::process::exit(1);
        }
    };

    let body_trimmed = body_part.trim_start_matches('\n').trim_start_matches('\r');
    let new_content = format!("---\n{new_yaml}---\n\n{body_trimmed}");

    if let Err(e) = fs::write(&log_path, new_content) {
        eprintln!("Error writing updated review log: {e}");
        std::process::exit(1);
    }

    println!("recorded: {}", input.id);
    println!("last_review: {}", input.reviewed_until);
    println!("notes_reviewed: {}", input.notes_reviewed);
    println!("average_score: {}", input.average_score);
    println!("decision: {}", input.decision);
}


fn guide_text() -> &'static str {
    r#"Project Notes Usage Guide for LLMs
==================================

Purpose:
- Use project-local notes to avoid rediscovering decisions, invariants, risks, tests, missing tests, traps, and dead ends.
- Markdown notes are source of truth. The CLI is a recall/brief helper.

Before implementation or broad source exploration:
1. Identify the target area, task, or tag.
2. Prefer:
   pnotes brief --area <area> --limit 5
3. If brief is unavailable or empty, fallback to:
   pnotes recall --area <area> --limit 5
4. Read only returned notes that are relevant.
5. Do not scan all .project-notes/notes by default.

Commands:
  pnotes init
    Create .project-notes structure.

  pnotes brief --area <path> [--tag <tag>] [--task <task>] [--limit <n>]
    Generate a Change Safety Brief from matched notes.

  pnotes recall --area <path> [--tag <tag>] [--task <task>] [--limit <n>]
    Return relevant note ids/paths/signals.

  pnotes add continuity ...
    Create a continuity note after implementation output.

  pnotes show <id>
    Print one full note.

  pnotes quality status
    Check pending notes and if a quality review is required.

  pnotes quality record --from <review-result.json>
    Record a completed quality review result.

After implementation output:
- Create a continuity note.
- Include decisions, invariants, risks, tests, and missing_tests when applicable.
- Do not use project notes as a changelog.

Completion gate:
- A task with implementation output is not complete until a continuity note is created or a valid skip reason is stated.
"#
}

fn cmd_guide() {
    println!("{}", guide_text());
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let raw_args: Vec<OsString> = std::env::args_os().collect();
    // Clap can collect repeatable values but does not preserve the grouping we need
    // for Option A. We parse raw args so each --test-covers attaches to the nearest
    // preceding --test-command.
    let (tests, missing_tests) = match parse_test_metadata_from_args(raw_args.iter().cloned()) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    let cli = Cli::parse_from(raw_args);
    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Add { note_type } => match note_type {
            NoteType::Continuity {
                task,
                signal,
                area,
                tag,
                handoff,
                run,
                decision,
                invariant,
                risk,
                test_command: _,
                test_covers: _,
                missing_test: _,
            } => cmd_add_continuity(
                task,
                signal,
                area,
                tag,
                handoff,
                run,
                decision,
                invariant,
                risk,
                tests,
                missing_tests,
            ),
        },
        Commands::Recall {
            area,
            tag,
            task,
            limit,
        } => cmd_recall(area, tag, task, limit),
        Commands::Show { id } => cmd_show(&id),
        Commands::Guide => cmd_guide(),
        Commands::Brief { area, tag, task, limit } => cmd_brief(area, tag, task, limit),
        Commands::Quality { sub } => match sub {
            QualitySubcommands::Status => cmd_quality_status(),
            QualitySubcommands::Record { from } => cmd_quality_record(&from),
        },
    }
}

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fm(task: &str, areas: Vec<&str>, tags: Vec<&str>, signal: &str) -> NoteFrontmatter {
        NoteFrontmatter {
            id: format!("2026-05-25-{task}"),
            note_type: "continuity".to_string(),
            task: task.to_string(),
            created_at: "2026-05-25".to_string(),
            signal: signal.to_string(),
            run: None,
            handoff: None,
            areas: areas.into_iter().map(String::from).collect(),
            tags: tags.into_iter().map(String::from).collect(),
            read_when: vec![],
            supersedes: vec![],
            decisions: vec![],
            invariants: vec![],
            risks: vec![],
            tests: vec![],
            missing_tests: vec![],
        }
    }

    fn filter(areas: Vec<&str>, tags: Vec<&str>, task: Option<&str>) -> RecallFilter {
        RecallFilter {
            areas: areas.into_iter().map(String::from).collect(),
            tags: tags.into_iter().map(String::from).collect(),
            task: task.map(String::from),
        }
    }

    #[test]
    fn test_path_normalization() {
        assert_eq!(normalize_path("src/auth/"), "src/auth");
        assert_eq!(normalize_path("SRC/Auth"), "src/auth");
        assert_eq!(normalize_path("src/auth"), "src/auth");
    }

    #[test]
    fn test_scoring_exact_area() {
        let fm = make_fm("fix", vec!["src/session-manager"], vec![], "test");
        let f = filter(vec!["src/session-manager"], vec![], None);
        assert_eq!(score_note(&fm, &f), 5);
    }

    #[test]
    fn test_scoring_prefix_area() {
        // note area: src/session-manager, query: src/
        let fm = make_fm("fix", vec!["src/session-manager"], vec![], "test");
        let f = filter(vec!["src/"], vec![], None);
        assert_eq!(score_note(&fm, &f), 4);

        // Reverse: note area src/, query src/session-manager
        let fm2 = make_fm("fix", vec!["src/"], vec![], "test");
        let f2 = filter(vec!["src/session-manager"], vec![], None);
        assert_eq!(score_note(&fm2, &f2), 4);
    }

    #[test]
    fn test_scoring_task_match() {
        let fm = make_fm("auth-fix", vec![], vec![], "test");
        let f = filter(vec![], vec![], Some("auth-fix"));
        assert_eq!(score_note(&fm, &f), 3);
    }

    #[test]
    fn test_scoring_no_match() {
        let fm = make_fm("unrelated", vec!["lib/util"], vec!["perf"], "minor fix");
        let f = filter(vec!["src/auth"], vec!["bug"], Some("auth-fix"));
        assert_eq!(score_note(&fm, &f), 0);
    }

    #[test]
    fn test_frontmatter_parse_valid() {
        let content = r#"---
id: "2026-05-25-test"
type: "continuity"
task: "test"
created_at: "2026-05-25"
signal: "hello world"
---

## task
test
"#;
        let fm = parse_frontmatter(content).unwrap();
        assert_eq!(fm.id, "2026-05-25-test");
        assert_eq!(fm.task, "test");
        assert_eq!(fm.signal, "hello world");
    }

    #[test]
    fn test_frontmatter_parse_missing_required() {
        // Missing signal field
        let content = r#"---
id: "2026-05-25-test"
type: "continuity"
task: "test"
created_at: "2026-05-25"
---
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_frontmatter_parse_malformed_yaml() {
        let content = r#"---
id: [bad yaml
---
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_err());
    }

    // Helper: wrap a NoteFrontmatter into the Vec format expected by build_brief
    fn wrap(fm: NoteFrontmatter) -> Vec<(PathBuf, NoteFrontmatter)> {
        vec![(PathBuf::from("fake.md"), fm)]
    }

    #[test]
    fn test_brief_empty() {
        let out = build_brief(vec![], vec!["src/no-such-area".to_string()], vec![], None, 3);
        assert!(out.contains("No project memory found for this area."));
        assert!(out.contains("area: src/no-such-area"));
    }

    #[test]
    fn test_brief_superseded_excluded() {
        let mut note_a = make_fm("note-a", vec!["src/foo"], vec![], "signal a");
        note_a.decisions = vec!["Decision from A".to_string()];

        let mut note_b = make_fm("note-b", vec!["src/foo"], vec![], "signal b");
        note_b.supersedes = vec!["2026-05-25-note-a".to_string()];
        note_b.decisions = vec!["Decision from B".to_string()];

        let notes = vec![
            (PathBuf::from("note-a.md"), note_a),
            (PathBuf::from("note-b.md"), note_b),
        ];
        let out = build_brief(notes, vec!["src/foo".to_string()], vec![], None, 3);
        assert!(!out.contains("Decision from A"), "superseded note-a should be excluded");
        assert!(out.contains("Decision from B"));
    }

    #[test]
    fn test_brief_two_notes_no_supersede() {
        let mut note_a = make_fm("note-a", vec!["src/foo"], vec![], "signal a");
        note_a.decisions = vec!["Decision A".to_string()];

        let mut note_b = make_fm("note-b", vec!["src/foo"], vec![], "signal b");
        note_b.id = "2026-05-24-note-b".to_string();
        note_b.created_at = "2026-05-24".to_string();
        note_b.decisions = vec!["Decision B".to_string()];

        let notes = vec![
            (PathBuf::from("note-a.md"), note_a),
            (PathBuf::from("note-b.md"), note_b),
        ];
        let out = build_brief(notes, vec!["src/foo".to_string()], vec![], None, 3);
        assert!(out.contains("Decision A"));
        assert!(out.contains("Decision B"));
        assert!(out.contains("from: 2026-05-25-note-a, 2026-05-25"));
        assert!(out.contains("from: 2026-05-24-note-b, 2026-05-24"));
    }

    #[test]
    fn test_brief_backward_compat() {
        // Legacy note without the 5 new fields
        let content = r#"---
id: "2026-05-25-legacy"
type: "continuity"
task: "legacy"
created_at: "2026-05-25"
signal: "legacy signal"
areas:
  - src/foo
---
"#;
        let fm = parse_frontmatter(content).expect("legacy note should parse without error");
        assert!(fm.decisions.is_empty());
        assert!(fm.invariants.is_empty());

        let out = build_brief(wrap(fm), vec!["src/foo".to_string()], vec![], None, 3);
        assert!(out.contains("(none)"), "empty sections should show (none)");
    }

    #[test]
    fn test_add_continuity_with_capture_flags() {
        // Verify that decisions/invariants/risks round-trip through YAML serialization
        let fm = NoteFrontmatter {
            id: "2026-05-25-capture-test".to_string(),
            note_type: "continuity".to_string(),
            task: "capture-test".to_string(),
            created_at: "2026-05-25".to_string(),
            signal: "test signal".to_string(),
            run: None,
            handoff: None,
            areas: vec![],
            tags: vec![],
            read_when: vec![],
            supersedes: vec![],
            decisions: vec!["D1".to_string()],
            invariants: vec!["I1".to_string()],
            risks: vec!["R1".to_string()],
            tests: vec![],
            missing_tests: vec![],
        };
        let yaml = serde_yaml::to_string(&fm).expect("serialize ok");
        let content = format!("---\n{yaml}---\n");
        let parsed = parse_frontmatter(&content).expect("parse ok");
        assert_eq!(parsed.decisions, vec!["D1"]);
        assert_eq!(parsed.invariants, vec!["I1"]);
        assert_eq!(parsed.risks, vec!["R1"]);
    }

    #[test]
    fn test_parse_one_test_command_with_multiple_covers() {
        let (tests, missing_tests) = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--test-command",
            "cargo test",
            "--test-covers",
            "stores exit_code",
            "--test-covers",
            "closes session",
        ])
        .expect("metadata should parse");

        assert!(missing_tests.is_empty());
        assert_eq!(
            tests,
            vec![TestEntry {
                command: "cargo test".to_string(),
                covers: vec!["stores exit_code".to_string(), "closes session".to_string()],
            }]
        );
    }

    #[test]
    fn test_parse_multiple_test_command_groups() {
        let (tests, _) = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--test-command",
            "cargo test",
            "--test-covers",
            "session lifecycle",
            "--test-command",
            "cargo test flush_running_tasks_updates_db",
            "--test-covers",
            "shutdown update path",
        ])
        .expect("metadata should parse");

        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0].command, "cargo test");
        assert_eq!(tests[0].covers, vec!["session lifecycle"]);
        assert_eq!(
            tests[1].command,
            "cargo test flush_running_tasks_updates_db"
        );
        assert_eq!(tests[1].covers, vec!["shutdown update path"]);
    }

    #[test]
    fn test_parse_repeatable_missing_tests() {
        let (_, missing_tests) = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--missing-test",
            "No E2E SIGTERM test.",
            "--missing-test",
            "No restart recovery test.",
        ])
        .expect("metadata should parse");

        assert_eq!(
            missing_tests,
            vec!["No E2E SIGTERM test.", "No restart recovery test."]
        );
    }

    #[test]
    fn test_parse_test_covers_requires_preceding_command() {
        let result = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--test-covers",
            "orphan coverage",
        ]);

        assert_eq!(
            result.expect_err("orphan coverage should fail"),
            "--test-covers requires a preceding --test-command"
        );
    }

    #[test]
    fn test_frontmatter_serializes_tests_and_missing_tests() {
        let fm = NoteFrontmatter {
            id: "2026-05-25-test-metadata".to_string(),
            note_type: "continuity".to_string(),
            task: "test-metadata".to_string(),
            created_at: "2026-05-25".to_string(),
            signal: "test metadata signal".to_string(),
            run: None,
            handoff: None,
            areas: vec![],
            tags: vec![],
            read_when: vec![],
            supersedes: vec![],
            decisions: vec![],
            invariants: vec![],
            risks: vec![],
            tests: vec![
                TestEntry {
                    command: "cargo test".to_string(),
                    covers: vec![
                        "subprocess exit detection stores exit_code and closes session".to_string(),
                        "cancel endpoint transitions session safely".to_string(),
                    ],
                },
                TestEntry {
                    command: "cargo test flush_running_tasks_updates_db".to_string(),
                    covers: vec!["graceful shutdown SQL update path".to_string()],
                },
            ],
            missing_tests: vec!["No E2E test for SIGTERM graceful shutdown.".to_string()],
        };

        let yaml = serde_yaml::to_string(&fm).expect("serialize ok");

        assert!(yaml.contains("tests:"));
        assert!(yaml.contains("- command: cargo test"));
        assert!(yaml.contains("- subprocess exit detection stores exit_code and closes session"));
        assert!(yaml.contains("- command: cargo test flush_running_tasks_updates_db"));
        assert!(yaml.contains("missing_tests:"));
        assert!(yaml.contains("- No E2E test for SIGTERM graceful shutdown."));

        let content = format!("---\n{yaml}---\n");
        let parsed = parse_frontmatter(&content).expect("parse ok");
        assert_eq!(parsed.tests.len(), 2);
        assert_eq!(
            parsed.missing_tests,
            vec!["No E2E test for SIGTERM graceful shutdown."]
        );
    }

    #[test]
    fn test_brief_aggregates_tests_and_missing_tests() {
        let mut note = make_fm("test-metadata", vec!["src/session"], vec![], "signal");
        note.tests = vec![TestEntry {
            command: "cargo test".to_string(),
            covers: vec!["session lifecycle".to_string(), "shutdown path".to_string()],
        }];
        note.missing_tests = vec!["No E2E SIGTERM test.".to_string()];

        let out = build_brief(wrap(note), vec!["src/session".to_string()], vec![], None, 3);

        assert!(out.contains("TESTS"));
        assert!(out.contains("cargo test covers: session lifecycle; shutdown path"));
        assert!(out.contains("MISSING TESTS"));
        assert!(out.contains("No E2E SIGTERM test."));
    }

    #[test]
    fn test_brief_supersedes_only_excludes_notes_superseded_by_matched_notes() {
        let mut old_backend = make_fm("old-backend", vec!["backend/src/services"], vec![], "old backend");
        old_backend.id = "old-backend".to_string();
        old_backend.decisions = vec!["Old backend decision".to_string()];

        let mut new_frontend = make_fm("new-frontend", vec!["frontend/src"], vec![], "new frontend");
        new_frontend.id = "new-frontend".to_string();
        new_frontend.supersedes = vec!["old-backend".to_string()];
        new_frontend.decisions = vec!["Frontend replacement".to_string()];

        let notes = vec![
            (PathBuf::from("old_backend.md"), old_backend),
            (PathBuf::from("new_frontend.md"), new_frontend),
        ];

        let out = build_brief(notes.clone(), vec!["backend/src/services".to_string()], vec![], None, 3);
        assert!(out.contains("Old backend decision"), "Should include Old backend decision because new-frontend did not match the brief query");

        let mut new_backend = make_fm("new-backend", vec!["backend/src/services"], vec![], "new backend");
        new_backend.id = "new-backend".to_string();
        new_backend.supersedes = vec!["old-backend".to_string()];
        new_backend.decisions = vec!["New backend decision".to_string()];

        let mut notes_updated = notes;
        notes_updated.push((PathBuf::from("new_backend.md"), new_backend));

        let out2 = build_brief(notes_updated, vec!["backend/src/services".to_string()], vec![], None, 3);
        assert!(out2.contains("New backend decision"));
        assert!(!out2.contains("Old backend decision"), "Should exclude Old backend decision because new-backend matches the brief query and supersedes it");
    }

    #[test]
    fn test_brief_respects_limit() {
        let mut note_a = make_fm("note-a", vec!["src/foo"], vec![], "signal a");
        note_a.id = "2026-05-25-note-a".to_string();
        note_a.decisions = vec!["Decision A".to_string()];
        note_a.created_at = "2026-05-25".to_string();

        let mut note_b = make_fm("note-b", vec!["src/foo"], vec![], "signal b");
        note_b.id = "2026-05-24-note-b".to_string();
        note_b.decisions = vec!["Decision B".to_string()];
        note_b.created_at = "2026-05-24".to_string();

        let notes = vec![
            (PathBuf::from("note-a.md"), note_a),
            (PathBuf::from("note-b.md"), note_b),
        ];

        let out = build_brief(notes, vec!["src/foo".to_string()], vec![], None, 1);
        assert!(out.contains("Decision A"));
        assert!(!out.contains("Decision B"));
    }

    #[test]
    fn test_parse_test_metadata_supports_equals_syntax_for_test_command() {
        let (tests, _) = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--test-command=cargo test",
        ])
        .expect("metadata should parse");

        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].command, "cargo test");
    }

    #[test]
    fn test_parse_test_metadata_supports_equals_syntax_for_test_covers() {
        let (tests, _) = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--test-command",
            "cargo test",
            "--test-covers=behavior A",
            "--test-covers=behavior B",
        ])
        .expect("metadata should parse");

        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].covers, vec!["behavior A".to_string(), "behavior B".to_string()]);
    }

    #[test]
    fn test_parse_test_metadata_supports_equals_syntax_for_missing_test() {
        let (_, missing) = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--missing-test=gap A",
        ])
        .expect("metadata should parse");

        assert_eq!(missing, vec!["gap A".to_string()]);
    }

    #[test]
    fn test_parse_test_metadata_errors_when_equals_test_covers_has_no_command() {
        let result = parse_test_metadata_from_args([
            "pnotes",
            "add",
            "continuity",
            "--test-covers=orphan",
        ]);

        assert_eq!(
            result.expect_err("orphan should fail"),
            "--test-covers requires a preceding --test-command"
        );
    }

    #[test]
    fn test_add_continuity_uses_resolved_filename_as_id_when_collision_occurs() {
        let test_dir = std::env::temp_dir().join(format!("pnotes-test-collision-{}", std::process::id()));
        if test_dir.exists() {
            std::fs::remove_dir_all(&test_dir).unwrap();
        }
        std::fs::create_dir_all(&test_dir).unwrap();
        std::env::set_var("PNOTES_DIR", &test_dir);

        let task = "collision-task".to_string();
        let signal = "first signal".to_string();
        let date = Local::now().format("%Y-%m-%d").to_string();

        cmd_add_continuity(
            Some(task.clone()),
            Some(signal.clone()),
            vec![],
            vec![],
            None,
            None,
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );

        let first_file = test_dir.join(format!("{date}-{task}.md"));
        assert!(first_file.exists());
        let first_content = fs::read_to_string(&first_file).unwrap();
        let first_fm = parse_frontmatter(&first_content).unwrap();
        assert_eq!(first_fm.id, format!("{date}-{task}"));

        cmd_add_continuity(
            Some(task.clone()),
            Some("second signal".to_string()),
            vec![],
            vec![],
            None,
            None,
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );

        let second_file = test_dir.join(format!("{date}-{task}-2.md"));
        assert!(second_file.exists());
        let second_content = fs::read_to_string(&second_file).unwrap();
        let second_fm = parse_frontmatter(&second_content).unwrap();
        assert_eq!(second_fm.id, format!("{date}-{task}-2"));

        std::fs::remove_dir_all(&test_dir).unwrap();
        std::env::remove_var("PNOTES_DIR");
    }

    #[test]
    fn test_brief_prints_recent_continuity_notes() {
        let mut note = make_fm("test-recent", vec!["src/session"], vec![], "My special signal message");
        note.id = "2026-05-26-test-recent".to_string();

        let out = build_brief(
            vec![(PathBuf::from(".project-notes/notes/2026-05-26-test-recent.md"), note)],
            vec!["src/session".to_string()],
            vec![],
            None,
            3,
        );

        assert!(out.contains("RECENT CONTINUITY NOTES"));
        assert!(out.contains(".project-notes/notes/2026-05-26-test-recent.md"));
        assert!(out.contains("signal: My special signal message"));
    }

    #[test]
    fn test_guide_content() {
        let text = guide_text();
        assert!(text.contains("pnotes brief"));
        assert!(text.contains("pnotes recall"));
        assert!(text.contains("Prefer"));
        assert!(text.contains("fallback"));
        assert!(text.contains("pnotes quality status"));
        assert!(text.contains("pnotes quality record"));
    }

    #[test]
    fn test_brief_applies_recency_boost_consistently() {
        let mut note_old = make_fm("note-old", vec!["src/session-manager"], vec![], "old signal");
        note_old.id = "note-old".to_string();
        note_old.created_at = "2026-05-25".to_string();
        note_old.decisions = vec!["Old decision".to_string()];

        let mut note_new = make_fm("note-new", vec!["src/session-manager"], vec![], "new signal");
        note_new.id = "note-new".to_string();
        note_new.created_at = "2026-05-26".to_string();
        note_new.decisions = vec!["New decision".to_string()];

        let notes = vec![
            (PathBuf::from("note-old.md"), note_old),
            (PathBuf::from("note-new.md"), note_new),
        ];

        let out = build_brief(notes, vec!["src/session-manager".to_string()], vec![], None, 1);
        assert!(out.contains("New decision"));
        assert!(!out.contains("Old decision"));
    }

    #[test]
    fn test_shared_scoring_and_filtering() {
        let note_old = make_fm("note-old", vec!["src/session-manager"], vec![], "old");
        let mut note_new = make_fm("note-new", vec!["src/session-manager"], vec![], "new");
        note_new.created_at = "2026-05-26".to_string();

        let notes = vec![
            (PathBuf::from("note-old.md"), note_old),
            (PathBuf::from("note-new.md"), note_new),
        ];

        let f = filter(vec!["src/session-manager"], vec![], None);
        let scored = score_and_filter_notes(notes, &f);

        assert_eq!(scored.len(), 2);
        assert_eq!(scored[0].0, 6);
        assert_eq!(scored[0].3.task, "note-new");
        assert_eq!(scored[1].0, 5);
        assert_eq!(scored[1].3.task, "note-old");
    }

    #[test]
    fn test_parse_datetime() {
        use chrono::Offset;
        let local_offset = chrono::Local::now().offset().fix();
        
        let dt1 = parse_datetime("2026-05-26T20:10:00+07:00", local_offset).unwrap();
        assert_eq!(dt1.to_rfc3339(), "2026-05-26T20:10:00+07:00");
        
        let dt2 = parse_datetime("2026-05-26", local_offset).unwrap();
        let expected = format!("2026-05-26T00:00:00{}", dt2.offset().to_string());
        assert_eq!(dt2.to_rfc3339()[..19], expected[..19]);
        
        assert!(parse_datetime("invalid", local_offset).is_err());
    }

    #[test]
    fn test_split_frontmatter_and_body() {
        let content = r#"---
schema_version: 1
---

# Header
Body text
"#;
        let (fm, body) = split_frontmatter_and_body(content).unwrap();
        assert_eq!(fm, "---\nschema_version: 1\n---");
        assert!(body.contains("# Header"));
        assert!(body.contains("Body text"));
    }
}
