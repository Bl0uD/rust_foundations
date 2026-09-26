use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let db: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
    
    // 1. On crée un vecteur pour stocker les handles des threads
    let mut handles = vec![]; 
    
    // 2. On lance tous les threads simultanément
    for i in 0..5 {
        let db_clone = Arc::clone(&db);
        
        let handle = thread::spawn(move || {
            let mut db_lock = db_clone.write().unwrap();
            // Petite astuce : format!() renvoie déjà une String, le .to_string() est superflu
            db_lock.insert(format!("clé_{}", i), format!("valeur_{}", i));
        });
        
        // On sauvegarde le handle dans le vecteur (sans bloquer le programme)
        handles.push(handle); 
    }

	for i in 0..5 {
        let db_clone = Arc::clone(&db);

        let handle = thread::spawn(move || {
            // 1. Pas de "mut" ici
            let db_lock = db_clone.read().unwrap();
            
            // 2. On affiche directement "db_lock"
            println!("Thread de lecture {} : {:#?}", i, db_lock);
        });

        // 4. On sauvegarde le thread pour l'attendre plus tard
        handles.push(handle);
	}

    // 3. On attend que tous les threads aient terminé
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 4. On vérifie le résultat final ! (Le {:#?} permet un affichage formaté sur plusieurs lignes)
    println!("Base de données finale : {:#?}", db.read().unwrap());
}