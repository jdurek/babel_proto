// Unit tests for various functions related to the map_data struct, mostly to validate everything behaves as expected. 

// Also mimics some functionality of the minimap editor (scale/center shifts) on some tests.

pub use babel_proto::data_structs::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tile_index() {
        // Given a few maps of varying sizes, correctly determine the index of a given coordinate
        let test_map = MapBase::new(3,3);
        assert_eq!(test_map.get_tile_index(0, 0), 0);
        assert_eq!(test_map.get_tile_index(2, 2), 8);
        assert_eq!(test_map.get_tile_index(1, 0), 1);
        assert_eq!(test_map.get_tile_index(0, 1), 3);

        // Assert out of bounds - waiting for the function to be adjusted to account for it (Or just have it return -1)
        assert_eq!(test_map.get_tile_index(4, 1), -1);
        
        


        let test_map = MapBase::new(2,5);
        assert_eq!(test_map.get_tile_index(1, 0), 1);
        assert_eq!(test_map.get_tile_index(0, 1), 2);
        
        assert_eq!(test_map.get_tile_index(2, 2), -1);


        let test_map = MapBase::new(5,2);
        assert_eq!(test_map.get_tile_index(0, 1), 5);
        assert_eq!(test_map.get_tile_index(2, 2), -1);
    }


    #[test]
    fn test_wall_index() {
        // TODO - Implement tests for this - basically come up with a few where I know the intended outcome and see how the function handles.
        // Given a few maps of varying sizes, correctly determine the wall index of a given line
        let test_map = MapBase::new(3,3);
        assert_eq!(test_map.get_wall_from_line(0, 0, 0, 1), Ok(3));
        assert_eq!(test_map.get_wall_from_line(0, 0, 1, 0), Ok(0));

        assert_eq!(test_map.get_wall_from_line(2, 2, 2, 1), Ok(12));
        assert_eq!(test_map.get_wall_from_line(2, 2, 2, 3), Ok(19));
        assert_eq!(test_map.get_wall_from_line(3, 2, 3, 1), Ok(13));
        assert_eq!(test_map.get_wall_from_line(3, 3, 2, 3), Ok(23));
        
        assert!(test_map.get_wall_from_line(5, 0, 5, 1).is_err());
        assert!(test_map.get_wall_from_line(0, 0, 1, 1).is_err());


        let test_map = MapBase::new(2,5);
        assert_eq!(test_map.get_wall_from_line(0, 0, 1, 0), Ok(0));
        assert_eq!(test_map.get_wall_from_line(0, 0, 0, 1), Ok(2));

        assert_eq!(test_map.get_wall_from_line(0, 5, 1, 5), Ok(25));
        assert_eq!(test_map.get_wall_from_line(2, 0, 2, 1), Ok(4));

        assert!(test_map.get_wall_from_line(3, 0, 3, 1).is_err());


        let test_map = MapBase::new(5,2);
        assert_eq!(test_map.get_wall_from_line(0, 0, 1, 0), Ok(0));
        assert_eq!(test_map.get_wall_from_line(0, 0, 0, 1), Ok(5));
    }


    
}