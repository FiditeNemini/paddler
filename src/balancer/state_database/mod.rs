mod file;
mod memory;

use anyhow::Result;
use async_trait::async_trait;

pub use self::file::File;
pub use self::memory::Memory;
use crate::agent_desired_state::AgentDesiredState;

#[async_trait]
pub trait StateDatabase: Send + Sync {
    async fn read_agent_desired_state(&self) -> Result<AgentDesiredState>;

    async fn store_agent_desired_state(&self, state: &AgentDesiredState) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempfile::NamedTempFile;

    use super::*;
    use crate::agent_desired_model::AgentDesiredModel;
    use crate::inference_parameters::InferenceParameters;

    async fn subtest_store_desired_state<TDatabase: StateDatabase>(db: &TDatabase) -> Result<()> {
        let desired_state = AgentDesiredState {
            inference_parameters: InferenceParameters::default(),
            model: AgentDesiredModel::Local("test_model_path".to_string()),
        };

        db.store_agent_desired_state(&desired_state).await?;

        let read_state = db.read_agent_desired_state().await?;

        assert_eq!(read_state.model, desired_state.model);

        Ok(())
    }

    #[tokio::test]
    async fn test_file_database() -> Result<()> {
        let tempfile = NamedTempFile::new()?;
        let db = File::new(tempfile.path().to_path_buf());

        subtest_store_desired_state(&db).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_memory_database() -> Result<()> {
        let db = Memory::new();

        subtest_store_desired_state(&db).await?;

        Ok(())
    }
}
