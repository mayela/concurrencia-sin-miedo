#[derive(Clone)]
struct Tipo {
    num: usize
}

fn sum(x: Tipo, y: Tipo) -> usize {
  x.num + y.num
}

fn main() {
  let x = Tipo {num: 3};
  let y = Tipo {num: 2};
  let s = sum(x, y);
  
  let c = sum(x, y);
  
  println!("{} {}", s, c);
}
