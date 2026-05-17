# Progress Log

## Step 1 - Define wine tracking CLI
**Status:** Completed
**Commit:** eb0daed

**Changes made:**
- Renamed domain from "thing" to "wine"
- Added wine-specific CLI arguments to `add` command:
  - `name` (required)
  - `--vintage` (optional)
  - `--producer` (optional)
  - `--region` (optional, no short flag to avoid conflict with --rating)
  - `--country` (optional)
  - `--grape` (optional)
  - `--rating` (optional)
  - `--notes` (optional)
- Updated CLI output to print wine details

**Verification:**
- `cargo fmt --check` - passed
- `cargo check` - passed
- `cargo run -- add "Test Wine" --rating 4 --notes "Nice"` - outputs correctly
- `cargo run -- list` - outputs correctly

**Note:** Fixed short flag conflict between `--region` and `--rating` by removing short flag from region.

## Step 2 - Add wine data model
**Status:** Completed
**Commit:** f39ac2d

**Changes made:**
- Created `src/wine.rs` with `Wine` struct and `WineInput` struct
- Added serde serialization/deserialization support
- Implemented validation:
  - Non-empty name (trimmed)
  - Rating must be between 1-5
  - Vintage must be between 1900-2100
- Updated main.rs to use validation

**Verification:**
- `cargo fmt --check` - passed (after fmt)
- `cargo check` - passed
- `cargo test` - passed
- `cargo run -- add "Test Wine" --rating 4` - validates successfully
- `cargo run -- add "" --rating 4` - fails with "Wine name cannot be empty"
- `cargo run -- add "Bad Rating" --rating 99` - fails with "Rating must be between 1 and 5"

## Step 3 - Add configurable data path
**Status:** Completed
**Commit:** b7dfec9

**Changes made:**
- Added `--data` global CLI option with default value `./whyno.json`
- Updated CLI to display the data file being used

**Verification:**
- `cargo fmt --check` - passed (after fmt)
- `cargo check` - passed
- `cargo run -- --data ./tmp-wines.json add "Test Wine" --rating 4` - displays correct data path
- `cargo run -- --data ./tmp-wines.json list` - displays correct data path

## Step 4 - Implement JSON loading and saving
**Status:** Completed
**Commit:** 2f5a269

**Changes made:**
- Created `src/storage.rs` with `load_wines()` and `save_wines()` functions
- `load_wines()`:
  - Returns empty Vec for missing files
  - Returns helpful error for invalid JSON
  - Returns empty Vec for empty files
- `save_wines()`:
  - Creates parent directories if needed
  - Writes pretty-printed JSON

**Verification:**
- `cargo fmt --check` - passed (after fmt)
- `cargo check` - passed (with warnings for unused save_wines, expected)
- `cargo test` - passed
- `cargo run -- --data ./tmp-wines.json list` - loads 0 wines for missing file
- `printf 'not json\n' > ./tmp-wines.json && cargo run -- --data ./tmp-wines.json list` - fails with helpful error

## Step 5 - Make `add` persist wine records
**Status:** Completed
**Commit:** da90ab7

**Changes made:**
- Updated `add` command to:
  - Load existing wines
  - Calculate next ID (max existing + 1)
  - Validate input and create Wine
  - Append to collection
  - Save to file
  - Display success message

**Verification:**
- `cargo test` - passed
- `cargo run -- --data ./tmp-wines.json add "Test Wine" --rating 4 --notes "Good"` - adds successfully
- `cat ./tmp-wines.json` - contains valid JSON with wine record (ID: 1)
- `cargo run -- --data ./tmp-wines.json add "Second Wine" --rating 5` - adds second wine (ID: 2)
- `cat ./tmp-wines.json` - contains both wines with unique IDs

## Step 6 - Make `list` display saved wines
**Status:** Completed
**Commit:** 71b8e2b

**Changes made:**
- Updated `list` command to:
  - Load wines from data file
  - Display empty state message if no wines
  - Display formatted wine list with:
    - ID, name, rating
    - Producer (if present)
    - Vintage (if present)
    - Region and country (combined if both present)
    - Grapes (if present)
    - Notes (if present)

**Verification:**
- `cargo test` - passed
- `cargo run -- --data ./tmp-wines.json list` - displays "No wines found" for empty file
- `cargo run -- --data ./tmp-wines.json add "Test Wine" --rating 4 --region "Rioja"` - adds successfully
- `cargo run -- --data ./tmp-wines.json list` - displays formatted wine with region

**Milestone Complete:** Steps 1-6 complete. whyno is now a usable wine journal.