use sysinfo::{ProcessesToUpdate, System};
use std::thread;
use std::time::Duration;

fn main() {
	// On crée un objet "système"
	let mut sys = System::new();
    
    // Photo A (Initialisation des compteurs)
    sys.refresh_processes(ProcessesToUpdate::All, true);

    // On met notre programme en pause pendant 200ms
    thread::sleep(Duration::from_millis(200));

	// Photo B (Calcul du delta CPU)
    sys.refresh_processes(ProcessesToUpdate::All, true);
    
    // On dessine l'en-tête UNE SEULE FOIS, AVANT la boucle
    println!("{:<14} | {:>9} | {:>13} | {}", "PID", "CPU (%)", "RAM (Mo)", "NOM DU PROCESSUS");
    println!("{:-<14}-+-{:->9}-+-{:->13}-+-{:-<40}", "", "", "", ""); // Ligne de séparation
    
    for (pid, process) in sys.processes() {
        // On récupère les données d'utilisation RAM & CPU
        let memory_mb = process.memory() / 1_048_576;
        let cpu = process.cpu_usage();
        
        // On affiche uniquement les données dans la boucle
        println!("PID : {:>8} | {:>7.2} % | {:>10} Mo | {:?}", pid.as_u32(), cpu, memory_mb, process.name());
    }
}