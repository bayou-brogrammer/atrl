use std::ops::{Add, Bound::*, Div, Mul, RangeBounds, Sub};
pub fn map_range<T: Copy>(source_value: T, from_range: (T, T), to_range: (T, T)) -> T
where T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> {
    to_range.0 + (source_value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
pub fn map_range_u32(source_value: u32, from_range: (u32, u32), to_range: (u32, u32)) -> u32 {
    let from_range_x = from_range.0 as i64;
    let from_range_y = from_range.1 as i64;
    let to_range_x = to_range.0 as i64;
    let to_range_y = to_range.1 as i64;
    let source = source_value as i64;
    map_range(
        source,
        (from_range_x, from_range_y),
        (to_range_x, to_range_y),
    ) as u32
}
pub fn get_range_bounds<T: Copy, R: RangeBounds<T>>(
    range: R,
    lower_unbounded: T,
    upper_unbounded: T,
) -> (T, T) {
    let start = match range.start_bound() {
        Excluded(v) => *v,
        _ => lower_unbounded,
    };
    let end = match range.end_bound() {
        Included(v) | Excluded(v) => *v,
        Unbounded => upper_unbounded,
    };
    (start, end)
}
