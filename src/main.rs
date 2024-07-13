use std::time;

fn vec_allocation(reserved: bool) {
  let mut vec: Vec<String>;
  let num: usize = 256;
  if reserved {
    vec = Vec::with_capacity(num);
  } else {
    vec = Vec::new();
  }
  for _i in 0..num {
    vec.push(_i.to_string());
  }
  return;
}

fn main() {
  let test_times: i32 = 50;
  let start: time::Instant = time::Instant::now();
  for _i in 0..test_times {
    vec_allocation(true);
  }
  let elapsed_time: f64 = start.elapsed().as_secs_f64();
  let formated_time: String = format!("{:.9}", elapsed_time);
  println!("Total time spent: {} s", formated_time);

  let start: time::Instant = time::Instant::now();
  for _i in 0..test_times {
    vec_allocation(false);
  }
  let elapsed_time: f64 = start.elapsed().as_secs_f64();
  let formated_time: String = format!("{:.9}", elapsed_time);
  println!("Total time spent: {} s", formated_time);

}