use reqwest::Client;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
pub struct ErrbitError {
    api_key: String,
    environment: String,
    app_name: String,
    version: String,
    source_url: String,
    node_name: String,
    component_name: String,
    exc_class: String,
    message: String,
    backtrace: Option<Vec<(String, u32, String, String)>>,
}

pub struct ErrbitClient {
    client: Client,
    endpoint: String,
    api_key: String,
}

impl ErrbitClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {

        ErrbitClient {
            client: Client::new(),
            endpoint: endpoint.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn notify(&self, error: &dyn Error) -> Result<(), Box<dyn Error>> {
        let notify_endpont = format!("{}/notifier_api/v2/notices", self.endpoint);
        
        let environment = std::env::var("RUST_ENV").unwrap_or_else(|_| "production".to_string());
        let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "MyApp".to_string());
        let node_name = std::env::var("NODE_NAME").unwrap_or_else(|_| "hostname".to_string());
        let component_name = std::env::var("COMPONENT_NAME").unwrap_or_else(|_| "MyComponent".to_string());
        let source_url = std::env::var("SOURCE_URL").unwrap_or_else(|_| {
            // Fallback to something like a hostname or local IP
            format!("https://{}", hostname::get().unwrap_or_default().to_string_lossy())
        });

        let error = ErrbitError {
            api_key: self.api_key.clone(),
            environment,
            app_name,
            version: "1.0".to_string(),
            source_url: source_url,
            node_name,
            component_name,
            exc_class: error.to_string(),
            message: error.to_string(),
            backtrace: Some(vec![("filename".to_string(), 42, "function_name".to_string(), "text".to_string())]),
        };
    
        let xml_payload = generate_errbit_xml(&error);
        let resp = self.client
            .post(&notify_endpont)
            .header("X-Airbrake-Token", &self.api_key)
            .header("Content-Type", "application/xml")
            .body(xml_payload)
            .send()
            .await;

        match resp {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Success: {:?}", response);
                    Ok(())
                } else {
                    println!("Request error: {:?}", response);
                    Err(Box::from(format!("Request failed with status: {}", response.status())))
                }
            }
            Err(err) => {
                println!("HTTP error: {:?}", err);
                Err(Box::new(err))
            }
        }
    }
}

/// Generates an XML payload similar to the Python example.
pub fn generate_errbit_xml(
    error: &ErrbitError,
) -> String {
    let mut xml = format!(
r#"<notice version="2.0">
    <api-key>{api_key}</api-key>
    <notifier>
        <name>{app_name}</name>
        <version>{version}</version>
        <url>{source_url}</url>
    </notifier>
    <server-environment>
        <environment-name>{environment}</environment-name>
    </server-environment>
    <request>
        <url></url>
        <component>{component_name}</component>
        <cgi-data>
            <var key="nodeName">{node_name}</var>
            <var key="componentName">{component_name}</var>
        </cgi-data>
    </request>
    <error>
        <class>{exc_class}</class>
        <message>{message}</message>
"#,
        api_key = error.api_key,
        environment = error.environment,
        app_name = error.app_name,
        version = error.version,
        source_url = error.source_url,
        node_name = error.node_name,
        component_name = error.component_name,
        exc_class = error.exc_class,
        message = error.message,
    );

    // If a backtrace exists, append its lines
    if !error.backtrace.clone().unwrap().is_empty() {
        xml.push_str("        <backtrace>\n");
        for (filename, lineno, function, _text) in error.backtrace.as_ref().unwrap() {
            xml.push_str(&format!(
                "            <line method=\"{function}\" file=\"{filename}\" number=\"{lineno}\"/>\n",
                function = function,
                filename = filename,
                lineno = lineno,
            ));
        }
        xml.push_str("        </backtrace>\n");
    };

    xml.push_str(
        r#"    </error>
        </notice>"#,
    );
    xml

}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_notify() {
        let rt = Runtime::new().unwrap();
        let url = std::env::var("ERRBIT_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let api_key = std::env::var("ERRBIT_API_KEY").unwrap_or_else(|_| "generate-app-key-in-errbit".to_string());
        rt.block_on(async {
            let client = ErrbitClient::new(&url.as_str(), &api_key.as_str());
            let error = std::io::Error::new(std::io::ErrorKind::Other, "test error");
            let result = client.notify(&error).await;
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_reqwest_json() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let client = reqwest::Client::new();
            let res = client
                .post("https://httpbin.org/post")
                .json(&serde_json::json!({ "key": "value" }))
                .send()
                .await;
            assert!(res.is_ok());
        });
    }
}
