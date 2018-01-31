# Introducción a los patrones de concurrencia en Rust

## ¿Cómo instalar Rust?

[Enlace a la pagina](https://www.rust-lang.org/en-US/install.html)

### Cargo

Iniciar proyecto:

```
$ cargo new <nombre_del_proyecto> --bin
```

Compilar y correr:

```
$ cargo build
```

## Por qué es difícil hacer concurrencia en otros lenguajes

### Data races(Condiciones de carrera)

Son situaciones en las que dos o más procesos leen o escriben en un área 
compartida y el resultado final depende de los instantes de ejecución de 
cada uno.

### _Dangling pointers_ y _NullPointerException_

Si un dato es destruido antes de que todas las referencias 
(los apuntadores) al mismo sean destruidos, cualquier 
referencia restante se llama referencia colgante.

## ¿Cómo soluciona Rust estos problemas?

### Mecanismo de pertenencia

```rust
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
```

```rust
fn sum(x: usize, y: usize) -> usize {
  x.num + y.num
}

fn main() {
  let x = 3;
  let y = 2;
  let s = sum(x, y);

  let c = sum(x, y);

  println!("{} {}", s, c);
}
```

### ¿Cómo funcionan los hilos en Rust?

```rust
use std::thread;
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
```

### Paso de mensajes

```rust
use std::thread;
use std::sync::mpsc;


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || { tx.send(42).unwrap(); });

    println!("got {}", rx.recv().unwrap());
}
```

### Facilidades de sincronización

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn producer(id: u64, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        let mut i = 0;
        loop {
            i += 1;
            println!("producer {} sending: {}", id, i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(350 * id));
        }
    });
}

fn consumer(rx: Arc<Mutex<mpsc::Receiver<usize>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for v in rx.lock().unwrap().iter() {
            println!("consumer got: {}", v);
        }
    })
}

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..5 {
        producer(i, tx.clone());
    }

    consumer(Arc::new(Mutex::new(rx))).join();
}
```
