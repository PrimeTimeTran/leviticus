use cli::{CliCommand, Context};

// =========================
// VIEW COMMANDS
// =========================

pub struct View {
    pub name: String,
}

#[async_trait::async_trait]
impl CliCommand for View {
    async fn run(&self, ctx: &Context) {
        println!("👁️ switching to view: {}", self.name);

        // real logic: set active view
    }
}

// -------------------------

pub struct ViewFork {
    pub name: String,
}
#[async_trait::async_trait]
impl CliCommand for ViewFork {
    async fn run(&self, ctx: &Context) {
        println!("🍴 forking view: {}", self.name);

        // real logic: fork view state
    }
}

// -------------------------

pub struct ViewList;
#[async_trait::async_trait]
impl CliCommand for ViewList {
    async fn run(&self, ctx: &Context) {
        println!("📋 listing views");

        // real logic: list available views
    }
}

// =========================
// EXPLAIN / DOCS COMMANDS
// =========================

pub struct Explain;
#[async_trait::async_trait]
impl CliCommand for Explain {
    async fn run(&self, ctx: &Context) {
        println!("🧠 generating explanation of system state");

        // real logic: dependency + resolution explanation
    }
}

// -------------------------

pub struct ExplainDoc;
#[async_trait::async_trait]
impl CliCommand for ExplainDoc {
    async fn run(&self, ctx: &Context) {
        println!("📄 opening explanation document");

        // real logic: open doc viewer or file
    }
}

pub struct Deps {
    pub name: String,
}
#[async_trait::async_trait]
impl CliCommand for Deps {
    async fn run(&self, ctx: &Context) {
        println!("📄 opening explanation document");
        // real logic: open doc viewer or file
    }
}
