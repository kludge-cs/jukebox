use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PlayerFrameError {
	#[error("Frame was interrupted - {0}")]
	Interrupted(String),
	#[error("Frame timed out - {0}")]
	Timeout(String),
}
