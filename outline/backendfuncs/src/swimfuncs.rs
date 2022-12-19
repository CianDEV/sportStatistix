#[allow(dead_code)]
#[allow(unused_assignments)]

pub fn calc_difference(mut old_time: f64, mut new_time: f64) -> f64 {
    old_time = old_time * 1000.0;
    new_time = new_time * 1000.0;

    let percentage_diff = (old_time / new_time) * 100.0;

    return percentage_diff;
}
