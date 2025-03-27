# TODO

As more eyes start seeing this code, I feel like I should spend some time making it understandable and functional. This list should at the very least track the process to making this code understandable, and at the very most give you a super abstract idea of what the code is supposed to do. It is worth noting that although this is the main `NewsNet` branch, it is home to the rewrite branch of the submodules (hopefully. I'm gonna have to restructure after a superficial design rewrite). Also worth pointing out that a rewrite means that i have done some of this stuff already, but it was kinda shit because I didn't really have a plan with the database at the time. Now, I'm being more intentional about making sure that the data is as rich and usable as possible so that: 
1. The frontend doesnt need to fight through abstractions. 
2. The data is represented with the fewest conversion work necessary, as well as making it waaaaaaaaaaaaaaaaaaaaay more readable especially when that data is serialised and used for cool stats stuff
3. Encapsulation and separation of concerns: I dont want to find every single instance of a field type whenever a change is made to the gdelt structures. if i can put them in traits, i can manage their behaviours accordingly. For example: If the country code standard is changed or updates, instead of finding every single time i check `if actor.country == "ZA"` and instead change the value of `"ZA"` in the enum conversion dictionary. also imagine string checking everytime you want to check something. what if i make a typo?
4. Serialisation and interoperability: instead of hoping that a person who sees field names knows exactly what types should be used and are serialised correctly, i can just serialise the richer object and type checking will prevent silly stuff like accidentally using the name variable `name: &str = "name"` in a place where an `id: &str = "2uqpgbr"` should be for a query:
	1. Say i have a function `fn get_record(id: &str) -> Record` and someone accidentally uses `rec = get_record("name")`, type checking would not pick up on it.
	2. If `id: ObjTypeID = ObjTypeID("2uqpgr")`, and `fn get_record(id: ObjecTypeID)-> Record`, `rec = get_record("name")` would be caught


# Data Fetching
GDELT used several ways of distributing data. Unfortunately, this means that we have to parse the various data sources, which would be fine if they weren't so different from each other. JSON.gz, RSS, JSON variants and CSV. I'm defs missing something though. Either way, I needa be careful when creating the in memory representations because they should be easy to change and update as gdelt tends to do that.
- [ ] CSV File fetching is mostly done, i just need to add some logic that manages and switches which file types are downloaded, as well which file date ranges
- [ ] RSS is barely started. I've done enough in the traits to not forget about it in implementation.
- [ ] JSON is not started at all lmao. its like 20 minutes of work only if the rest of the models are foolproof.
- [ ] API from GDELT. Not a huge priority, but would love to use dynamic queries to enrich the data.

# Models
Creation
---
> I'm also trying to separate the concerns of the gdelt objects and their database representations as the conversion to database representations should be done by the database service. styll, i need to make sure that its serialisable and byte friendly ^netabase_separation_of_concerns
- [ ] Create Main Rust objects and their child structs
- [ ] Implement traits and behaviours for the datatypes and their categories (I want to be able to create states that are rich enough to use for cool queries, behaviours and displays)
- [ ] Create the Shallow memory objects for the main record types

Conversion
---

| Progress    | Input                   | Output                  | Depends on                                                                                                                                                                                                                            |
| ----------- | ----------------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Not Started | Raw Files               | Shallow memory object   | Get raw files from all sources                                                                                                                                                                                                        |
| Not Started | Shallow memory Object   | Rust Object             |                                                                                                                                                                                                                                       |
| Not Started | Rust Object             | Database Representation |                                                                                                                                                                                                                                       |
| Not Started | Database Representation | Rust Object             | > As [[#^67eb67\|mentioned]], the actual database types have their own representation (sledb.rs) which simply needs byte friendly representations. this library should not create any types that aren't defined here or are primitive |


