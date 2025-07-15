use wasmer::{Store, Module, Instance, Value, imports};
use deno_core::{JsRuntime, RuntimeOptions};
use std::collections::HashMap;

pub struct AgentExecutor {
    // For WebAssembly (Wasmer)
    wasmer_store: Store,
    // For JavaScript (Deno)
    deno_runtime: JsRuntime,
}

impl AgentExecutor {
    pub fn new() -> Self {
        AgentExecutor {
            wasmer_store: Store::default(),
            deno_runtime: JsRuntime::new(RuntimeOptions::default()),
        }
    }

    pub fn execute_wasm_agent(&mut self, wasm_bytes: &[u8], function_name: &str, args: &[Value]) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let module = Module::new(&self.wasmer_store, wasm_bytes)?;
        let import_object = imports! {}; // Define any necessary imports for the WASM module
        let instance = Instance::new(&mut self.wasmer_store, &module, &import_object)?;
        
        let function = instance.get_function(&mut self.wasmer_store, function_name)
            .ok_or(format!("Function '{}' not found in WASM module", function_name))?;
        
        let results = function.call(&mut self.wasmer_store, args)?;
        Ok(results.to_vec())
    }

    pub async fn execute_js_agent(&mut self, js_code: &str, context: HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
        // Inject context into the JS runtime
        let context_json = serde_json::to_string(&context)?;
        let setup_script = format!("const context = JSON.parse(`{}`);", context_json);
        self.deno_runtime.execute_script("<setup>", &setup_script)?;

        // Execute the agent's JavaScript code
        let result = self.deno_runtime.execute_script("<agent_code>", js_code)?;
        
        // Get the result from the JS runtime (e.g., by calling a specific function or reading a global variable)
        let scope = &mut self.deno_runtime.handle_scope();
        let value = result.open(scope);
        let js_result = value.to_string(scope).unwrap().to_rust_string_lossy(scope);
        
        Ok(js_result)
    }
}
