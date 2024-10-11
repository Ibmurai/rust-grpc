use tonic::{transport::Server, Request, Response, Status};

// Include the generated code
tonic::include_proto!("example"); // Matches the package name in your proto file

#[derive(Debug, Default)]
pub struct MySearchService {}

// Implement the SearchService trait
#[tonic::async_trait]
impl search_service_server::SearchService for MySearchService {
    async fn search(
        &self,
        _request: Request<Empty>,  // We expect an empty request
    ) -> Result<Response<SearchResponse>, Status> {
        // Create some mock data (you can replace with actual data)
        let person1 = Person {
            name: Some("John Doe".to_string()),
            id: Some(1),
            email: Some("john.doe@example.com".to_string()),
            phones: vec![person::PhoneNumber {
                number: Some("123-4567".to_string()),
                r#type: Some(person::PhoneType::Mobile as i32),
            }],
        };

        let person2 = Person {
            name: Some("Jane Smith".to_string()),
            id: Some(2),
            email: Some("jane.smith@example.com".to_string()),
            phones: vec![person::PhoneNumber {
                number: Some("987-6543".to_string()),
                r#type: Some(person::PhoneType::Work as i32),
            }],
        };

        let address_book = AddressBook {
            people: vec![person1, person2],
        };

        let response = SearchResponse {
            result: vec![address_book],
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let search_service = MySearchService::default();

    println!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(search_service_server::SearchServiceServer::new(search_service))
        .serve(addr)
        .await?;

    Ok(())
}
