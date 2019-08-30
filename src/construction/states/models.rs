use super::*;
use crate::construction::states::route::RouteState;
use crate::models::common::Cost;
use crate::models::problem::Job;
use crate::models::solution::{Activity, Registry, Route};
use crate::models::{Problem, Solution};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Provides the way to get some meta information about insertion progress.
pub struct InsertionProgress {
    /// Specifies best known cost depending on context.
    pub cost: Cost,

    /// Specifies solution completeness.
    pub completeness: f64,

    /// Total amount of jobs.
    pub total: usize,
}

/// Specifies insertion context for activity.
pub struct ActivityContext {
    /// Activity insertion index.
    pub index: usize,

    /// Previous activity.
    pub prev: Activity,

    /// Target activity.
    pub target: Activity,

    /// Next activity. Absent if tour is open and target activity inserted last.
    pub next: Option<Activity>,
}

/// Specifies insertion context for route.
pub struct RouteContext {
    /// Used route.
    pub route: Arc<Route>,

    /// Insertion state.
    pub state: Arc<RouteState>,
}

/// Contains information needed to performed insertions in solution.
pub struct InsertionContext {
    /// Solution progress.
    pub progress: InsertionProgress,

    /// Original problem.
    pub problem: Arc<Problem>,

    /// Solution context.
    pub solution: Arc<Solution>,

    /// Random generator.
    pub random: Arc<String>,
}

/// Contains information regarding insertion solution.
pub struct SolutionContext {
    /// List of jobs which require permanent assignment.
    pub required: Vec<Arc<Job>>,

    /// List of jobs which at the moment does not require assignment and might be ignored.
    pub ignored: Vec<Arc<Job>>,

    /// Map of jobs which cannot be assigned and within reason code.
    pub unassigned: HashMap<Arc<Job>, i32>,

    // TODO implement proper hash function for RouteContext
    /// Set of routes within their state.
    pub routes: HashSet<RouteContext>,

    /// Keeps track of used resources.
    pub registry: Arc<Registry>,
}