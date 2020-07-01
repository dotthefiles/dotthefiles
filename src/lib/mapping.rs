use crate::lib::config::Config;
use async_std::io;
use async_std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct File {
  name: String,
  from: PathBuf,
  to: PathBuf,
}

#[derive(Debug)]
pub struct Mapping<'a> {
  pub base_dir: &'a PathBuf,
  pub os_type: &'a os_info::Type,
  pub home_dir: &'a PathBuf,
}

impl<'a> Mapping<'a> {
  pub async fn map(&self, config: &Config) -> io::Result<Vec<File>> {
    let mut v: Vec<File> = Vec::with_capacity(32);

    for section in &config.map {
      let mut compatible = false;

      for target in &section.target {
        if target == self.os_type {
          compatible = true;
          break;
        }
      }

      if !compatible {
        continue;
      }

      for file in &section.files {
        let to: PathBuf = if file.to == "~/" {
          self.home_dir.clone()
        } else {
          PathBuf::from(&file.to)
        };

        let from = PathBuf::from(self.base_dir).join("files/linux");

        v.push(File {
          name: file.name.clone(),
          to,
          from,
        })
      }
    }

    Ok(v)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lib::read_yaml;

  fn base_dir(t: &str) -> PathBuf {
    PathBuf::from(std::env::current_dir().unwrap())
      .join("examples")
      .join(t)
  }

  #[tokio::test]
  /// given the right target it should provide us with the simplest file mapping
  async fn a01() -> io::Result<()> {
    let os_type = os_info::Type::Linux;
    let base_dir = &base_dir("a01");
    let home_dir = dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path).await?;

    let expected: Vec<File> = vec![File {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files/linux")),
      to: PathBuf::from(&home_dir),
    }];

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config).await?;

    assert_eq!(actual, expected);

    Ok(())
  }

  #[tokio::test]
  /// If we are using undeclared OS then return nothing
  async fn a02() -> io::Result<()> {
    let os_type = os_info::Type::Macos;
    let base_dir = &base_dir("a02");
    let home_dir = dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path).await?;

    let expected: Vec<File> = vec![];

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config).await?;

    assert_eq!(actual, expected);

    Ok(())
  }
}
