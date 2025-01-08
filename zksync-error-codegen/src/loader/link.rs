use std::path::PathBuf;

use super::{error::LinkError, DescriptionFile};

#[derive(Clone, Debug)]
pub enum Link {
    PackageLink { package: String, filename: String },
    URL { url: String },
}
impl Link {
    /// Part before "://"
    pub const FORMAT_PREFIX: &str = "cargo";
    pub const PACKAGE_SEPARATOR: &str = "@@";
}
impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Link::PackageLink { package, filename } => f.write_fmt(format_args!(
                "{}://{package}{}{filename}",
                Link::FORMAT_PREFIX,
                Link::PACKAGE_SEPARATOR
            )),
            Link::URL { url } => f.write_str(url),
        }
    }
}

pub fn parse_link(link: impl Into<String>) -> Result<Link, LinkError> {
    let string: String = link.into();

    match string.split_once("://") {
        Some((Link::FORMAT_PREFIX, path)) => match path.split_once(Link::PACKAGE_SEPARATOR) {
            Some((package, filename)) => Ok(Link::PackageLink {
                package: package.to_owned(),
                filename: filename.to_owned(),
            }),
            None => Err(LinkError::InvalidLinkFormat(string)),
        },
        Some((_, url)) => Ok(Link::URL {
            url: url.to_string(),
        }),
        _ => Err(LinkError::InvalidLinkFormat(string)),
    }
}

pub fn link_matches(link: &Link, file: &DescriptionFile) -> bool {
    if let Link::PackageLink { package, filename } = link {
        let DescriptionFile {
            package: candidate_package,
            absolute_path,
        } = file;

        if package != candidate_package {
            return false;
        };
        let pathbuf = PathBuf::from(absolute_path);
        let stripped_filename = pathbuf
            .file_name()
            .expect(&format!("Error accessing file `{absolute_path:?}`."));

        stripped_filename.to_str().is_some_and(|s| s == filename)
    } else {
        false
    }
}
