mod stack_heap; //importo il file stack_heap.rs come se fosse un header del C
use std::mem;


const CIAO:u8 = 42; //"let" non funziona per le variabili globali. La var globale non ha un indirizzo fisso 

static mut NOT_SAFE:u16 = 123; //con static è possibile usare "mut" per poter cambiare il valore della variabile, ma per modificare il valore bisogna scriverte codice in un blocco chiamato "unsafe"

fn prova_var_globale()
{
    unsafe
    {
        NOT_SAFE = 456;
        println!("global NOT_SAFE: {}", NOT_SAFE);
    }
}

fn main() {
    let a:u8 = 123;
    println!("a = {}", a);

    //senza mut, non è possibile cambiare il valore della variabile.
    let mut b:i8 = 0;
    b = 42;
    println!("b = {}", b);

    let c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z:isize = 123; // isize/usize utile perchè mette in automatico la dimensione relativa all'architettura dell'OS
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d = 'x'; //senza specificare la dimensione, la size è 4 bytes e non 1 perchè sceglie la grandezza dell'insieme UNICODE.
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; //f64 tipo di default
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let f = false; //bool true/false 1 byte
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    //a++ --> ERRORE, non esistono ++, --
    //a+=1 --> GIUSTO

    let b_cubed = i32::pow(b as i32, 3); //b as i32 --> cast a i32 (altri tipi danno errore perchè si aspetta esattamente una variabile i32).
    println!("{} cubed is {}", b, b_cubed);

    let g = 1|2; // OR --> 01 || 10 == 11 == 3_10
    println!("1|2 = {}", g);

    let h = 1 << 10; // bit shift che corrisponde a 2^10
    println!("2^10 = {}", h);

    println!("const global var = {}", CIAO);

    prova_var_globale();

    stack_heap::stack_and_heap();
}
