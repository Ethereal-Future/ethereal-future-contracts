#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub struct Project { pub owner: Address, pub name: String, pub goal: i128, pub raised: i128, pub active: bool }

#[derive(Clone)]
#[contracttype]
pub struct Milestone { pub title: String, pub complete: bool }

#[derive(Clone)]
#[contracttype]
pub enum DataKey { Project(u32), Milestones(u32), NextProject }

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
        env.storage().instance().set(&DataKey::Milestones(id), &Vec::<Milestone>::new(&env));
        env.storage().instance().set(&DataKey::NextProject, &(id + 1));
        env.events().publish((symbol_short!("project"), id), project);
        id
    }

    pub fn get(env: Env, project_id: u32) -> Project { env.storage().instance().get(&DataKey::Project(project_id)).unwrap() }

    pub fn get_milestones(env: Env, project_id: u32) -> Vec<Milestone> { env.storage().instance().get(&DataKey::Milestones(project_id)).unwrap_or(Vec::new(&env)) }

    pub fn update_project(env: Env, project_id: u32, name: String, goal: i128) {
        if goal <= 0 { panic!() }
        let mut project: Project = Self::get(env.clone(), project_id);
        project.owner.require_auth();
        if goal < project.raised { panic!() }
        project.name = name;
        project.goal = goal;
        env.storage().instance().set(&DataKey::Project(project_id), &project);
        env.events().publish((symbol_short!("updated"), project_id), project);
    }

    pub fn add_milestone(env: Env, project_id: u32, title: String) -> u32 {
        let project: Project = Self::get(env.clone(), project_id);
        project.owner.require_auth();
        let mut milestones = Self::get_milestones(env.clone(), project_id);
        let id = milestones.len();
        milestones.push_back(Milestone { title, complete: false });
        env.storage().instance().set(&DataKey::Milestones(project_id), &milestones);
        env.events().publish((symbol_short!("milestone"), project_id, id), false);
        id
    }

    pub fn complete_milestone(env: Env, project_id: u32, milestone_id: u32) {
        let project: Project = Self::get(env.clone(), project_id);
        project.owner.require_auth();
        let mut milestones = Self::get_milestones(env.clone(), project_id);
        let mut milestone = milestones.get(milestone_id).unwrap();
        milestone.complete = true;
        milestones.set(milestone_id, milestone);
        env.storage().instance().set(&DataKey::Milestones(project_id), &milestones);
        env.events().publish((symbol_short!("complete"), project_id, milestone_id), true);
    }

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
