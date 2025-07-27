mod git_helpers;
mod versioning;

use std::collections::HashSet;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use git2::Repository;

use crate::{
    git_helpers::{get_current_branch, git_pull, git_push, git_tag},
    versioning::{Part, ScopedTag, group_tags_by_scope},
};

#[derive(Parser)]
#[command(version = env!("VERGEN_GIT_DESCRIBE"), about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
    #[arg(
        long,
        default_value_t = false,
        help = "Whether to pull new commits / tags before any action"
    )]
    no_pull: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List releases")]
    List {
        #[arg(help = "Scope to consider")]
        scope: Option<String>,
        #[arg(long)]
        latest: bool,
        #[arg(long)]
        all_scopes: bool,
    },
    #[command(about = "Bump version")]
    Bump {
        #[arg(help = "Scope to consider")]
        scope: Option<String>,
        #[arg(short, long, value_enum, default_value_t=Part::Patch)]
        part: Part,
        #[arg(long)]
        push: bool,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        allow_non_main: bool,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.no_pull {
        git_pull().context("Unable to pull from remote")?
    }

    let main_branches: HashSet<&str> = ["main", "master"].iter().cloned().collect();
    let repo = Repository::discover(".").context("No git repository found")?;

    match args.cmd {
        Commands::List {
            scope,
            latest,
            all_scopes,
        } => {
            let tags = repo.tag_names(None)?;
            let mut scope_tag_map = group_tags_by_scope(&tags);
            if !all_scopes {
                scope_tag_map.retain(|k, _| k == &scope);
            }
            for versioned_tags in scope_tag_map.values() {
                if latest && !versioned_tags.is_empty() {
                    if let Some(latest_tag) = versioned_tags.values().next_back() {
                        println!("{latest_tag}");
                    }
                } else {
                    for tag in versioned_tags.values() {
                        println!("{tag}");
                    }
                }
            }
        }
        Commands::Bump {
            scope,
            part,
            push,
            dry_run,
            allow_non_main,
        } => {
            let current_branch = get_current_branch()?;
            if !allow_non_main && !main_branches.contains(current_branch.as_str()) {
                return Err(anyhow::anyhow!(
                    "You must be on a main branch (main/master), currently on '{}'",
                    current_branch
                ));
            }
            let tags = repo.tag_names(None)?;
            let scope_tag_map = group_tags_by_scope(&tags);
            let versioned_tags = scope_tag_map.get(&scope).context(match &scope {
                Some(s) => format!("No tags found for scope '{s}'"),
                None => "No unscoped tags found".to_string(),
            })?;
            let latest_tag = versioned_tags.values().next_back().context(match &scope {
                Some(s) => format!("No tags found for scope '{s}'"),
                None => "No unscoped tags found".to_string(),
            })?;
            let latest_scoped_tag = ScopedTag::parse(latest_tag).context(format!(
                "Unable to parse tag '{latest_tag}' as a valid version"
            ))?;
            let new_scoped_tag = latest_scoped_tag.bump(part);
            println!("Bumping {latest_scoped_tag} -> {new_scoped_tag}");
            if !dry_run {
                git_tag(&new_scoped_tag.to_string())?;
                if push {
                    git_push()?;
                }
            }
        }
    }
    Ok(())
}
