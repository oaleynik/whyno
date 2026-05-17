# whyno

A simple CLI tool for tracking wines you've tried.

## Installation

```bash
cargo install --path .
```

Or build and run directly:

```bash
cargo run -- <command> [args]
```

## Usage

### Add a wine

```bash
whyno add "Château Margaux" \
  --vintage 2018 \
  --producer "Château Margaux" \
  --region "Margaux" \
  --country "France" \
  --grape Cabernet \
  --rating 5 \
  --notes "Elegant, full-bodied with dark fruit" \
  --tag favorite,cellar
```

### List all wines

```bash
whyno list
```

### Filter wines

```bash
# Filter by country
whyno list --country france

# Filter by grape
whyno list --grape merlot

# Filter by minimum rating
whyno list --min-rating 4

# Search across all fields
whyno list --query rioja

# Filter by tag
whyno list --tag favorite
```

### Show a specific wine

```bash
whyno show 1
```

### Update a wine

```bash
whyno update 1 --rating 5
whyno update 1 --notes "Better on day two"
whyno update 1 --tag special,aged
```

### Remove a wine

```bash
whyno remove 1
```

### Use a custom data file

```bash
whyno --data ./my-wines.json add "Test Wine" --rating 4
whyno --data ./my-wines.json list
```

## Data Storage

By default, whyno stores wine data in a platform-appropriate location:

- **macOS**: `~/Library/Application Support/whyno/whyno/wines.json`
- **Linux**: `~/.local/share/whyno/whyno/wines.json`
- **Windows**: `%APPDATA%\whyno\whyno\wines.json`

Use the `--data` flag to specify a custom location.

## Rating Scale

Ratings are on a 1-5 scale:

- 1: Poor
- 2: Below average
- 3: Average
- 4: Good
- 5: Excellent

## Data Format

Wine data is stored as JSON. You can manually edit the file if needed:

```json
[
  {
    "id": 1,
    "name": "Château Margaux",
    "producer": "Château Margaux",
    "vintage": 2018,
    "region": "Margaux",
    "country": "France",
    "grapes": ["Cabernet"],
    "rating": 5,
    "notes": "Elegant, full-bodied with dark fruit",
    "tags": ["favorite", "cellar"]
  }
]
```

## Commands

- `add <name>` - Add a new wine
- `list` - List all wines (with optional filters)
- `show <id>` - Show details of a specific wine
- `update <id>` - Update a wine's details
- `remove <id>` - Remove a wine
- `--data <path>` - Use a custom data file (global option)

### Options for `add` and `update`

- `-v, --vintage <year>` - Wine vintage
- `-p, --producer <name>` - Producer/winery name
- `--region <name>` - Region
- `-c, --country <name>` - Country
- `-g, --grape <name>` - Grape variety
- `-r, --rating <1-5>` - Rating (1-5)
- `-n, --notes <text>` - Tasting notes
- `-t, --tag <tag>` - Tags (can be used multiple times or comma-separated)

### Options for `list`

- `--tag <tag>` - Filter by tag
- `--grape <name>` - Filter by grape variety
- `--country <name>` - Filter by country
- `--min-rating <1-5>` - Filter by minimum rating
- `--query <text>` - Search across all fields

## Examples

```bash
# Add a quick wine with just name and rating
whyno add "Everyday Red" --rating 3

# Add a detailed wine
whyno add "Opus One" \
  --vintage 2015 \
  --producer "Opus One" \
  --region "Napa Valley" \
  --country "USA" \
  --grape Cabernet \
  --rating 5 \
  --notes "Complex, layered, excellent balance" \
  --tag special,birthday

# Find all your favorite wines
whyno list --tag favorite

# Find all highly-rated wines
whyno list --min-rating 4

# Search for wines from a specific region
whyno list --query napa

# Update a wine with more notes
whyno update 1 --notes "Even better after decanting for 2 hours"
```