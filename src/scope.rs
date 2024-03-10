fn main() {
    let x = 10; //*  x è accessibile da qui...

    {
        let y = 5; //*  y è accessibile solo all'interno di questo blocco
        println!("Il valore di x è {} e il valore di y è {}", x, y);
    }   //! y non è più accessibile da qui

    // println!("{}", y); //! Questo causerebbe un errore di compilazione perché y non è più nello scope

} //! x non è più accessibile da qui


