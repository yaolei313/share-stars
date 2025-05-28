use anyhow::Result;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub trait PathResolveExt {
    fn resolve(&self) -> Result<PathBuf>;
}

impl<T> PathResolveExt for T
where
    T: AsRef<OsStr>,
{
    fn resolve(&self) -> Result<PathBuf> {
        let mut p = PathBuf::new();
        p.push(self.as_ref());
        if p.is_absolute() {
            return Ok(p);
        }

        if !p.starts_with("~") {
            // 相对路径处理
            let base = std::env::current_dir()?;
            let resolved = base.join(p);
            println!("before: {:?}", resolved);
            let mut cleaned_components = Vec::new();
            for component in resolved.components() {
                match component {
                    std::path::Component::ParentDir => {
                        // ".."
                        cleaned_components.pop(); // 移除上一个组件
                    }
                    std::path::Component::CurDir => { // "."
                        // 忽略当前目录
                    }
                    _ => {
                        // 其他组件（Normal, Prefix, RootDir）
                        cleaned_components.push(component);
                    }
                }
            }
            let p2: PathBuf = cleaned_components.into_iter().collect();
            println!("after: {:?}", p2);
            return Ok(p2);
        }

        // 相对home路径处理
        let home = dirs::home_dir()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "homedir not found"))?;
        if p.eq(Path::new("~")) {
            return Ok(home);
        }
        let stripped = p.strip_prefix("~/")?;
        Ok(home.join(stripped))
    }
}

#[cfg(test)]
mod tests {
    use crate::PathResolveExt;
    use std::path::Path;

    #[test]
    fn it_works() {
        assert_eq!(
            "./README.md".resolve().expect("resolve fail"),
            Path::new("/Users/yao/study/share-stars/crates/libs/lib-utils/README.md")
        );
        assert_eq!(
            "~/README.md".resolve().expect("resolve fail"),
            Path::new("/Users/yao/README.md")
        );
        assert_eq!(
            "~".resolve().expect("resolve fail"),
            Path::new("/Users/yao/")
        );
    }
}
