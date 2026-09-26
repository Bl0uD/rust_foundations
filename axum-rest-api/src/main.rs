use axum::{routing::{get, post}, Router, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

async fn hello() -> &'static str {
    "Bienvenue sur mon API Rust !"
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(Serialize)]
struct Utilisateur {
    id: u32,
    username: String,
    email: String,
}

async fn create_user(Json(payload): Json<CreateUser>,) -> Result<(StatusCode, Json<Utilisateur>), (StatusCode, String)> {
	// On simule une insertion en base de données
    // "payload" contient les données désérialisées
    println!("Nouveau user reçu : {}", payload.username);

	if !payload.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            "Format d'email invalide : il manque un '@'".to_string(),
        ));
    }

	let new_user = Utilisateur {
		id: 99, // On simule un ID généré
		username: payload.username,
		email: payload.email,
	};

	// À la place de : (StatusCode::CREATED, Json(new_user))
    // Écris :
    Ok((StatusCode::CREATED, Json(new_user)))
}

async fn get_user() -> Json<Utilisateur> {
    // On crée une instance de notre structure
    let user = Utilisateur {
        id: 42,
        username: String::from("rust_student"), 
        email: "student@42.fr".to_string(), // .to_string() est une autre façon de créer une String
    };

    // On enveloppe notre objet dans Json() pour Axum
    Json(user)
}

#[tokio::main]
async fn main() {
	let app = Router::new().route("/", get(hello)).route("/user", get(get_user)).route("/create_user", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serveur lancé sur le port 3000...");
    
    axum::serve(listener, app).await.unwrap();
}
