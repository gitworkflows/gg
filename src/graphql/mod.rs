use reqwest::Client;
use serde_json::{json, Value};
use anyhow::{Result, anyhow};
use log::{info, error};

/// Represents a GraphQL client for making queries and mutations.
pub struct GraphQLClient {
    client: Client,
    endpoint: String,
}

impl GraphQLClient {
    /// Creates a new `GraphQLClient` instance.
    ///
    /// # Arguments
    /// * `endpoint` - The URL of the GraphQL API.
    pub fn new(endpoint: String) -> Self {
        GraphQLClient {
            client: Client::new(),
            endpoint,
        }
    }

    /// Executes a GraphQL query or mutation.
    ///
    /// # Arguments
    /// * `query` - The GraphQL query string.
    /// * `variables` - Optional JSON object for query variables.
    pub async fn execute_query(&self, query: &str, variables: Option<Value>) -> Result<Value> {
        let request_body = json!({
            "query": query,
            "variables": variables.unwrap_or_else(|| json!({})),
        });

        info!("Sending GraphQL request to {}: {}", self.endpoint, request_body);

        let response = self.client.post(&self.endpoint)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send GraphQL request: {:?}", e))?;

        let status = response.status();
        let response_body: Value = response.json().await
            .map_err(|e| anyhow!("Failed to parse GraphQL response: {:?}", e))?;

        if status.is_success() {
            if let Some(errors) = response_body.get("errors") {
                error!("GraphQL response contained errors: {}", errors);
                Err(anyhow!("GraphQL errors: {}", errors))
            } else {
                info!("GraphQL request successful. Data: {}", response_body.get("data").unwrap_or(&json!({})));
                Ok(response_body)
            }
        } else {
            error!("GraphQL HTTP error {}: {}", status, response_body);
            Err(anyhow!("GraphQL HTTP error: {} - {}", status, response_body))
        }
    }

    /// A convenience method for `post` that matches the previous `main.rs` usage.
    pub async fn post(&self, query: &str, variables: Option<Value>) -> Result<Value> {
        self.execute_query(query, variables).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, body_json};
    use serde_json::json;

    #[tokio::test]
    async fn test_graphql_client_success() -> Result<()> {
        let mock_server = MockServer::start().await;
        let client = GraphQLClient::new(mock_server.uri());

        let query = "{ hero { name } }";
        let expected_response = json!({
            "data": {
                "hero": {
                    "name": "R2-D2"
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_json(json!({"query": query, "variables": {}})))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let response = client.execute_query(query, None).await?;
        assert_eq!(response, expected_response);
        Ok(())
    }

    #[tokio::test]
    async fn test_graphql_client_with_variables() -> Result<()> {
        let mock_server = MockServer::start().await;
        let client = GraphQLClient::new(mock_server.uri());

        let query = "query Hero($episode: Episode) { hero(episode: $episode) { name } }";
        let variables = json!({"episode": "JEDI"});
        let expected_response = json!({
            "data": {
                "hero": {
                    "name": "Luke Skywalker"
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_json(json!({"query": query, "variables": variables})))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let response = client.execute_query(query, Some(variables)).await?;
        assert_eq!(response, expected_response);
        Ok(())
    }

    #[tokio::test]
    async fn test_graphql_client_graphql_error() {
        let mock_server = MockServer::start().await;
        let client = GraphQLClient::new(mock_server.uri());

        let query = "{ invalidField }";
        let error_response = json!({
            "errors": [
                {
                    "message": "Cannot query field \"invalidField\" on type \"Query\".",
                    "locations": [ { "line": 1, "column": 3 } ]
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_json(json!({"query": query, "variables": {}})))
            .respond_with(ResponseTemplate::new(200).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let result = client.execute_query(query, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("GraphQL errors"));
    }

    #[tokio::test]
    async fn test_graphql_client_http_error() {
        let mock_server = MockServer::start().await;
        let client = GraphQLClient::new(mock_server.uri());

        let query = "{ hero { name } }";

        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_json(json!({"query": query, "variables": {}})))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let result = client.execute_query(query, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("GraphQL HTTP error: 500 - \"Internal Server Error\""));
    }
}
