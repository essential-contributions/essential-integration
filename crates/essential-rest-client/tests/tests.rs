use std::vec;

use essential_rest_client::EssentialClient;
use essential_server_types::{QueryStateReads, QueryStateReadsOutput, Slots, SolutionOutcome};
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    PredicateAddress,
};
use utils::compile_pint_project;

mod utils;

#[tokio::test]
async fn test_api() {
    let contract = compile_pint_project(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../apps/counter/pint/pint.toml"
        )
        .into(),
    )
    .await
    .unwrap();

    let predicate_address = PredicateAddress {
        contract: essential_hash::contract_addr::from_contract(&contract),
        predicate: essential_hash::content_addr(&contract.predicates[0]),
    };
    let solution = Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate_address.clone(),
            decision_variables: Default::default(),
            transient_data: Default::default(),
            state_mutations: vec![Mutation {
                key: vec![0],
                value: vec![1],
            }],
        }],
    };

    let mut wallet = essential_wallet::Wallet::temp().unwrap();
    wallet
        .new_key_pair("alice", essential_wallet::Scheme::Secp256k1)
        .unwrap();

    let signed_contract = wallet.sign_contract(contract.clone(), "alice").unwrap();

    let (addr, _server) = utils::setup_server().await.unwrap();

    let client = EssentialClient::new(addr).unwrap();
    let r = client
        .deploy_contract(signed_contract.clone())
        .await
        .unwrap();
    assert_eq!(r, predicate_address.contract);

    let r = client
        .check_solution_with_contracts(solution.clone(), vec![contract.clone()])
        .await
        .unwrap();
    assert_eq!(r.utility, 1.0);

    let r = client.check_solution(solution.clone()).await.unwrap();
    assert_eq!(r.utility, 1.0);

    let r = client.submit_solution(solution.clone()).await.unwrap();
    assert_eq!(r, essential_hash::content_addr(&solution));

    loop {
        let r = client
            .query_state(&predicate_address.contract, &vec![0])
            .await
            .unwrap();
        if let [count] = r.as_slice() {
            if *count == 1 {
                break;
            }
        }
    }

    let r = client
        .solution_outcome(&essential_hash::hash(&solution))
        .await
        .unwrap();
    assert_eq!(r, vec![SolutionOutcome::Success(0)]);

    let r = client
        .get_predicate(&predicate_address)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(r, contract.predicates[0]);

    let r = client
        .get_contract(&predicate_address.contract)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(r, signed_contract);

    let r = client.list_contracts(None, None).await.unwrap();
    assert_eq!(r[1], contract);

    let r = client.list_solutions_pool(None).await.unwrap();
    assert!(r.is_empty());

    let r = client.list_blocks(None, None).await.unwrap();
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].number, 0);
    assert_eq!(r[0].solutions.len(), 1);
    assert_eq!(r[0].solutions[0], solution.clone());

    let query = QueryStateReads::from_solution(
        solution.clone(),
        0,
        &contract.predicates[0],
        Default::default(),
    );

    let r = client.query_state_reads(query).await.unwrap();
    assert_eq!(
        r,
        QueryStateReadsOutput::All(
            [(
                predicate_address.contract.clone(),
                [(vec![0], vec![1])].into_iter().collect()
            )]
            .into_iter()
            .collect(),
            Slots {
                pre: vec![vec![1]],
                post: vec![vec![1]]
            }
        )
    );
}
