#[cfg(test)]
mod tests {
    use crate::{have_negative_sides, have_zero_sides, can_construct_triangle};

    #[test]
    fn test_have_negative_sides_function() {
        let test_data = [
            [3.0, 2.0, 3.0],
            [-1.0, 2.0, 0.0],
            [1.0, 1.0, 1.0],
            [5.0, -5.0, 5.0],
            [0.0, 0.0 , -1.0],
        ];

        let expected_data = [false, true, false, true, true];
        let mut func_result = [false; 5];

        let mut i = 0;
        for sides in test_data.iter() {
            func_result[i] = have_negative_sides(sides);
            i += 1;
        }

        assert_eq!(expected_data, func_result);
    }

    #[test]
    fn test_have_zero_sides_function() {
        let test_data = [
            [3.0, 2.0, 3.0],
            [-1.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
            [5.0, -5.0, 0.0],
            [0.0, 0.0 , -1.0],
        ];

        let expected_data = [false, true, false, true, true];
        let mut func_result = [false; 5];

        let mut i = 0;
        for sides in test_data.iter() {
            func_result[i] = have_zero_sides(sides);
            i += 1;
        }

        assert_eq!(expected_data, func_result);
    }

    #[test]
    fn test_can_construct_triangle_function() {
        let test_data: [[f32; 3]; 5] = [
            [3.0, 2.0, 3.0],
            [1.0, 1.0, 3.0],
            [1.0, 1.0, 1.0],
            [5.0, 6.0, 1.0],
            [5.0, 10.0, 13.0],
        ];

        let expected_data = [true, false, true, false, true];
        let mut func_result = [false; 5];

        let mut i = 0;
        for sides in test_data.iter() {
            func_result[i] = can_construct_triangle(&sides[0], &sides[1], &sides[2]);
            i += 1;
        }

        assert_eq!(expected_data, func_result);
    }
}