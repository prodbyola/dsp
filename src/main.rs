use ch2::running_statistics::{arithmetic_mean, standard_deviation, RS};

mod ch2;
mod algos;

fn main() {
    let data: Vec<u8> = vec![100, 12, 34, 73];

    arithmetic_mean(&data);
    standard_deviation(&data);

    let mut rs = RS::new(data, true);
    rs.add_samples(vec![70, 22, 70, 35, 62], true);
    rs.add_sample(240, true);
}