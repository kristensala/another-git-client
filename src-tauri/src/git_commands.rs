pub enum GitCommand {
    Log,
    Merge,
    Pull,
    Push,
    Commit,
    Checkout,
    Branch
}

pub struct GitAction {
    command: GitCommand,
    working_branch: String,
    target_branch: String
}


