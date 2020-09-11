use crate::ConnectorResult;
use serde::Deserialize;

/// A timestamp.
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// A unique identifier.
pub type UniqueId = String;

/// Management of imperative migrations state in the database.
#[async_trait::async_trait]
pub trait ImperativeMigrationsPersistence {
    /// Record that a migration is about to be applied.
    async fn start_migration(&self, migration_name: &str, script: &str) -> ConnectorResult<UniqueId>;

    /// Increase the applied_steps_count counter, and append the given logs.
    async fn record_successful_step(&self, id: &UniqueId, logs: &str) -> ConnectorResult<()>;

    /// Report logs for a failed migration step. We assume the next steps in the
    /// migration will not be applied, and the error reported.
    async fn record_failed_step(&self, id: &UniqueId, logs: &str) -> ConnectorResult<()>;

    /// Record that the migration completed *successfully*. This means
    /// populating the `finished_at` field in the migration record.
    async fn record_migration_finished(&self, id: &UniqueId) -> ConnectorResult<()>;

    /// List all applied migrations, ordered by `started_at`.
    async fn list_migrations(&self) -> ConnectorResult<Vec<MigrationRecord>>;
}

/// An applied migration, as returned by list_migrations.
#[derive(Debug, Deserialize)]
pub struct MigrationRecord {
    /// A unique, randomly generated identifier.
    pub id: UniqueId,
    /// The SHA-256 checksum of the migration script, to detect if it was
    /// edited. It covers only the content of the script, it does not include
    /// timestamp or migration name information.
    pub checksum: String,
    /// The timestamp at which the migration completed *successfully*.
    pub finished_at: Option<Timestamp>,
    /// The name of the migration, i.e. the name of migration directory
    /// containing the migration script.
    pub migration_name: String,
    /// The human-readable log of actions performed by the engine, up to and
    /// including the point where the migration failed, with the relevant error.
    ///
    /// Implementation detail note: a tracing collector with specific events in
    /// the database applier.
    pub logs: String,
    /// If the migration was rolled back, and when.
    pub rolled_back_at: Option<Timestamp>,
    /// The time the migration started being applied.
    pub started_at: Timestamp,
    /// The number of migration steps that were successfully applied.
    pub applied_steps_count: u32,
    /// The whole migration script.
    pub script: String,
}
