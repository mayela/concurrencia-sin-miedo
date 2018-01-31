fn sum(x: usize, y: usize) -> usize {
  x.num + y.num
}

fn main() {
  let x = Tipo {num: 3};
  let y = Tipo {num: 2};
  let s = sum(x, y);
  
  let c = sum(x, y);
  
  println!("{} {}", s, c);
}
