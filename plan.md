# whyno Wine Journal Implementation Plan

This project tracks wines that have been tried. The plan below is intentionally split into commit-sized steps. Each step should produce a self-contained set of changes, leave the application in a working state, be verified, and be committed individually.

Every time a decision is made or a step of this plan is completed, record it in `progress.md`. The progress log should capture the decision or completed step, the reasoning when relevant, the verification performed, and the commit hash once committed.

## Step 1 — Define the wine-tracking CLI contract

Goal: Turn the generic `Add` / `List` skeleton into a wine-specific CLI shape without persistence yet.

Changes:

- Rename the domain internally from “thing” to “wine”.
- Add arguments to `add`, for example:

  ```bash
  whyno add "Château Example" \
    --vintage 2020 \
    --producer "Example Estate" \
    --region "Bordeaux" \
    --country "France" \
    --grape "Merlot" \
    --rating 4 \
    --notes "Smooth, dark fruit"
  ```

- Keep `list` working, but for now it can still say that no storage has been implemented.
- Add basic CLI validation through `clap` where possible:
  - `name` required
  - `rating` constrained later or validated manually
  - optional metadata fields accepted

Application state after step: The CLI parses real wine input and prints a useful confirmation, but does not save anything yet.

Verification:

```bash
cargo fmt --check
cargo check
cargo run -- add "Test Wine" --rating 4 --notes "Nice"
cargo run -- list
```

Commit:

```bash
git add src/main.rs
git commit -m "feat: define wine tracking CLI"
```

## Step 2 — Add the core wine data model

Goal: Introduce a deterministic, testable model for stored wine records before adding file I/O.

Changes:

- Add a `Wine` struct with `serde` support:
  - `id`
  - `name`
  - `producer`
  - `vintage`
  - `region`
  - `country`
  - `grapes`
  - `rating`
  - `notes`
  - `tags`
- Add a `WineInput` or equivalent internal struct representing CLI input before it becomes a stored record.
- Add validation logic:
  - non-empty name
  - rating in an agreed range, likely `1..=5` or `1..=10`
  - optional vintage sanity check if provided
- Keep the logic pure: input data in, validated `Wine` or error out.

Application state after step: The app still does not persist records, but wine records have a clear schema and validation.

Verification:

```bash
cargo fmt --check
cargo check
cargo test
cargo run -- add "Test Wine" --rating 4
cargo run -- add "" --rating 4
cargo run -- add "Bad Rating" --rating 99
```

Commit:

```bash
git add src/main.rs
git commit -m "feat: add wine data model"
```

## Step 3 — Add configurable JSON storage path

Goal: Decide where data lives, but keep storage behavior minimal and safe.

Changes:

- Add a global CLI option:

  ```bash
  whyno --data ./wines.json add "Test Wine"
  whyno --data ./wines.json list
  ```

- Choose a default storage path if `--data` is omitted:
  - Minimal option: `./whyno.json`
  - Better user-facing option later: platform-specific config/data directory
- Implement a small storage path resolver.
- Do not yet fully implement add/list persistence if that makes the step too large.

Application state after step: The app accepts a storage path consistently. Existing commands still work.

Verification:

```bash
cargo fmt --check
cargo check
cargo run -- --data ./tmp-wines.json add "Test Wine" --rating 4
cargo run -- --data ./tmp-wines.json list
```

Commit:

```bash
git add src/main.rs
git commit -m "feat: add configurable data path"
```

## Step 4 — Implement JSON loading and saving

Goal: Add the imperative storage layer while keeping domain logic separate.

Changes:

- Add functions like:
  - `load_wines(path) -> Result<Vec<Wine>>`
  - `save_wines(path, wines) -> Result<()>`
- Behavior:
  - missing file means empty collection
  - invalid JSON returns a helpful error
  - save writes pretty JSON
- Create parent directory if needed, only if the chosen default path requires it.
- Keep file I/O isolated from wine validation and command behavior.

Application state after step: The app can read and write a JSON collection, though commands may still only partially use it depending on how the change is sliced.

Verification:

```bash
cargo fmt --check
cargo check
cargo test
cargo run -- --data ./tmp-wines.json list
printf 'not json\n' > ./tmp-wines.json
cargo run -- --data ./tmp-wines.json list
```

Commit:

```bash
git add src/main.rs
git commit -m "feat: add JSON wine storage"
```

## Step 5 — Make `add` persist wine records

Goal: Make the first truly useful workflow work end-to-end.

Changes:

- `add` should:
  1. load existing wines
  2. validate CLI input
  3. assign a stable ID
  4. append the new wine
  5. save the updated collection
  6. print a clear success message
- Use simple IDs initially:
  - either incrementing integer IDs based on current max
  - or string IDs if adding a UUID dependency later
- Prefer no new dependency at first; an incrementing `u64` ID is enough.

Application state after step: Users can add wines and they are saved to JSON.

Verification:

```bash
rm -f ./tmp-wines.json
cargo fmt --check
cargo check
cargo test
cargo run -- --data ./tmp-wines.json add "Test Wine" --rating 4 --notes "Good"
cat ./tmp-wines.json
cargo run -- --data ./tmp-wines.json add "Second Wine" --rating 5
cat ./tmp-wines.json
```

Expected result:

- JSON file exists.
- It contains two wine records.
- IDs are unique and predictable.

Commit:

```bash
git add src/main.rs
git commit -m "feat: persist added wines"
```

## Step 6 — Make `list` display saved wines

Goal: Complete the minimal useful loop: add wines, then list them.

Changes:

- `list` should:
  1. load wines from the configured data path
  2. print an empty-state message if none exist
  3. display saved wines in a readable format
- Start with simple human-readable output:

  ```text
  1. Château Example 2020 — 4/5
     Producer: Example Estate
     Region: Bordeaux, France
     Grapes: Merlot
     Notes: Smooth, dark fruit
  ```

- Avoid table dependencies for now unless output formatting becomes painful.

Application state after step: The core app is usable as a local wine journal.

Verification:

```bash
rm -f ./tmp-wines.json
cargo fmt --check
cargo check
cargo test
cargo run -- --data ./tmp-wines.json list
cargo run -- --data ./tmp-wines.json add "Test Wine" --rating 4 --region "Rioja"
cargo run -- --data ./tmp-wines.json list
```

Expected result:

- Empty list message before adding.
- Added wine appears after adding.

Commit:

```bash
git add src/main.rs
git commit -m "feat: list saved wines"
```

## Step 7 — Add focused tests for model and storage behavior

Goal: Protect the important behavior before adding more commands.

Changes:

- Add unit tests for:
  - empty name validation
  - invalid rating validation
  - valid wine creation
  - next ID generation
- Add storage tests using a temporary directory.
- This may require adding a dev dependency like `tempfile`.

Application state after step: No user-facing behavior changes, but the project becomes safer to evolve.

Verification:

```bash
cargo fmt --check
cargo test
cargo run -- --data ./tmp-wines.json add "Regression Test Wine" --rating 4
cargo run -- --data ./tmp-wines.json list
```

Commit:

```bash
git add Cargo.toml Cargo.lock src/main.rs
git commit -m "test: cover wine model and storage"
```

## Step 8 — Split code into small modules

Goal: Keep the project maintainable before `main.rs` becomes too large.

Changes:

- Move domain logic into `src/wine.rs`.
- Move JSON file persistence into `src/storage.rs`.
- Keep CLI parsing and command dispatch in `src/main.rs`.
- Preserve existing behavior exactly.

Suggested shape:

```text
src/
  main.rs      # CLI and command dispatch
  wine.rs      # Wine model, validation, formatting helpers
  storage.rs   # load/save JSON file
```

Application state after step: Same behavior as before, cleaner structure.

Verification:

```bash
cargo fmt --check
cargo test
cargo run -- --data ./tmp-wines.json add "Modular Wine" --rating 5
cargo run -- --data ./tmp-wines.json list
```

Commit:

```bash
git add src/main.rs src/wine.rs src/storage.rs
git commit -m "refactor: split wine CLI modules"
```

## Step 9 — Add `show <id>` and `remove <id>`

Goal: Add the next most useful CRUD operations without overcomplicating update flows yet.

Changes:

- Add:

  ```bash
  whyno show 1
  whyno remove 1
  ```

- `show` prints one full wine record.
- `remove` deletes by ID and saves.
- Helpful errors:
  - unknown ID
  - empty storage
- Do not reuse deleted IDs.

Application state after step: Users can inspect and delete records.

Verification:

```bash
rm -f ./tmp-wines.json
cargo fmt --check
cargo test
cargo run -- --data ./tmp-wines.json add "Wine To Remove" --rating 3
cargo run -- --data ./tmp-wines.json show 1
cargo run -- --data ./tmp-wines.json remove 1
cargo run -- --data ./tmp-wines.json list
cargo run -- --data ./tmp-wines.json show 1
```

Expected result:

- `show 1` works before removal.
- `remove 1` succeeds.
- `show 1` after removal returns a useful error.

Commit:

```bash
git add src/main.rs src/wine.rs src/storage.rs
git commit -m "feat: show and remove wines"
```

## Step 10 — Add search and simple filters to `list`

Goal: Make the journal useful once it contains many wines.

Changes:

- Add list filters:

  ```bash
  whyno list --tag favorite
  whyno list --grape merlot
  whyno list --country france
  whyno list --min-rating 4
  whyno list --query rioja
  ```

- Search fields:
  - name
  - producer
  - region
  - country
  - grapes
  - notes
  - tags
- Keep matching case-insensitive.
- Keep filtering logic pure and unit-tested.

Application state after step: Users can find previous wines without scanning the whole list.

Verification:

```bash
rm -f ./tmp-wines.json
cargo fmt --check
cargo test
cargo run -- --data ./tmp-wines.json add "French Merlot" --country France --grape Merlot --rating 4 --tag favorite
cargo run -- --data ./tmp-wines.json add "Italian Red" --country Italy --grape Sangiovese --rating 3
cargo run -- --data ./tmp-wines.json list --country france
cargo run -- --data ./tmp-wines.json list --grape merlot
cargo run -- --data ./tmp-wines.json list --min-rating 4
cargo run -- --data ./tmp-wines.json list --query italian
```

Commit:

```bash
git add src/main.rs src/wine.rs
git commit -m "feat: filter listed wines"
```

## Step 11 — Add `update <id>` for correcting records

Goal: Allow users to fix typos or add notes after the fact.

Changes:

- Add:

  ```bash
  whyno update 1 --rating 5
  whyno update 1 --notes "Better on day two"
  whyno update 1 --tag favorite
  ```

- Only fields provided on the command should change.
- Validate updated record before saving.
- Decide tag behavior:
  - simplest: `--tags a,b,c` replaces tags
  - nicer later: `--add-tag`, `--remove-tag`

Application state after step: Users can maintain records without manually editing JSON.

Verification:

```bash
rm -f ./tmp-wines.json
cargo fmt --check
cargo test
cargo run -- --data ./tmp-wines.json add "Editable Wine" --rating 3
cargo run -- --data ./tmp-wines.json update 1 --rating 5 --notes "Improved"
cargo run -- --data ./tmp-wines.json show 1
cargo run -- --data ./tmp-wines.json update 1 --rating 99
```

Expected result:

- Valid update persists.
- Invalid update fails without corrupting saved data.

Commit:

```bash
git add src/main.rs src/wine.rs src/storage.rs
git commit -m "feat: update wine records"
```

## Step 12 — Improve default data location

Goal: Make the app pleasant for real daily use.

Changes:

- Add a platform-appropriate data location, probably using the `directories` crate:
  - macOS: `~/Library/Application Support/whyno/wines.json`
  - Linux: `~/.local/share/whyno/wines.json`
  - Windows: appropriate app data path
- Keep `--data` override for testing and custom workflows.
- Print the data path in a debug/helpful command only if needed; avoid noisy normal output.

Application state after step: Users no longer need to pass `--data` for normal usage.

Verification:

```bash
cargo fmt --check
cargo test
cargo run -- add "Default Path Wine" --rating 4
cargo run -- list
cargo run -- --data ./tmp-wines.json list
```

Commit:

```bash
git add Cargo.toml Cargo.lock src/main.rs src/storage.rs
git commit -m "feat: use platform data directory"
```

## Step 13 — Add README usage documentation

Goal: Make the tool understandable without reading source code.

Changes:

- Add `README.md` with:
  - what `whyno` does
  - install/run instructions
  - examples for add/list/show/update/remove
  - data file behavior
  - rating scale
  - JSON storage note
- Keep examples matching actual CLI behavior.

Application state after step: No behavior changes; documentation reflects the current working app.

Verification:

```bash
cargo fmt --check
cargo test
cargo run -- --help
cargo run -- add --help
cargo run -- list --help
```

Commit:

```bash
git add README.md
git commit -m "docs: document wine tracking usage"
```

## Step 14 — Add integration tests for CLI workflows

Goal: Verify the real command-line behavior, not just internal functions.

Changes:

- Add CLI integration tests under `tests/`.
- Use crates like:
  - `assert_cmd`
  - `predicates`
  - `tempfile`
- Cover workflows:
  - add then list
  - add then show
  - remove missing ID
  - invalid rating
  - corrupt JSON file
- Always use `--data` with a temp file in tests.

Application state after step: No behavior changes; stronger regression protection.

Verification:

```bash
cargo fmt --check
cargo test
```

Commit:

```bash
git add Cargo.toml Cargo.lock tests/
git commit -m "test: cover CLI wine workflows"
```

## Step 15 — Add quality automation

Goal: Make future changes easy to verify consistently.

Changes:

- Add a minimal CI workflow if this repo is intended to live on GitHub:

  ```text
  .github/workflows/ci.yml
  ```

- CI should run:

  ```bash
  cargo fmt --check
  cargo clippy -- -D warnings
  cargo test
  ```

- Optionally add local instructions to README.

Application state after step: No behavior changes; every pushed change can be automatically verified.

Verification:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Commit:

```bash
git add .github/workflows/ci.yml README.md
git commit -m "ci: verify Rust checks"
```

## Recommended first milestone

The most valuable initial milestone is Steps 1–6:

```text
1. Wine-specific CLI
2. Wine data model
3. Configurable data path
4. JSON load/save
5. Persistent add
6. Saved list
```

After Step 6, `whyno` is already a usable wine journal:

```bash
whyno add "Château Example" --vintage 2020 --rating 4 --notes "Dark fruit, smooth finish"
whyno list
```

Everything after that improves maintainability, discoverability, and power-user workflows.
