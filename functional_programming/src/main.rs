fn foo<F, T>(f: F, x: T) -> T
where
  F: Fn(T) -> T,
{
  f(x)
}

fn main() {
  let bar = foo(|8| {}, 8);
  println!("Higher Order! {}", bar);
}
