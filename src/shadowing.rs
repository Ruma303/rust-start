fn main() {
    let x = 5;
    println!("Il valore di x è: {}", x); // Stampa 5

    let x = x + 1;
    println!("Il valore di x è: {}", x); // Stampa 6

    {
        let x = x * 2;
        println!("Il valore di x nello scope interno è: {}", x); // Stampa 12
    }

    println!("Il valore di x è: {}", x); // Stampa 6
}

