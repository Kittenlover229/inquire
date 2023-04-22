use crate::error::InquireResult;
use crate::terminal::{get_default_terminal, Terminal};
use crate::ui::{Backend, RenderConfig};
use crate::validator::PathValidator;
use std::collections::HashSet;
use std::env::{self, current_dir};
use std::fs::FileType;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Explorer<'a> {
    /// Message to be presented to the user.
    message: &'a str,

    /// Filetypes to display and allow for selection
    filetype_filter: HashSet<FileType>,

    /// RenderConfig to apply to the rendered interface.
    render_config: RenderConfig<'a>,

    /// Validators to check the filepath against
    /// Validators are executed in order, stopping when an error has appeared
    /// The error is displayed above the current path and path list
    validators: Vec<Box<dyn PathValidator>>,

    vim_mode: bool,
}

impl<'a> Explorer<'a> {
    /// Default validators added to the [Explorer] prompt, none.
    pub const DEFAULT_VALIDATORS: Vec<Box<dyn PathValidator>> = vec![];

    /// Default value of vim mode.
    pub const DEFAULT_VIM_MODE: bool = false;
}

impl<'a> Explorer<'a> {
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            filetype_filter: Default::default(),
            render_config: Default::default(),
            validators: Self::DEFAULT_VALIDATORS,
            vim_mode: Self::DEFAULT_VIM_MODE,
        }
    }

    /// Parses the provided behavioral and rendering options and prompts
    /// the CLI user for input according to the defined rules.
    pub fn prompt(self) -> InquireResult<PathBuf> {
        let terminal = get_default_terminal()?;
        let mut backend = Backend::new(terminal, self.render_config)?;
        self.prompt_with_backend(&mut backend)
    }

    pub(crate) fn prompt_with_backend<T: Terminal>(
        self,
        backend: &mut Backend<'a, T>,
    ) -> InquireResult<PathBuf> {
        FileSelectPrompt::new(self)?.prompt(backend)
    }
}

struct FileSelectPrompt<'a> {
    message: &'a str,
    current_path: PathBuf,
}

impl<'a> FileSelectPrompt<'a> {
    pub fn new(explorer: Explorer<'a>) -> InquireResult<Self> {
        Ok(FileSelectPrompt {
            message: explorer.message,
            current_path: PathBuf::from(current_dir()?),
        })
    }

    pub fn prompt<T: Terminal>(self, backend: &mut Backend<'a, T>) -> InquireResult<PathBuf> {
        Ok(self.current_path.into())
    }
}
