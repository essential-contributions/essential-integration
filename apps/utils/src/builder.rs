use essential_builder::{error::BuildBlockError, SolutionSetsSummary};
use essential_types::{solution::SolutionSet, ContentAddress};

use crate::db::Dbs;

pub async fn build_default(dbs: &Dbs) -> Result<SolutionSetsSummary, BuildBlockError> {
    essential_builder::build_block_fifo(&dbs.builder, &dbs.node, &Default::default())
        .await
        .map(|(_, o)| o)
}

pub async fn submit(
    builder: &essential_builder_db::ConnectionPool,
    solution_set: SolutionSet,
) -> anyhow::Result<ContentAddress> {
    Ok(builder
        .insert_solution_set_submission(
            std::sync::Arc::new(solution_set),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
        )
        .await?)
}
