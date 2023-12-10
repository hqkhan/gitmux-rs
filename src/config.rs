// Config is the configuration of the Git status tmux formatter.
#[derive(Debug, Default)]
pub struct Config {
    // Symbols contains the symbols printed before the Git status components.
    symbols: Symbols,
    // Styles contains the tmux style strings for symbols and Git status
    // components.
    styles: Styles,
    // Layout sets the output format of the Git status.
    // Layout []String `yaml:",flow"`
    // Options contains additional configuration options.
    // options: Options
}

#[derive(Debug)]
struct Symbols {
    branch: String,      // Branch is the String shown before local branch name.
    hash_prefix: String, // HasPrefix is the String shown before a SHA1 ref.

    ahead: String,  // Ahead is the String shown before the ahead count for the local/upstream branch divergence.
    behind: String, // Behind is the String shown before the behind count for the local/upstream branch divergence.

    staged: String,    // Staged is the String shown before the count of staged files.
    conflict: String,  // Conflict is the String shown before the count of files with conflicts.
    modified: String,  // Modified is the String shown before the count of modified files.
    untracked: String, // Untracked is the String shown before the count of untracked files.
    stashed: String,   // Stashed is the String shown before the count of stash entries.
    clean: String,     // Clean is the String shown when the working tree is clean.

    insertions: String, // Insertions is the String shown before the count of inserted lines.
    deletions: String,  // Deletions is the String shown before the count of deleted lines.
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            branch: String::from("⎇ "),
            hash_prefix: String::from(":"),
            ahead: String::from("↑·"),
            behind: String::from("↓·"),
            staged: String::from("● "),
            conflict: String::from("✖ "),
            modified: String::from("✚ "),
            untracked: String::from("… "),
            stashed: String::from("⚑ "),
            clean: String::from("✔"),
            insertions: String::from("Σ"),
            deletions: String::from("Δ"),
        }
    }
}

#[derive(Debug)]
struct Styles {
    clear: String, // Clear is the style String that clears all styles.

    state: String,  // State is the style String printed before eventual special state.
    branch: String, // Branch is the style String printed before the local branch.
    remote: String, // Remote is the style String printed before the upstream branch.

    divergence: String, // Divergence is the style String printed before divergence count/symbols.

    staged: String,    // Staged is the style String printed before the staged files count.
    conflict: String,  // Conflict is the style String printed before the conflict count.
    modified: String,  // Modified is the style String printed before the modified files count.
    untracked: String, // Untracked is the style String printed before the untracked files count.
    stashed: String,   // Stashed is the style String printed before the stash entries count.
    clean: String,     // Clean is the style String printed before the clean symbols.

    insertions: String, // Insertions is the style String printed before the count of inserted lines.
    deletions: String,  // Deletions is the style String printed before the count of deleted lines.
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            clear: String::from("#[fg=default]"),
            state: String::from("#[fg=red,bold]"),
            branch: String::from("#[fg=white,bold]"),
            remote: String::from("#[fg=cyan]"),
            divergence: String::from("#[fg=default]"),
            staged: String::from("#[fg=green,bold]"),
            conflict: String::from("#[fg=red,bold]"),
            modified: String::from("#[fg=red,bold]"),
            untracked: String::from("#[fg=magenta,bold]"),
            stashed: String::from("#[fg=cyan,bold]"),
            clean: String::from("#[fg=green,bold]"),
            insertions: String::from("#[fg=green]"),
            deletions: String::from("#[fg=red]"),
        }
    }
}
