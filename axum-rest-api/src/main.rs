use axum::{routing::{get, post, delete}, Router};
use sqlx::postgres::PgPoolOptions;

mod handlers;
use handlers::{create_user, get_user, delete_user, update_user};

async fn hello() -> &'static str {
	"Bienvenue sur mon API Rust !"
}

#[tokio::main]
async fn main() {
	// 1. Définir l'URL de connexion (On utilisera des variables d'environnement plus tard)
	// Format : postgres://utilisateur:motdepasse@hote:port/nom_de_la_base
	let db_url = "postgres://postgres:monmotdepasse@localhost:5432/rust_api_db";

	// 2. Créer le Pool de connexions
	let pool = PgPoolOptions::new()
		.max_connections(5) // On garde 5 connexions ouvertes maximum
		.connect(db_url)
		.await
		.expect("Erreur : Impossible de se connecter à la base de données PostgreSQL");

	println!("✅ Connexion à la base de données réussie !");

	sqlx::query(
		"CREATE TABLE IF NOT EXISTS utilisateurs (
			id SERIAL PRIMARY KEY,
			username TEXT NOT NULL,
			email TEXT NOT NULL UNIQUE
		)"
	)
	.execute(&pool)
	.await
	.expect("Erreur lors de la création de la table");

	let app = Router::new()
		.route("/", get(hello))
		.route("/user", get(get_user))
		.route("/create_user", post(create_user))
		.route("/user/{id}", delete(delete_user).put(update_user))
		.with_state(pool); // injecteur de donnees

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	println!("Serveur lancé sur le port 3000...");
	
	axum::serve(listener, app).await.unwrap();
}