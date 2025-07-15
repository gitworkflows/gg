use tokio::signal;
use log::{info, error};
use app::server::telemetry::TelemetryCollector;
use app::ai::mcp::view::{editor::McpViewEditor, collection::McpViewCollection, roles::McpRolesManager};
use app::api::ApiClient;
use app::input_block_example::run_input_block_examples;
use app::input_block_integration::InputBlockIntegrator;
use app::input_block::InputBlock;
use ui::core::app::CoreApp;
use ui::platform::app::PlatformApp;
use uuid::Uuid; // Import Uuid for generating IDs

mod shell;
mod editor; // This is now `input.rs`
mod fuzzy;
mod renderer;
mod themes; // This is now `config/theme.rs`, `config/yaml_theme.rs`, `config/yaml_theme_manager.rs`
mod theme_selector; // This is now `settings/yaml_theme_ui.rs`
mod theme_customizer; // This is now `settings/theme_editor.rs`
mod profiles;
mod profile_manager_ui;
mod profile_switcher;
mod workflow_browser; // This is now `workflows/ui.rs`
mod workflow_executor; // This is now `workflows/executor.rs`
mod collaboration;
mod command_palette;
mod config;
mod preferences;
mod prompt;
mod terminal;
mod warp_drive_ui;
mod workflows;
mod blocks; // This is now `block.rs`
mod agent_mode_eval;
mod asset_macro;
mod command;
mod graphql;
mod integration;
mod languages;
mod markdown_parser;
mod lpc;
mod mcq;
mod natural_language_detection;
mod resources;
mod serve_wasm;
mod string_offset;
mod sum_tree;
mod syntax_tree;
mod watcher;
mod drive; // Import the new drive module
mod websocket;
mod fuzzy_match;
mod virtual_fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Initialize Sentry for error reporting
    let _guard = sentry::init((
        "https://examplePublicKey@o0.ingest.sentry.io/0",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    info!("Warp Terminal starting...");

    // Initialize Telemetry
    let telemetry_collector = TelemetryCollector::new();
    telemetry_collector.collect_event("app_start", serde_json::json!({"version": "0.1.0"}));

    // Initialize UI crates
    let core_app = CoreApp::new();
    core_app.run();
    let platform_app = PlatformApp::new();
    platform_app.run();

    // Initialize AI MCP Views
    let mcp_editor = McpViewEditor::new();
    mcp_editor.render();
    let mcp_collection = McpViewCollection::new();
    mcp_collection.display();
    let mcp_roles_manager = McpRolesManager::new();
    mcp_roles_manager.assign_role("user_alpha", "developer");

    // Initialize API Client
    let api_client = ApiClient::new();
    match api_client.fetch_data("users/1").await {
        Ok(data) => info!("API Data: {}", data),
        Err(e) => error!("Failed to fetch API data: {:?}", e),
    }

    // Run Input Block Examples
    run_input_block_examples();

    // Demonstrate Input Block Integration
    let integrator = InputBlockIntegrator::new();
    let sample_block = InputBlock::new("This is a sample block.".to_string(), "text".to_string());
    integrator.integrate_block(&sample_block);

    // Start GraphQL Client (example usage)
    let graphql_client = graphql::GraphQLClient::new("https://countries.trevorblades.com/".to_string());
    let query = r#"
        query {
            country(code: "BR") {
                name
                capital
            }
        }
    "#.to_string();
    match graphql_client.execute_query(&query).await {
        Ok(response) => info!("GraphQL Response: {:?}", response),
        Err(e) => error!("GraphQL Error: {:?}", e),
    }

    // Start WASM Server
    let wasm_server = serve_wasm::WasmServer::new("127.0.0.1:3030".to_string());
    let server_handle = tokio::spawn(async move {
        if let Err(e) = wasm_server.start().await {
            error!("WASM server failed: {:?}", e);
        }
    });
    info!("WASM server started on http://127.0.0.1:3030");

    // Start Virtual Filesystem
    let mount_point = "/tmp/warp_vfs"; // Choose an appropriate mount point
    let vfs_handle = tokio::spawn(async move {
        if let Err(e) = virtual_fs::start_virtual_filesystem(mount_point).await {
            error!("Virtual filesystem failed: {:?}", e);
        }
    });
    info!("Virtual filesystem mounted at {}", mount_point);

    // Demonstrate LPC
    let processor = lpc::LanguageProcessor::new();
    let command = "show me files in /home".to_string();
    let processed_command = processor.process_command(&command);
    info!("Processed command: {:?}", processed_command);

    // Demonstrate Asset Macro
    let embedded_text = asset_macro::include_asset!("assets/dummy_asset.txt");
    info!("Embedded asset content: {}", embedded_text);
    info!("Embedded asset size: {} bytes", embedded_text.len());

    // --- Drive Manager Demonstration ---
    let mut drive_manager = drive::DriveManager::new();
    let workflow_item = drive::WarpDriveItem::Workflow {
        id: Uuid::new_v4(),
        name: "My First Workflow".to_string(),
    };
    let notebook_item = drive::WarpDriveItem::Notebook {
        id: Uuid::new_v4(),
        name: "Daily Notes".to_string(),
    };
    drive_manager.add_item(workflow_item.clone());
    drive_manager.add_item(notebook_item.clone());

    info!("Items in DriveManager: {:?}", drive_manager.get_all_items());
    if let Some(item) = drive_manager.get_item(&workflow_item.id()) {
        info!("Retrieved item by ID: {:?}", item);
    }
    // --- End Drive Manager Demonstration ---


    // Main application loop or event handling
    info!("Press Ctrl+C to exit...");
    signal::ctrl_c().await?;

    info!("Warp Terminal shutting down...");

    // Clean up virtual filesystem
    if let Err(e) = virtual_fs::unmount_virtual_filesystem(mount_point).await {
        error!("Failed to unmount virtual filesystem: {:?}", e);
    } else {
        info!("Virtual filesystem unmounted from {}", mount_point);
    }

    // Optionally, gracefully shut down the WASM server
    // wasm_server.stop().await; // Requires a mechanism to send stop signal to the spawned task

    server_handle.abort(); // Abort the server task for now

    Ok(())
}
