// TODO: I Actually don't have anything to do for now, just take note of the parsers and make sure they align with the correct Item codes. CAMEO and FIPS.
//
// TODO: Administrative regions. There are hella. like a lot. I'm still considering just keeping these as strings, but obviously it might be better to create the enums for each
// TODO: One of these Enums (I think its the Themes?) if fucking long and I havent created the enum for it because it's impractical otherwise. However, we could:
//       a. Store it as an enum
//       b. store it as a persistent dictionary file and write a reader for the dictionary instead

pub mod actor_type;
pub mod country;
pub mod ethnicity;
pub mod event_action_description;
pub mod geography_type;
pub mod known_group;
pub mod mention_type;
pub mod quad_class;
pub mod religion;
pub mod role;
pub mod social_embeds;
