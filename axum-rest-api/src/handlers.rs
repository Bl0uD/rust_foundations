use axum::{Json, http::StatusCode, extract::{State, Path}};
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

#[derive(Deserialize)]
pub struct UpdateUser {
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

pub	async fn delete_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, String)> {
	// Contrairement au SELECT, un DELETE ne renvoie pas de données,
    // on utilise donc la méthode `.execute()` de SQLx au lieu de query_as.
    
    let requete = "DELETE FROM utilisateurs WHERE id = $1";

    let resultat = sqlx::query(requete) // Note : query() tout court, pas query_as()
        .bind(id)
        .execute(&pool)
        .await;
	
	match resultat {
        Ok(res) => {
            // res.rows_affected() nous dit combien de lignes ont été supprimées
            if res.rows_affected() == 0 {
                Err((StatusCode::NOT_FOUND, "Utilisateur introuvable".to_string()))
            } else {
                Ok(StatusCode::OK) // Tout s'est bien passé
            }
        }
        Err(e) => {
            println!("Erreur de suppression : {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string()))
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

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
	) -> Result<(StatusCode, Json<Utilisateur>), (StatusCode, String)> {
    
    // On utilise UPDATE avec la clause RETURNING pour renvoyer la ligne une fois modifiée
    let requete = "
        UPDATE utilisateurs 
        SET username = $1, email = $2 
        WHERE id = $3 
        RETURNING id, username, email
    ";

    // On attache ("bind") les paramètres dans l'ordre exact des $1, $2, $3
    let resultat = sqlx::query_as::<_, Utilisateur>(requete)
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(id)
        .fetch_one(&pool)
        .await;

    match resultat {
        Ok(updated_user) => Ok((StatusCode::OK, Json(updated_user))),
        Err(e) => {
            println!("Erreur de mise à jour : {:?}", e);
            Err((StatusCode::NOT_FOUND, "Utilisateur introuvable ou email déjà pris".to_string()))
        }
    }
}