use axum::{Json, http::StatusCode, extract::State};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateUser {
	pub username: String,
	pub email: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Utilisateur {
	pub id: i32,
	pub username: String,
	pub email: String,
}

pub async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUser>,) -> Result<(StatusCode, Json<Utilisateur>), (StatusCode, String)> {
	// On simule une insertion en base de données
	// "payload" contient les données désérialisées
	println!("Nouveau user reçu : {}", payload.username);
	
	if !payload.email.contains('@') {
		return Err((
			StatusCode::BAD_REQUEST,
			"Format d'email invalide : il manque un '@'".to_string(),
		));
	}
	
	// La requête SQL. Les $1 et $2 sont des variables sécurisées pour éviter les injections SQL.
	// Le RETURNING permet de récupérer la ligne complète générée par Postgres.
	let requete = "
		INSERT INTO utilisateurs (username, email) 
		VALUES ($1, $2) 
		RETURNING id, username, email
	";

	// On exécute la requête
	let resultat = sqlx::query_as::<_, Utilisateur>(requete)
		.bind(&payload.username) // Remplace $1
		.bind(&payload.email)    // Remplace $2
		.fetch_one(&pool)        // Exécute et attend une seule ligne en retour
		.await;

	// On gère le résultat (succès ou erreur, par exemple si l'email existe déjà)
	match resultat {
		Ok(new_user) => Ok((StatusCode::CREATED, Json(new_user))),
		Err(e) => {
			println!("Erreur d'insertion : {:?}", e);
			Err((StatusCode::INTERNAL_SERVER_ERROR, "Impossible de créer l'utilisateur".to_string()))
		}
	}
}

pub async fn get_user(State(pool): State<PgPool>,) -> Result<Json<Vec<Utilisateur>>, (StatusCode, String)> {
	// 1. La requête entre guillemets
    let requete = "SELECT id, username, email FROM utilisateurs";

    // 2. L'exécution asynchrone
    let resultat = sqlx::query_as::<_, Utilisateur>(requete)
        .fetch_all(&pool)
        .await;

    // 3. La gestion du résultat
    match resultat {
        Ok(user) => Ok(Json(user)), // Succès : on renvoie le JSON
        Err(e) => {
            println!("Erreur de lecture : {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string()))
        }
    }
}