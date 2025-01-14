#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{Env,Address};
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(AuctionContract, ());
    let _client = AuctionContractClient::new(&env, &contract_id);
   
   
}

//#[test]
//fn test_initialize() {
    //// Set up the test environment
    //let env = Env::default();
    ////let contract_id = env.register(AuctionContract, ());
    ////let client = AuctionContractClient::new(&env, &contract_id);

    //let item = Symbol::new(&env,"Test Item");
    //let owner_address = Address::generate(&env);
    //let starting_bid = 100;
    //let duration = 3600; // 1 hour in seconds
                         ////
    //// Call the `initialize` function
    ////let result = client.initialize(&item, &owner_address, &starting_bid, &duration);


    ////// Fetch the auction details after initialization
    ////let auction = client.get_auction_details();
    
    ////// Assertions to check if the auction was initialized correctly
    ////assert_eq!(auction.item, item);
    ////assert_eq!(auction.owner, owner_address);
    ////assert_eq!(auction.highest_bid, starting_bid);
    ////assert_eq!(auction.highest_bidder, None);
    ////assert_eq!(auction.end_time, env.ledger().timestamp() + duration);
    ////assert_eq!(auction.is_active, true);
//}

#[test]
fn test_with_addresses() {
    let env = Env::default();
    
    // Using from_str (preferred)
    let _address1 = Address::from_str(
        &env, 
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );

    // Or using generate for random test addresses
    let _address2 = Address::generate(&env);
}
