use crate::{
    components::super_components::geography::Geography, schema::mention_table::CharOffset,
};

use super::{CountValue, ObjectType, gdelt_category::Category};

pub struct Count {
    pub count_type: Category,
    pub count: CountValue,
    pub object_type: ObjectType,
    pub location: Geography,
    pub offset: Option<CharOffset>,
}
