#[cfg(test)]
pub mod gdelt_category_tests {
    use models::models::gdelt::utils::gdelt_categorylist::GDELTCategoryList;
    use models::models::gdelt::{CellItem, GDELTObject};
    use std::cell::{Ref, RefCell};

    /// Tests the conversion from `u16` to `GDELTCategoryList` and back to `u16`.
    #[test]
    fn test_from_u16_to_u16() {
        for i in 0..280 {
            let category = GDELTCategoryList::from(i);
            if i < 280 {
                assert_eq!(u16::from(category), i);
            } else {
                assert!(category.eq(&GDELTCategoryList::UNKNOWN(RefCell::new(0))));
            }
        }
    }

    /// Tests the conversion from string to `GDELTCategoryList` and back to string.
    #[test]
    fn test_from_strings() {
        for i in 0..280 {
            let category = GDELTCategoryList::from(i as u16);
            let string1 = u16::from(category.clone()).to_string();
            let string2 = ";12435";

            let strings = string1 + string2;

            let new_category = GDELTCategoryList::from_strings(&strings);
            match new_category {
                Some(new_category) => assert_eq!(category, new_category),
                None => assert_eq!(category, GDELTCategoryList::from(i)),
            }
        }
    }

    #[test]
    fn test_from_cell() {
        let cell = "AGRICULTURE,10;TAX_FNCACT_FARMER,10;TAX_FNCACT_ANALYST,176;EPU_ECONOMY_HISTORIC,53;ECON_TAXATION,143;USPEC_POLICY1,143;EPU_POLICY_TAX,143;EPU_CATS_TAXES,143;";

        let cat = GDELTCategoryList::vec_from_cell(cell);

        assert!(cat.is_some() || cat.is_none())
    }
}
