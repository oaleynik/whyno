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

## Step 7 - Add `show <id>` command
**Status:** Completed
**Commit:** 9f30c18

**Changes made:**
- Added `show` subcommand with `id` argument
- Implemented logic to find and display a specific wine by ID
- Shows all wine details (name, rating, producer, vintage, region, country, grapes, notes, tags)
- Returns error if wine ID not found

**Verification:**
- `cargo fmt --check` - passed
- `cargo check` - passed
- Added wine and verified show displays correctly
- Show with non-existent ID returns error

## Step 8 - Add `remove <id>` command
**Status:** Completed
**Commit:** 9f30c18 (combined with step 7)

**Changes made:**
- Added `remove` subcommand with `id` argument
- Implemented logic to find and remove a wine by ID
- Saves updated collection after removal
- Returns error if wine ID not found

**Verification:**
- `cargo check` - passed
- Successfully removed existing wine
- Remove with non-existent ID returns error

## Step 9 - Add `--tag` support for tagging
**Status:** Completed
**Commit:** ad97319

**Changes made:**
- Added `--tag` argument to `add` command (supports multiple uses and comma-separated)
- Updated `WineInput` struct to include `tags` field
- Updated `Wine::from_input()` to process tags
- Updated `show` command to display tags

**Verification:**
- `cargo fmt --check` - passed (after fmt)
- `cargo check` - passed
- Added wine with multiple tags
- Tags displayed in show command
- Tags persisted in JSON

## Step 10 - Add filtering to `list`
**Status:** Completed
**Commit:** 4a78fc0

**Changes made:**
- Converted `List` subcommand to `List(ListArgs)` struct
- Added filter options: `--tag`, `--grape`, `--country`, `--min-rating`, `--query`
- Implemented case-insensitive filtering logic
- `--query` searches across name, producer, region, country, grapes, notes, and tags
- All filters are composable (can combine multiple)

**Verification:**
- `cargo fmt --check` - passed (after fmt)
- `cargo check` - passed
- Test data: French Merlot (France, Merlot, rating 4) and Italian Red (Italy, Sangiovese, rating 3)
- `--country france` - shows only French wine
- `--grape merlot` - shows only wine with Merlot
- `--min-rating 4` - shows only wine with rating 4+
- `--query italian` - shows wine with "italian" in any field

## Step 11 - Add `update <id>` for correcting records
**Status:** Completed
**Commit:** b6b556f

**Changes made:**
- Added `Update(UpdateArgs)` subcommand with same fields as `add` (except name)
- Only fields provided on command line are updated
- Validates updated record before saving
- Tags replace existing tags (not additive)
- Displays success message on update

**Verification:**
- `cargo fmt --check` - passed
- `cargo check` - passed
- Added wine with rating 3
- Updated to rating 5 with notes - successful
- Show command reflects updates
- Invalid rating update fails without corrupting data

## Step 12 - Improve default data location
**Status:** Completed
**Commit:** 3049220

**Changes made:**
- Added `directories` crate dependency
- Changed `--data` from required with default to optional
- Implemented `get_default_data_path()` function using platform-appropriate directories:
  - macOS: `~/Library/Application Support/whyno/wines.json`
  - Linux: `~/.local/share/whyno/wines.json`
  - Windows: App data directory
- Creates parent directory if it doesn't exist
- Falls back to `./whyno.json` if ProjectDirs fails

**Verification:**
- `cargo test` - passed
- `cargo run -- add "Default Path Wine" --rating 4` - uses platform default
- `cargo run -- list` - displays wine from default location
- `cargo run -- --data ./tmp-wines.json list` - uses custom location

## Step 13 - Add README usage documentation
**Status:** Completed
**Commit:** 627b408

**Changes made:**
- Created comprehensive README.md with:
  - Installation instructions
  - Usage examples for all commands
  - Data storage information
  - Rating scale explanation
  - Data format documentation
  - Command reference with all options
  - Practical examples

**Verification:**
- `cargo fmt --check` - passed
- `cargo test` - passed
- `cargo run -- --help` - displays help
- `cargo run -- add --help` - displays add options
- `cargo run -- list --help` - displays list options

## Step 14 - Add integration tests for CLI workflows
**Status:** Completed
**Commit:** cd47f33

**Changes made:**
- Added dev dependencies: `assert_cmd`, `predicates`, `tempfile`
- Created `tests/cli.rs` with 10 integration tests:
  - test_add_and_list
  - test_add_and_show
  - test_remove_missing_id
  - test_invalid_rating
  - test_corrupt_json
  - test_empty_name
  - test_update_wine
  - test_filter_by_country
  - test_filter_by_min_rating
  - test_tags
- All tests use `--data` with temp files for isolation

**Verification:**
- `cargo test` - all 10 tests pass

## Step 15 - Add quality automation
**Status:** Completed
**Commit:** 2adea24

**Changes made:**
- Created `.github/workflows/ci.yml` with:
  - Triggers on push and pull requests to main/master
  - Runs on Ubuntu with stable Rust
  - Steps: checkout, setup Rust, fmt check, clippy, tests, release build
- Fixed all clippy warnings:
  - Collapsed nested if statements
  - Used `is_some_and()` instead of `map_or(false, ...)`
  - Used range contains instead of manual comparisons
- All checks pass locally

**Verification:**
- `cargo fmt --check` - passed
- `cargo clippy -- -D warnings` - passed
- `cargo test` - passed

## Plan Complete

All 15 steps completed successfully. whyno is a fully functional wine journal CLI tool with:

- Wine tracking (add, list, show, update, remove)
- Tagging and filtering
- Platform-appropriate data storage
- Comprehensive documentation
- Full test coverage
- CI/CD automation

The tool is ready for use and further development.