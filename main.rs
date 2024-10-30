use std::io;

fn main() {
    let mut is_running = true;

    let mut make = false;

    println!("V 0.1");
    while is_running {
        println!();
        if make != true {
            println!("please input your action: ");
        } else {
            println!("");
        }

        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("fial");

        let input = input.trim();

        // actions
        if input == "astro" {
            println!("OUTPUT::");
            println!("Could find commando after astro error: 11");

        } else if input == "exit" {
            is_running = false;

        } else if input == "astro update" {
            println!("OUTPUT::");
            println!("System is up to date in 0.0078862628226 seconds");

        } else if input == "astro install make" {
            println!("OUTPUT::");
            println!("Make install done.");
            make = true;






            // make programeer taal
        } else if input == "make" && make == true {
            println!("OUTPUT::");
            println!("Make uitgevoerd.");
        } else if input.starts_with("print ") {
            let message = input.trim_start_matches("echoln ");
            println!("OUTPUT::");
            println!("{}", message);
        } else if input == "help" {
            println!("Beschikbare commando's:");
            println!("astro - Basis astro commando");
            println!("exit - Programma afsluiten");
            println!("astro update - Systeem updaten");
            println!("astro install make - Make installeren");
            println!("make - Make uitvoeren (indien geïnstalleerd)");
            println!("print [bericht] - Bericht afdrukken");
            println!("help - Deze hulp weergeven");
        }
    

    

    }



}