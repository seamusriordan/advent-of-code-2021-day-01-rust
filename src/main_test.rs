#[cfg(test)]
mod tests {
    use crate::{count_increases, count_increases_for_window};

    #[test]
    fn empty_vector_gives_0() {
        assert_eq!(0, count_increases(&vec![]));
    }

    #[test]
    fn example_vector_gives_7_increases() {
        assert_eq!(7, count_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
    }

    #[test]
    fn example_vector_gives_5_windowed_increases() {
        assert_eq!(5, count_increases_for_window(
            &vec![607, 618, 618, 617, 647, 716, 769, 792]
        ));
    }
}