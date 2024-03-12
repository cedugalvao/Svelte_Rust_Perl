use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct FormData {
    nome: String,
    cpf: String,
    email: String,
    senha: String,
    confirmarSenha: String,
}

async fn receive_form(form: web::Json<FormData>) -> impl Responder {
    println!("Nome: {}", form.nome);
    println!("CPF: {}", form.cpf);
    println!("Email: {}", form.email);
    println!("Senha: {}", form.senha);
    println!("Confirmar Senha: {}", form.confirmarSenha);
    let output = pass_to_perl(&form).expect("Falha ao executar o script Perl");

    HttpResponse::Ok().body(String::from_utf8(output.stdout).unwrap())
}

fn pass_to_perl(form: &FormData) -> std::io::Result<std::process::Output> {
    Command::new("perl")
        .arg("C:/Users/Cadu/Documents/react/proj2/OxeBank_PERL/Main.pl")
        .arg(&form.nome)
        .arg(&form.cpf)
        .output()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(receive_form))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}