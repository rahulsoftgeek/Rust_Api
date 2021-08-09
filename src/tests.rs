extern crate httpmock;

use httpmock::MockServer;
use httpmock::Method::GET;
use serde_json::json;
use isahc::{prelude::*, Response,Request};

#[test]

fn get_emp_test() {
    // Arrange
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/emp/2")
            //.header("Authorization", " Basic Zm9vOmJhcg==")
            .header("Content-Type", "application/json");
        then.status(200)
            .json_body(json!({"created_at":"2021-07-24 20:54:29","email":"g@gmail.com","id":2,"name":"Rahul2"}));
    });
    let response = isahc::get(server.url("127.0.0.1:8000")).header("Authorization", "Basic Zm9vOmJhcg==");

    mock.assert();
    assert_eq!(response.status(), 200);
    
    //let client = get_emp("Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==", "127.0.0.1:8000");

    // Act
    //let result = client.create_repo("myRepo");

    // Assert
    //mock.assert();
    //assert_eq!(client.is_ok, true);
    //assert_eq!(client.unwrap(), "127.0.0.1:8000");
}