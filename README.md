# NewsNet GDELT Data fetcher

This crate is expressly designed for `NewsNet` usage and implementation.
As well as being incomplete, the models and process flows are either intended to be refactored or completely removed.
There is no plan to make an API, (for now) as some of the details of `NewsNet`'s implementation might evolve and require many changes to how GDELT data is fetched and handled, but there are functional goals I have in mind for this crate, in case you would like to use it for your own super specific thing:

## Current functionality:
- Fetch data using GDELT API:
    - Currently only fetching `Event`, `Mention` and `Global Knowledge Graph` data from GDELT, more tables are probably going to be supported
        - Latest available batch
        - Specific timestamp
    - Checks file hash and unzips file
    - Parses tables into respective `Event`, `Mention` and `GlobalKnowledgeGraph` structs
        - **These are intended for use with `NewsNet` and implementations, traits etc might be specific for that use case. This is currently not a problem, but they are going to need to interact with the `netabase` crate and dependencies or quirks might get weird in the future**
        - Rich representations of field types with separate structs and NewTypes

## TODO:
- [ ] Implement remaining tables
- [ ] Fix the super weird hack used to index the GCAM codebook.
    - In memory Enum? On demand kv? Not sure yet.
- [ ] Abstract parser by input type (CSV, JSON etc.)
- [ ] Clean up download and valitation logic
- [ ] Create BigQuery implementation for larger loads
- [ ] Create a more consolodated API
- [ ] Create binary and *maybe* python port for data research
