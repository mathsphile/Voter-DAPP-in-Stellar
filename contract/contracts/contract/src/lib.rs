#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Address, Map};

#[contract]
pub struct VotingContract;

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub title: Symbol,
    pub vote_count: u32,
}

#[contracttype]
pub enum DataKey {
    Proposal(u32),
    ProposalCount,
    Voted(Address, u32),
}

#[contractimpl]
impl VotingContract {

    // Initialize proposal count
    pub fn init(env: Env) {
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
    }

    // Create a new proposal
    pub fn create_proposal(env: Env, title: Symbol) {
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0);

        count += 1;

        let proposal = Proposal {
            id: count,
            title,
            vote_count: 0,
        };

        env.storage().instance().set(&DataKey::Proposal(count), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &count);
    }

    // Vote for a proposal
    pub fn vote(env: Env, voter: Address, proposal_id: u32) {
        voter.require_auth();

        // Prevent double voting
        if env.storage().instance().has(&DataKey::Voted(voter.clone(), proposal_id)) {
            panic!("Already voted");
        }

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found");

        proposal.vote_count += 1;

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage().instance().set(&DataKey::Voted(voter, proposal_id), &true);
    }

    // Get proposal details
    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found")
    }

    // Get total proposals
    pub fn get_proposal_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }
}