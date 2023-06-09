#![cfg(test)]
extern crate std;
use super::*;

use soroban_sdk::{testutils::Address as _, testutils::Logger, Address, Env, IntoVal, Symbol};

#[test]
fn test_ticket_store_contract() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicketContract);
    let client = TicketContractClient::new(&env, &contract_id);

    let event_organizer = Address::random(&env); // Create only once

    let event1 = Symbol::short("EVT123");

    // Test creating a new event
    log!(&env, "TEST: Creating a new event");
    client.create(&event_organizer, &event1, &10);
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            event_organizer.clone(),
            contract_id.clone(),
            Symbol::short("create"),
            (event_organizer.clone(), event1.clone(), 10_u32).into_val(&env)
        )]
    );

    let user_1 = Address::random(&env);

    // Test issuing a ticket and a stub
    log!(&env, "TEST: Issuing a ticket and a stub");
    client.issue(&event_organizer, &user_1, &event1);
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            event_organizer.clone(),
            contract_id.clone(),
            Symbol::short("issue"),
            (event_organizer.clone(), user_1.clone(), event1.clone()).into_val(&env)
        )]
    );

    // Test checking if a user has a ticket
    let has_ticket = client.has_ticket(&user_1, &event1);
    assert!(has_ticket);
    log!(&env, "TEST: Checking if a user has a ticket, result: {}", has_ticket);

    std::println!("{}", env.logger().all().join("\n"));
}
