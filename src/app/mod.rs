use crate::app::run::{Run, RunError};
use crate::modules::auth::login::{Login, LoginError};
use thiserror::Error;

pub struct App {
    login: Login,
    routing: Run,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("login error")]
    LoginError(#[from] LoginError),
    #[error("login error")]
    MenuError(#[from] RunError),
}

impl App {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            login: Login::new()?,
            routing: Run::new(),
        })
    }

    pub fn launch(&mut self) -> Result<(), AppError> {
        self.login.validate()?;
        self.routing.start()?;
        Ok(())
    }
}

mod run;
