use dtflib::client_os;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Target {
  #[serde(alias = "linux")]
  Linux,

  #[serde(alias = "darwin", alias = "macos")]
  Darwin,

  #[serde(alias = "win", alias = "windows")]
  Win,

  #[serde(
    alias = "*",
    alias = "any",
    alias = "all",
    alias = "every",
    alias = "each"
  )]
  Any,
}

impl Default for Target {
  fn default() -> Self {
    Self::Any
  }
}

impl std::cmp::PartialEq<client_os::Type> for Target {
  fn eq(&self, x: &client_os::Type) -> bool {
    match self {
      Target::Linux => x == &client_os::Type::Linux,
      Target::Darwin => x == &client_os::Type::Darwin,
      Target::Win => x == &client_os::Type::Win,
      Target::Any => true,
    }
  }
}

impl std::cmp::PartialOrd for Target {
  fn partial_cmp(&self, x: &Self) -> Option<std::cmp::Ordering> {
    match self {
      Target::Any => match x {
        Target::Any => Some(std::cmp::Ordering::Equal),
        _ => Some(std::cmp::Ordering::Less),
      },
      _ => match x {
        Target::Any => Some(std::cmp::Ordering::Greater),
        _ => Some(std::cmp::Ordering::Equal),
      },
    }
  }
}

impl std::cmp::Ord for Target {
  fn cmp(&self, x: &Self) -> std::cmp::Ordering {
    self.partial_cmp(x).unwrap()
  }
}

#[derive(Debug, Deserialize)]
pub struct Block {
  pub name: String,
  pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct Section {
  #[serde(default = "Section::default_target")]
  pub target: Vec<Target>,

  pub files: Vec<Block>,

  #[serde(default = "Section::default_from")]
  pub from: String,
}

impl Section {
  fn default_target() -> Vec<Target> {
    vec![Target::default()]
  }

  fn default_from() -> String {
    String::from("files/$TARGET")
  }
}

#[derive(Debug, Deserialize)]
pub struct Config {
  pub map: Vec<Section>,
}