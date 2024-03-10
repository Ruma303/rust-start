fn main() {
    //%  Variabili

    //, Dichiarazione di variabili
    let x = 10; // Implicitamente tipizzata
    let y: i64 = 50; // Esplicitamente tipizzata

    //x = 20; //. Errore


    //, Mutabilità e riassegnazione
    /* let mut y = 10;
    println!("Il valore di y è: {y}");
    y = 20;
    println!("Il valore di y è: {y}"); */

    /* let a = 10;
    println!("Il valore di a è: {}", a);
    let a = 20;
    println!("Il valore di a è: {}", a); */


    //, Assegnazione multipla
    // let (a, b, name) = (10, 20.99, "Giovanni");

    
    //% Costanti
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);
}
