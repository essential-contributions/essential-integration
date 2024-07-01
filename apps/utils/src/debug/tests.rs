use essential_types::contract::Contract;

use super::*;

#[test]
fn test_get_source() {
    let code = r#"
signed (path+root=df0ad451fc3c4baa)
interface ::Token {
    storage {
        balances: ( b256 => int ),
        nonce: ( b256 => int ),
        token_name: b256,
        token_symbol: b256,
        decimals: int,
    }
enum ::TransferMode = All | Key | KeyTo | KeyAmount;
type ::std::lib::Secp256k1Signature = {b256, b256, int};
type ::std::lib::Secp256k1PublicKey = {b256, int};

predicate ::Burn {
    constraint (__mut_keys_len() == 0);
}


predicate ::Transfer {
    constraint (__mut_keys_len() == 0);
}


token (essential-integration/apps/token/pint/token)
const ::auth::signed::TransferWith::ADDRESS: b256 = 0x3750D1EE658C1A69072EC71B7C586C29779B4570DB1B19C054A58A9AD5803653;
storage {
    balances: ( b256 => int ),
}
interface ::Auth {
    predicate Predicate {
        pub var addr: {contract: b256, addr: b256};
    }
}
type ::std::lib::PredicateAddress = {contract: b256, addr: b256};

predicate ::Burn {
    constraint ((::A::addr.contract == __this_set_address()) && (::A::addr.addr == __this_address()));
}

predicate ::Cancel {
    constraint ((::A::addr.contract == __this_set_address()) && (::A::addr.addr == __this_address()));
}

predicate ::Transfer {
    storage {
        balances: ( b256 => int ),
    }
    interface ::Auth {
        predicate Predicate {
            pub var addr: {contract: b256, addr: b256};
        }
    }
    pub var ::amount: int;
    constraint (__mut_keys_len() == 3);
    constraint ((::A::addr.contract == __this_set_address()) && (::A::addr.addr == __this_address()));
    constraint (((__state_len(::nonce) == 0) && (::nonce' == 1)) || ((::nonce' - ::nonce) == 1));
}

predicate ::Mint {
    constraint (__mut_keys_len() == 5);
}
    "#;

    let other = r#"const ::auth::signed::TransferWith::ADDRESS: b256 = 0x3750D1EE658C1A69072EC71B7C586C29779B4570DB1B19C054A58A9AD5803653;
storage {
    balances: ( b256 => int ),
}
interface ::Auth {
    predicate Predicate {
        pub var addr: {contract: b256, addr: b256};
    }
}
type ::std::lib::PredicateAddress = {contract: b256, addr: b256};

"#;
    let predicate = r#"predicate ::Transfer {
    storage {
        balances: ( b256 => int ),
    }
    interface ::Auth {
        predicate Predicate {
            pub var addr: {contract: b256, addr: b256};
        }
    }
    pub var ::amount: int;
    constraint (__mut_keys_len() == 3);
    constraint ((::A::addr.contract == __this_set_address()) && (::A::addr.addr == __this_address()));
    constraint (((__state_len(::nonce) == 0) && (::nonce' == 1)) || ((::nonce' - ::nonce) == 1));
}

"#;
    let constraint_line = Some(11);

    let contract = NamedContract {
        name: "token".to_string(),
        contract: Contract::default(),
        predicates: vec![],
        source: code.to_string(),
    };

    let source = get_source(&contract, "transfer", 1);
    assert_eq!(source.other, other);
    assert_eq!(source.predicate, predicate);
    assert_eq!(source.constraint_line, constraint_line);
}
