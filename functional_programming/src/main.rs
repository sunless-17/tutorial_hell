fn higher_order_function<T>(y: T, x: u32) -> u32
where
  T: Fn(u32) -> u32,
{
  y(x + 2)
}

fn main() {
  let z = higher_order_function(|x| x, 2);
  println!("Higher Order! {z}");
}
