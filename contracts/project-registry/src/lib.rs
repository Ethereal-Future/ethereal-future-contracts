#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub struct Project {
    pub owner: Address,
    pub name: String,
    pub goal: i128,
    pub raised: i128,
}

#[contracttype]
pub enum DataKey {
    Project,
}

#[contract]
pub struct ProjectRegistry;

#[contractimpl]
impl ProjectRegistry {
    pub fn register(env: Env, owner: Address, name: String, goal: i128) {
        owner.require_auth();
        if goal <= 0 { panic!() }
        if env.storage().instance().has(&DataKey::Project) { panic!() }
        let project = Project { owner: owner.clone(), name, goal, raised: 0 };
        env.storage().instance().set(&DataKey::Project, &project);
        env.events().publish((symbol_short!("project"),), project);
    }

    pub fn get(env: Env) -> Project {
        env.storage().instance().get(&DataKey::Project).unwrap()
    }

    pub fn record_contribution(env: Env, contributor: Address, amount: i128) {
        contributor.require_auth();
        if amount <= 0 { panic!() }
        let mut project: Project = env.storage().instance().get(&DataKey::Project).unwrap();
        project.raised = project.raised.checked_add(amount).unwrap();
        env.storage().instance().set(&DataKey::Project, &project);
        env.events().publish((symbol_short!("contribution"), contributor), amount);
    }
}
