#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol, Address, contracttype};

#[contracttype]
pub enum DataKey {
    Ticket(Address, Symbol),
    Stub(Address, Symbol),
    EventTotalTickets(Symbol),
}

pub struct TicketContract;

#[contractimpl]
impl TicketContract {
    /// Creates a new event.
    pub fn create(env: Env, event_organizer: Address, event_name: Symbol, total_tickets: u32) {
        event_organizer.require_auth();
        let event_key = DataKey::EventTotalTickets(event_name.clone());
        env.storage().set(&event_key, &total_tickets);
        log!(&env, "Created event: {} with tickets: {} by organizer: {}", event_name, total_tickets, event_organizer);
    }
    
    /// Issues a ticket and a stub to a given user for a specific event.
    pub fn issue(env: Env, event_organizer: Address, user: Address, event: Symbol) {
        event_organizer.require_auth();

        let event_key = DataKey::EventTotalTickets(event.clone());
        let mut total_tickets: u32 = env
            .storage()
            .get(&event_key)
            .unwrap_or(Ok(0)) // If no event, assume zero tickets.
            .unwrap(); // Panic if the value is not u32.

        if total_tickets > 0 {
            total_tickets -= 1;
            env.storage().set(&event_key, &total_tickets);

            log!(&env, "Issuing ticket and stub for event: {} to user: {} by organizer: {}", event, user, event_organizer);

            let ticket_key = DataKey::Ticket(user.clone(), event.clone());
            let stub_key = DataKey::Stub(user.clone(), event.clone());
            env.storage().set(&ticket_key, &true);
            env.storage().set(&stub_key, &true);
        } else {
            log!(&env, "Failed to issue ticket and stub for event: {} to user: {}. No tickets available.", event, user);
        }
    }

    /// Checks if a given user has a ticket for a specific event.
    pub fn has_ticket(env: Env, user: Address, event: Symbol) -> bool {
        let ticket_key = DataKey::Ticket(user.clone(), event.clone());
        env.storage().get(&ticket_key).unwrap_or(Ok(false)).unwrap()
    }

}

mod test;