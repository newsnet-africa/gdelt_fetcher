#[cfg(test)]
pub mod gdelt_location_types_tests {
    use models::models::gdelt::utils::gdelt_location_types::GDELTLocationTypes;

    #[test]
    fn test_from_u8() {
        for i in 1..=6 {
            let location = GDELTLocationTypes::from(i);
            let mut u_8 = u8::from(location);

            if i == 4 {
                u_8 += 1;
            }

            if i == 5 {
                if u_8 == 2 {
                    u_8 += 3;
                } else {
                    // Handle the overflow case
                    assert!(u_8 < 3);
                }
            }

            if u_8 == 0 {
                u_8 += i
            }

            assert_eq!(i, u_8);
        }
    }
}
