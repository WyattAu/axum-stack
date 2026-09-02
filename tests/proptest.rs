use axum_stack::request_id::request_id_layer;
use proptest::prelude::*;

proptest! {
    #[test]
    fn request_id_layer_builds_with_any_params(_dummy in 0..1u32) {
        let (set, propagate) = request_id_layer();
        drop(set);
        drop(propagate);
    }

    #[test]
    fn request_id_layer_builds_multiple_times(n in 1..100usize) {
        for _ in 0..n {
            let (set, propagate) = request_id_layer();
            drop(set);
            drop(propagate);
        }
    }
}
