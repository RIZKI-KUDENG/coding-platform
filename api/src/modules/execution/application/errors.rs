use crate::modules::shared::error::InfrastructureError;

#[derive(Debug)]
pub enum ExecutionError {
    RunnerUnavailable(InfrastructureError),
    Persistence(InfrastructureError),
}
