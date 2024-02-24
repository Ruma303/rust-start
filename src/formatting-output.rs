fn main() {

    //, Macro di formattazione
    /* let world = "mondo";
    let formatted = format!("Ciao, {}", world);
    print!("{}", formatted); */


    /* print!("Testo");
    print!(" sulla");
    print!(" stessa");
    print!(" riga."); */


    /* println!("Prima riga");
    println!("Seconda riga");
    println!("Terza riga"); */

    /* let not_found = String::from("404");
    eprint!("Errore {}!", not_found); */

    /* let internal_error = String::from("500");
    eprintln!("Errore {}!", internal_error); */


    //, Formattazione output

    //# Formattazione di base + placeholders
    /* let name = "Ruma";
    let age = 30;
    println!("Mi chiamo {}, ho {} {}!", name, age, "anni"); */
    // Mi chiamo Ruma e ho 30 anni


    //# Ordine di interpolazione
    /* let first = "Dave";
    let second = "Mary";
    let third = "Ramon";
    println!("Classificati: 1.{2} 2.{1} 3.{0}", first, second, third); */
    // Classificati: 1.Ramon 2.Mary 3.Dave

    // println!("Classificati: 1.{1} 2.{1} 3.{0} 4.{0}", first, second);
    // Classificati: 1.Mary 2.Mary 3.Dave 4.Dave


    //# Named arguments
    // println!("Lavoro come {job1} e {job2}", job1 = "developer", job2 = "insegnante di programmazione");
    // Lavoro come developer e insegnante di programmazione


    //# Esadecimali
    // println!("Red: #{:x}{:x}{:x}00", 255, 0, 0); // Red: #ff0000
    // println!("Blue: #00{:X}{:X}{:X}", 0, 0, 255); // Blue: #0000FF


    //# Binario
    // println!("{0} in formato binario è: {:b}", 10);
    // 10 in formato binario è: 1010


    //# Ottale
    // println!("{0} in formato ottale è: {:o}", 10);
    // 10 in formato ottale è: 12


    //# Floating Points
    // println!("{0} in notazione scientifica vale: {:e}", 999999999);
    // 999999999 in notazione scientifica vale: 9.99999999e8

    // println!("{0} in notazione scientifica vale: {:E}", 999999999);
    // 999999999 in notazione scientifica vale: 9.99999999E8

    




    //# Tuple
    // println!("Status: {:?}", ("User1", 22, true));
    // Status: ("User1", 22, true)
}

