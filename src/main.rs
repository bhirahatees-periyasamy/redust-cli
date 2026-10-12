use redust::service::{RedustService, ServiceOperations};

fn main() {
    let mut service = RedustService::new();

    // SET
    println!(
        "SET name = redust -> {}",
        service.set("name".into(), "redust".into())
    );

    // GET (existing key)
    println!("GET name -> {:?}", service.get("name"));

    // Overwrite
    println!(
        "SET name = rust -> {}",
        service.set("name".into(), "rust".into())
    );
    println!("GET name -> {:?}", service.get("name"));

    // GET (missing key)
    println!("GET missing -> {:?}", service.get("missing"));

    // DELETE
    println!("DELETE name -> {}", service.delete("name"));
    println!("DELETE name (again) -> {}", service.delete("name"));
    println!("GET name -> {:?}", service.get("name"));
}
