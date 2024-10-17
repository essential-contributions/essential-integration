use essential_builder::{error::BuildBlockError, SolutionsSummary};
use essential_types::{solution::Solution, ContentAddress};

use crate::db::Dbs;

pub async fn build_default(dbs: &Dbs) -> Result<SolutionsSummary, BuildBlockError> {
    essential_builder::build_block_fifo(&dbs.builder, &dbs.node, &Default::default()).await
}

pub async fn submit(
    builder: &essential_builder_db::ConnectionPool,
    solution: Solution,
) -> anyhow::Result<ContentAddress> {
    Ok(builder
        .insert_solution_submission(
            std::sync::Arc::new(solution),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await?)
}
