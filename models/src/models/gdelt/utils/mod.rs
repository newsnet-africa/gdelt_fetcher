/// The `gdelt_actor` module contains definitions and functionalities related to GDELT actors.
/// Actors in GDELT are entities such as individuals, organizations, or countries that are involved
/// in events. This module likely includes structures and functions to parse, store, and manipulate
/// actor data from GDELT datasets.
pub mod gdelt_actor;

/// The `gdelt_categorylist` module is responsible for handling the category lists used in GDELT data.
/// Categories in GDELT classify events into different types or groups. This module likely includes
/// structures and functions to parse, store, and manage these categories.
pub mod gdelt_categorylist;

/// The `gdelt_counts` module deals with the counts of various elements in GDELT data.
/// This could include counts of events, mentions, or other entities. This module likely includes
/// structures and functions to parse, store, and manipulate count data from GDELT datasets.
pub mod gdelt_counts;

/// The `gdelt_date` module handles date-related functionalities for GDELT data.
/// This includes parsing dates from GDELT datasets, formatting dates, and performing date-related
/// calculations. This module likely includes structures and functions to manage dates effectively.
pub mod gdelt_date;

/// The `gdelt_event_action` module is responsible for handling event actions in GDELT data.
/// Event actions describe what happened during an event, such as protests, meetings, or conflicts.
/// This module likely includes structures and functions to parse, store, and manipulate event action data.
pub mod gdelt_event_action;

/// The `gdelt_location` module deals with location data in GDELT datasets.
/// Locations in GDELT refer to the geographical places where events occur. This module likely includes
/// structures and functions to parse, store, and manage location data.
pub mod gdelt_location;

/// The `gdelt_location_types` module handles different types of locations in GDELT data.
/// This could include types such as cities, countries, or regions. This module likely includes
/// structures and functions to parse, store, and manage location type data.
pub mod gdelt_location_types;

/// The `gdelt_quotation` module is responsible for handling quotations in GDELT data.
/// Quotations refer to direct quotes from individuals or organizations involved in events.
/// This module likely includes structures and functions to parse, store, and manipulate quotation data.
pub mod gdelt_quotation;

/// The `gdelt_source_collection_identifier` module deals with source collection identifiers in GDELT data.
/// These identifiers are used to track the sources of information in GDELT datasets. This module likely includes
/// structures and functions to parse, store, and manage source collection identifier data.
pub mod gdelt_source_collection_identifier;

/// The `gdelt_tone` module handles tone analysis in GDELT data.
/// Tone refers to the sentiment or emotional tone of events, such as positive, negative, or neutral.
/// This module likely includes structures and functions to parse, store, and analyze tone data from GDELT datasets.
pub mod gdelt_tone;
