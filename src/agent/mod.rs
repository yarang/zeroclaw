#[allow(clippy::module_inception)]
pub mod agent;
pub mod classifier;
pub mod dispatcher;
pub mod loop_;
pub mod memory_loader;
pub mod prompt;
pub mod registry;
pub mod watcher;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use agent::{Agent, AgentBuilder};
#[allow(unused_imports)]
pub use loop_::{process_message, run};

// Re-export registry types for convenience
pub use registry::{
    AgentDefinition, AgentExecution, AgentMemory, AgentMetadata, AgentProvider, AgentRegistry,
    AgentReporting, AgentRetry, AgentSystem, AgentToolConfig, AgentToolDeny, AgentTools,
    ExecutionMode, MemoryBackend, OutputFormat, ReportingMode,
};

// Re-export watcher
pub use watcher::AgentWatcher;
