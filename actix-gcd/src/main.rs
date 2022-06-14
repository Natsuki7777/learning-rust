use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(get_index)));

    println!("Serving on http://localhost:3002");

    server
        .bind("127.0.0.1:3002")
        .expect("error binding server to add")
        .run()
        .expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().body(
        r#"
                <title>GCD</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute GCD</button>
                </form>
    "#,
    )
}
