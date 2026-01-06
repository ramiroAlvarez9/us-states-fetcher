# fetch_counties

Fetches US state (and DC) county FIPS codes from `https://api.fips.codes/index` and writes the result to `counties.json`.

## Requirements

- Rust toolchain (stable)
- Network access (the program calls `https://api.fips.codes/index`)

## Usage

Run:

```bash
cargo run
```

This will:

- Request the full dataset from `https://api.fips.codes/index`
- Transform it into a list of objects with the shape:
  - `uspsAbbreviation` (state USPS abbreviation)
  - `fips` (state FIPS code)
  - `counties` (list of `{ county, fips }`)
- Write the formatted JSON output to `counties.json`

## Output

The output file is written to:

- `counties.json`
# us-states-fetcher
