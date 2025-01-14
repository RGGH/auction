#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Symbol};

#[contract]
pub struct AuctionContract;

#[contracterror]
#[derive(Debug, PartialEq)]
pub enum AuctionError {
    AuctionEnded = 1,
    BidTooLow = 2,
    NotAuctionOwner = 3,
    AuctionNotEnded = 4,
}

#[contracttype]
pub struct Auction {
    item: Symbol,
    owner: Address,
    highest_bid: u64,
    highest_bidder: Option<Address>,
    end_time: u64,
    is_active: bool, // New field
}

#[contractimpl]
impl AuctionContract {
    pub fn initialize(
        env: Env,
        item: Symbol,
        owner: Address,
        starting_bid: u64,
        duration: u64,
    ) -> Result<(), AuctionError> {
        let current_time = env.ledger().timestamp();
        let auction = Auction {
            item,
            owner,
            highest_bid: starting_bid,
            highest_bidder: None,
            end_time: current_time + duration,
            is_active: true,
        };
        env.storage().instance().set(b"auction", &auction);
        Ok(())
    }

    pub fn bid(env: Env, bidder: Address, amount: u64) -> Result<(), AuctionError> {
        let mut auction: Auction = env.storage().instance().get(b"auction").unwrap();
        let current_time = env.ledger().timestamp();

        if current_time > auction.end_time {
            return Err(AuctionError::AuctionEnded);
        }

        if amount <= auction.highest_bid {
            return Err(AuctionError::BidTooLow);
        }

        auction.highest_bid = amount;
        auction.highest_bidder = Some(bidder.clone());
        env.storage().instance().set(b"auction", &auction);
        Ok(())
    }

    pub fn end_auction(env: Env, caller: Address) -> Result<(Address, u64), AuctionError> {
        let mut auction: Auction = env.storage().instance().get(b"auction").unwrap();
        let current_time = env.ledger().timestamp();

        if caller != auction.owner {
            return Err(AuctionError::NotAuctionOwner);
        }

        if current_time <= auction.end_time {
            return Err(AuctionError::AuctionNotEnded);
        }

        let winner = auction.highest_bidder.clone().unwrap();
        let winning_bid = auction.highest_bid;

        // Auction is ended
        auction.is_active = false;
        env.storage().instance().set(b"auction", &auction);

        Ok((winner, winning_bid))
    }

    pub fn get_auction_details(env: Env) -> Auction {
        env.storage().instance().get(b"auction").unwrap()
    }
}

mod test;
