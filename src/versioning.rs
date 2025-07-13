use std::collections::BTreeMap;
use std::{fmt::Display, str::FromStr};

use anyhow::{Context, Error, Result};
use clap::ValueEnum;
use git2::string_array::StringArray;
use semver::{BuildMetadata, Prerelease, Version};

#[derive(Debug, Clone, ValueEnum)]
pub enum Part {
    Major,
    Minor,
    Patch,
}

#[derive(Debug)]
pub struct ScopedTag {
    pub scope: Option<String>,
    pub version: Version,
    sep: Option<char>,
}

impl ScopedTag {
    const POSSIBLE_SEP: &[char] = &['@', '/', '_'];

    pub fn parse(tag_str: &str) -> Result<Self> {
        for &sep in Self::POSSIBLE_SEP {
            if let Some((scope, ver)) = tag_str.rsplit_once(sep) {
                if let Ok(version) = Version::parse(ver) {
                    return Ok(ScopedTag {
                        scope: Some(scope.to_string()),
                        version,
                        sep: Some(sep),
                    });
                }
            };
        }
        let version = Version::parse(tag_str).context("Unable to parse tag")?;
        Ok(ScopedTag {
            scope: None,
            version,
            sep: None,
        })
    }

    pub fn bump(&self, part: Part) -> Self {
        let new_version = match part {
            Part::Major => Version {
                major: self.version.major + 1,
                minor: 0,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            },
            Part::Minor => Version {
                major: self.version.major,
                minor: self.version.minor + 1,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            },
            Part::Patch => Version {
                major: self.version.major,
                minor: self.version.minor,
                patch: self.version.patch + 1,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            },
        };
        ScopedTag {
            scope: self.scope.clone(),
            version: new_version,
            sep: self.sep,
        }
    }
}

impl FromStr for ScopedTag {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Display for ScopedTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(scope), Some(sep)) = (&self.scope, self.sep) {
            write!(f, "{}{}{}", scope, sep, self.version)
        } else {
            write!(f, "{}", self.version)
        }
    }
}

pub fn group_tags_by_scope(
    tags: &StringArray,
) -> BTreeMap<Option<String>, BTreeMap<Version, String>> {
    let scoped_tags = tags
        .iter()
        .flatten()
        .filter_map(|tag| ScopedTag::parse(tag).ok());
    let mut scope_tag_map: BTreeMap<Option<String>, BTreeMap<Version, String>> = BTreeMap::new();
    for tag in scoped_tags {
        let entry = scope_tag_map.entry(tag.scope.clone()).or_default();
        entry.insert(tag.version.clone(), tag.to_string());
    }
    scope_tag_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoped_tag_parse_unscoped() {
        let tag = ScopedTag::parse("1.0.0").unwrap();
        assert_eq!(tag.scope, None);
        assert_eq!(tag.version.major, 1);
        assert_eq!(tag.version.minor, 0);
        assert_eq!(tag.version.patch, 0);
    }

    #[test]
    fn test_scoped_tag_parse_scoped() {
        let tag = ScopedTag::parse("backend/1.2.3").unwrap();
        assert_eq!(tag.scope, Some("backend".to_string()));
        assert_eq!(tag.version.major, 1);
        assert_eq!(tag.version.minor, 2);
        assert_eq!(tag.version.patch, 3);
    }

    #[test]
    fn test_scoped_tag_parse_invalid() {
        assert!(ScopedTag::parse("invalid").is_err());
        assert!(ScopedTag::parse("scope/invalid").is_err());
    }

    #[test]
    fn test_scoped_tag_bump_major() {
        let tag = ScopedTag::parse("1.2.3").unwrap();
        let bumped = tag.bump(Part::Major);
        assert_eq!(bumped.version.major, 2);
        assert_eq!(bumped.version.minor, 0);
        assert_eq!(bumped.version.patch, 0);
    }

    #[test]
    fn test_scoped_tag_bump_minor() {
        let tag = ScopedTag::parse("1.2.3").unwrap();
        let bumped = tag.bump(Part::Minor);
        assert_eq!(bumped.version.major, 1);
        assert_eq!(bumped.version.minor, 3);
        assert_eq!(bumped.version.patch, 0);
    }

    #[test]
    fn test_scoped_tag_bump_patch() {
        let tag = ScopedTag::parse("1.2.3").unwrap();
        let bumped = tag.bump(Part::Patch);
        assert_eq!(bumped.version.major, 1);
        assert_eq!(bumped.version.minor, 2);
        assert_eq!(bumped.version.patch, 4);
    }

    #[test]
    fn test_scoped_tag_display_unscoped() {
        let tag = ScopedTag::parse("1.0.0").unwrap();
        assert_eq!(tag.to_string(), "1.0.0");
    }

    #[test]
    fn test_scoped_tag_display_scoped() {
        let tag = ScopedTag::parse("backend/1.2.3").unwrap();
        assert_eq!(tag.to_string(), "backend/1.2.3");
    }

    #[test]
    fn test_parse_slash_sep() {
        let tag = "scope/subscope/0.1.2";
        let scoped_tag = ScopedTag::parse(tag).unwrap();
        assert_eq!(scoped_tag.sep, Some('/'))
    }

    #[test]
    fn test_parse_at_sep() {
        let tag = "scope/subscope@0.1.2";
        let scoped_tag = ScopedTag::parse(tag).unwrap();
        assert_eq!(scoped_tag.sep, Some('@'))
    }

    #[test]
    fn test_parse_underscore_sep() {
        let tag = "scope/subscope_0.1.2";
        let scoped_tag = ScopedTag::parse(tag).unwrap();
        assert_eq!(scoped_tag.sep, Some('_'))
    }

    #[test]
    #[should_panic]
    fn test_unparsable_tag() {
        let tag = "scope/subscope/nothing";
        let _scoped_tag = ScopedTag::parse(tag).unwrap();
    }
}
