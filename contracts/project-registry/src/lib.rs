#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub struct Project { pub owner: Address, pub name: String, pub goal: i128, pub raised: i128, pub active: bool }

#[derive(Clone)]
#[contracttype]
pub enum DataKey { Project(u32), NextProject }

#[contract]
pub struct ProjectRegistry;

#[contractimpl]
impl ProjectRegistry {
    pub fn register(env: Env, owner: Address, name: String, goal: i128) -> u32 {
        owner.require_auth();
        if goal <= 0 { panic!() }
        let id: u32 = env.storage().instance().get(&DataKey::NextProject).unwrap_or(0);
        let project = Project { owner: owner.clone(), name, goal, raised: 0, active: true };
        env.storage().instance().set(&DataKey::Project(id), &project);
        env.storage().instance().set(&DataKey::NextProject, &(id + 1));
        env.events().publish((symbol_short!("project"), id), project);
        id
    }

    pub fn get(env: Env, project_id: u32) -> Project { env.storage().instance().get(&DataKey::Project(project_id)).unwrap() }

    pub fn set_active(env: Env, project_id: u32, active: bool) {
        let mut project: Project = Self::get(env.clone(), project_id);
        project.owner.require_auth();
        project.active = active;
        env.storage().instance().set(&DataKey::Project(project_id), &project);
        env.events().publish((symbol_short!("status"), project_id), active);
    }

    pub fn record_contribution(env: Env, project_id: u32, contributor: Address, amount: i128) {
        contributor.require_auth();
        if amount <= 0 { panic!() }
        let mut project: Project = Self::get(env.clone(), project_id);
        if !project.active { panic!() }
        project.raised = project.raised.checked_add(amount).unwrap();
        env.storage().instance().set(&DataKey::Project(project_id), &project);
        env.events().publish((symbol_short!("contribution"), project_id, contributor), amount);
    }
}
