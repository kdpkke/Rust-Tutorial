mod stack_heap; //importo il file stack_heap.rs come se fosse un header del C
use std::mem;


const CIAO:u8 = 42; //"let" non funziona per le variabili globali. La var globale non ha un indirizzo fisso 

static mut NOT_SAFE:u16 = 123; //con static è possibile usare "mut" per poter cambiare il valore della variabile, ma per modificare il valore bisogna scriverte codice in un blocco chiamato "unsafe"

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8}
}

fn enums()
{
    let c:Color = Color::Cmyk{cyan: 0, magenta: 128, yellow: 0, black: 0};
    
    match c
    {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RgbColor(0,0,0) | Color::Cmyk{cyan:_, magenta:_, yellow:_, black:255} => println!("black"), //si possono mettere in OR le due condizioni e l'underscore indica che ci può essere qualsiasi tipo di valore in quell'oggetto della struct. 
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r, g, b),
                                                                                                         
        _ => println!("invalid")
    };
}

fn prova_var_globale()
{
    unsafe
    {
        NOT_SAFE = 456;
        println!("global NOT_SAFE: {}", NOT_SAFE);
    }
}

fn option()
{
    let x = 3.0;
    let y = 2.0;

    let result:Option<f64> = if y != 0.0 {Some(x/y)} else { None };

    match result
    {
        Some(z) => println!("{}/{} = {}", x,y,z),
        None => println!("cannot divide {} by {}", x,y)
    };

    //fare match result come fatto sopra e fare if let...else come fatto sotto è la stessa cosa. La differenza è che nel match devi obbligatoriamente
    //includere tutti i possibili risultati, mentre con if let NO.
    if let Some(z) = result { println!("{}/{} = {}", x,y,z) } //questo println viene eseguito solo se result ha "valore" Some. Se result ha valore "None", non viene eseguita la println.
    else { println!("cannot divide {} by {}", x,y) }; 
}

fn arrays()
{
    let mut arr:[i32;5] = [1,2,3,4,5];
    let mut arr2 = [2,3,4,5,6];

    println!("arr has {} elements, first is {}", arr.len(), arr[0]);
    println!("{:?}", arr); //stampa tutto l'array

    
    if arr2 == [1,2,3,4,5]
    {
        println!("array is equal to the other one.");
    }

    let arr3 = [1u16; 10]; //riempie un array grande 10 con dieci "1" u16.

    println!("arr3 takes up {} bytes", mem::size_of_val(&arr3));

    let mtx:[[f32;3]; 2] = //3 sono le colonne, 2 le righe.
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);
}

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);

    match a.get(6) //get ritorna un Option così da essere memory safe nel caso in cui l'index sia out of range.
    {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element")
    };

    for x in &a { println!("{}", x);} //serve la & per accedere a tutti gli element del vector.

    while let Some(x) = a.pop() //stampa tutto il vettore al contrario togliendo nel frattempo tutti i valori al suo interno.
    {
        println!("{}",x); 
    }
}

fn use_slice(slice: &mut[i32])
{
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321
}
fn slices()
{
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]); //gli passo solo gli elementi in mezzo dell'array
    use_slice(&mut data); //gli passo tutto l'array

    println!("{:?}", data);
}

fn strings()
{
    let s:&'static str = "hello there!"; // &str = string slice. Si alloca nello Heap.
    //s = "abc"; ERRORE
    //let h = s[0]; ERRORE

    for c in s.chars()
    {
        println!("{}", c); // stampa tutti i caratteri 
    }

    for c in s.chars().rev()
    {
        println!("{}", c); // stampa tutti i caratteri in reverse
    }

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is: {}", first_char)
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    //concatenation String + str è possibile. non è possibile concatenare String + String.
    //let z = letters + &letters --> non funziona
    //&str <> string clone, se prendo &letters direttamente mi da errore alla concatenazione (come se facendo &letters venisse modificata la variabile letters in tutte le sue chiamate)
    let u:&str = &letters.clone();
    let z = letters + u; //funziona
    println!("concatenation of the same string {}", z);
}

fn tuples()
{
    let sp = (4, true); //i tipi all'interno dei tuples può essere diverso.
    println!("First elem: {}, Scond elem: {}", sp.0, sp.1);
    //salvo in a e b rispettivamente gli elementi 0, 1
    let (a, b) = sp;
    println!("First elem: {}, Scond elem: {}", a, b);

    //tuple con un solo elemento
    let meaning = (42,);
    println!("tuple single element: {:?}", meaning);
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
    
    for x in 1..11 //da 1 a 10. 11 NON è incluso 
    {
        println!("x = {}", x);
    }    

    for (pos,val) in (30..41).enumerate() //questo for va da 0 a 10 per "pos" e va da 30 a 40 per "val"
    {
        println!("pos: {}, y: {}", pos, val);
    }

    let country_code = 77;

    let country = match country_code //è come fare switch..case inline.
    {
        44 => "UK",
        77 => "russia",
        1..=999 => "unknown", //in questo caso, a differenza del range del for, ..= tra i due numeri indica un range inclusivo da 1 a 999 (compreso)  
        _ => "invalid" 
    };

    println!("the country with code {} is {}", country_code, country);

    enums();
    option();
    arrays();
    vectors();
    slices();
    strings();
    tuples();
}
