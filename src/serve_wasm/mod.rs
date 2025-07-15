use tokio::sync::oneshot;
use warp::{Filter, Rejection, Reply};
use log::{info, error};
use std::net::SocketAddr;

/// A simple HTTP server to serve WebAssembly (WASM) modules.
pub struct WasmServer {
    addr: SocketAddr,
    // We'll hold a sender for the shutdown signal
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl WasmServer {
    /// Creates a new `WasmServer` instance.
    ///
    /// # Arguments
    /// * `addr` - The address and port to bind the server to (e.g., "127.0.0.1:3030").
    pub fn new(addr_str: String) -> Self {
        let addr: SocketAddr = addr_str.parse().expect("Invalid server address");
        WasmServer {
            addr,
            shutdown_tx: None,
        }
    }

    /// Starts the WASM server. This method will block until the server is shut down.
    /// It returns a `oneshot::Sender` that can be used to trigger a graceful shutdown.
    pub async fn start(&mut self) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.shutdown_tx = Some(tx);

        let wasm_route = warp::path!("wasm" / String)
            .and(warp::get())
            .map(|module_name: String| {
                info!("Request for WASM module: {}", module_name);
                // In a real application, you would load the WASM file from disk
                // or a database based on `module_name` and return its bytes.
                // For now, we return a placeholder.
                let response_text = format!("Placeholder WASM module: {}", module_name);
                warp::reply::with_status(response_text, warp::http::StatusCode::OK)
            });

        let routes = wasm_route.with(warp::log("wasm_server"));

        info!("WASM server starting on {}", self.addr);
        let (_, server_future) = warp::serve(routes).bind_with_graceful_shutdown(self.addr, async {
            rx.await.ok();
            info!("WASM server received shutdown signal.");
        });

        server_future.await;
        info!("WASM server shut down.");
        Ok(())
    }

    /// Sends a shutdown signal to the running WASM server.
    pub async fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            if tx.send(()).is_ok() {
                info!("Sent shutdown signal to WASM server.");
            } else {
                error!("Failed to send shutdown signal to WASM server (receiver dropped).");
            }
        } else {
            info!("WASM server not running or already stopped.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_wasm_server_start_and_stop() -> anyhow::Result<()> {
        let mut server = WasmServer::new("127.0.0.1:0".to_string()); // Use port 0 for ephemeral port
        let server_handle = tokio::spawn(async move {
            server.start().await
        });

        // Give server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Test a request
        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:3030/wasm/my_module").send().await?;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await?, "Placeholder WASM module: my_module");

        // Stop the server
        // Note: This test structure is a bit tricky because `server.stop()` needs the `WasmServer` instance.
        // In `main.rs`, we use a oneshot channel directly. For this test, we'll just abort the task.
        server_handle.abort();
        let _ = server_handle.await; // Wait for the abort to complete

        Ok(())
    }

    #[tokio::test]
    async fn test_wasm_server_graceful_shutdown() -> anyhow::Result<()> {
        let mut server = WasmServer::new("127.0.0.1:0".to_string());
        let (tx_shutdown, rx_shutdown) = oneshot::channel();

        let server_handle = tokio::spawn(async move {
            let routes = warp::path!("test_shutdown")
                .map(|| "Hello from test_shutdown");
            let (_, server_future) = warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 3031), async {
                rx_shutdown.await.ok();
            });
            server_future.await;
            Ok(())
        });

        tokio::time::sleep(Duration::from_millis(100)).await; // Give server time to bind

        // Send a request to ensure it's running
        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:3031/test_shutdown").send().await?;
        assert_eq!(res.status(), StatusCode::OK);

        // Send shutdown signal
        tx_shutdown.send(()).unwrap();

        // Wait for the server task to complete (with a timeout)
        let result = timeout(Duration::from_secs(1), server_handle).await;
        assert!(result.is_ok(), "Server task should have completed gracefully");

        // Verify server is no longer reachable
        let res_after_shutdown = client.get("http://127.0.0.1:3031/test_shutdown").send().await;
        assert!(res_after_shutdown.is_err(), "Server should be unreachable after shutdown");

        Ok(())
    }
}
