/// Groups of repository properties that are expensive enough to compute only on demand.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Needed(u32);

impl Needed {
    pub const NONE: Needed = Needed(0);
    pub const REMOTE: Needed = Needed(1);
    pub const BRANCH: Needed = Needed(1 << 1);
    pub const BRANCH_COUNT: Needed = Needed(1 << 2);
    pub const COMMIT: Needed = Needed(1 << 3);
    pub const COMMIT_COUNT: Needed = Needed(1 << 4);
    pub const DIRTY: Needed = Needed(1 << 5);
    pub const SIZE: Needed = Needed(1 << 6);
    pub const LANGUAGE: Needed = Needed(1 << 7);
    pub const TRACKING: Needed = Needed(1 << 8);
    pub const STATE: Needed = Needed(1 << 9);
    pub const COUNTS: Needed = Needed(1 << 10);
    pub const USER: Needed = Needed(1 << 11);
    pub const SUMMARY: Needed = Needed(1 << 12);

    pub fn has(self, other: Needed) -> bool {
        self.0 & other.0 != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn without(self, other: Needed) -> Needed {
        Needed(self.0 & !other.0)
    }

    /// Flag for a placeholder tag as captured by the placeholder regex (without `{_` / `_}`).
    pub fn for_tag(tag: &str) -> Needed {
        match tag {
            "remote:n" | "remote:f" | "remote:p" => Self::REMOTE,
            "branch:n" => Self::BRANCH,
            "branch:c" => Self::BRANCH_COUNT,
            "hash" | "hash:f" | "author:e" | "author:n" | "time:r" | "time:rc" | "time:a" => {
                Self::COMMIT
            }
            "commit:c" => Self::COMMIT_COUNT,
            "dirty" => Self::DIRTY,
            "size" => Self::SIZE,
            "language" => Self::LANGUAGE,
            "upstream" | "ahead" | "behind" => Self::TRACKING,
            "state" | "shallow" => Self::STATE,
            "remote:c" | "tag:c" | "worktree:c" | "stash:c" => Self::COUNTS,
            "user:n" | "user:e" => Self::USER,
            "commit:s" => Self::SUMMARY,
            _ => Self::NONE,
        }
    }
}

impl std::ops::BitOr for Needed {
    type Output = Needed;

    fn bitor(self, rhs: Needed) -> Needed {
        Needed(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Needed {
    fn bitor_assign(&mut self, rhs: Needed) {
        self.0 |= rhs.0;
    }
}
