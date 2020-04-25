#![allow(dead_code)]
use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point // -> Point signifca che il ritorno della funzione è di tipo Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
    let p1 = origin(); //il valore è salvato nello stack come una var qualsiasi
    let p2 = Box::new(origin()); //in questo modo salvo nello stack il nome del puntatore p2 e nello heap vengono salvati i valori. 
    
    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); //p1 occupa 16 bytes di memoria in x64 perchè ne occupa 8 per salvare la x e 8 per la y.
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); //p2 occupa solo 8 bytes di memoria in x64 perchè p2 è un puntatore salvato sullo stack e quindi non vengono visti gli indirizzi salvati nello heap.

    let p3 = *p2; //dereferenzio p2 per accedere al valore salvato nello heap
    println!("{}", p3.x) //valore di x è 0
}