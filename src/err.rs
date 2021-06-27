use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerFrameError {
	Interrupted(String),
}
