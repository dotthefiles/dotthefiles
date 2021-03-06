use cli::{self, App, Cli};
use dtflib::{client_os, Context, DotFile};
use parser::Parser;
use std::collections::HashMap;
use std::io::Result;

mod validate_config;
use validate_config::validate_config;

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args()
    .filter(|arg| arg != dtflib::CHILD_PARAM)
    .collect();
  let app = App::with_args(&args);

  let home_dir = &dirs::home_dir().unwrap();

  let child: bool = match std::env::args().find(|arg| arg == dtflib::CHILD_PARAM) {
    None => false,
    Some(_) => true,
  };

  match app {
    Cli::Link { config, force, os } => {
      let (config_path, base_dir) = &validate_config(&config);
      let client_os = client_os::digest(os);

      let cx = Context {
        config_path,
        base_dir,
        client_os: &client_os,
        home_dir,
        child,
      };

      let mut parser = Parser::with(&cx);

      if cx.is_main() {
        let dotfiles = parser.parse(&config_path)?;

        cli::link(&cx, &dotfiles, force)?;
      } else {
        let mut dotfiles_json = String::with_capacity(256);
        std::io::stdin().read_line(&mut dotfiles_json)?;

        let dotfiles: HashMap<u32, DotFile> = serde_json::from_str(&dotfiles_json)?;

        cli::link(&cx, &dotfiles, force)?;
      }
    }
    Cli::List { config, os } => {
      let (config_path, base_dir) = &validate_config(&config);
      let client_os = client_os::digest(os);

      let cx = Context {
        config_path,
        base_dir,
        client_os: &client_os,
        home_dir,
        child,
      };

      let mut parser = Parser::with(&cx);
      let dotfiles = parser.parse(&config_path)?;

      cli::list(&cx, &dotfiles)?;
    }
    Cli::Show { config } => {
      let (config_path, base_dir) = &validate_config(&config);
      let client_os = client_os::digest(None);

      let cx = Context {
        config_path,
        base_dir,
        client_os: &client_os,
        home_dir,
        child,
      };

      let mut parser = Parser::with(&cx);
      parser.read_config(config_path)?;

      let config_str = serde_json::to_string_pretty(parser.config().unwrap()).unwrap();

      cli::show(&config_str)?;
    }
  }

  Ok(())
}
