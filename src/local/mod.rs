//! The local cluster that hosts the runtime
//!
//! `nsx agent run` brings the runtime up after the code review and takes it
//! down after Safe Seat is met. Up means a namespace of its own for this run
//! in the machine's shared kind cluster, with one one-replica Deployment per
//! component of the applied `runtime.md` revision, each at exactly its
//! proposed share. Down means the Deployments and the namespace are deleted
//! and the namespace's absence is confirmed, so that "nothing left" is true.
//! The person's own kubeconfig is never touched.
//!
//! - `Cluster`: the cluster, as a trait
//! - `Kind`: the cluster on the person's machine
//! - `Deployment`: one running component
//! - `Execution`: the execution ledger of what the run created
//! - `State`: the runtime, ready or stopped
//! - `Error`: what can go wrong here
//!
//! ## Rules
//!
//! - The run's order: `codeReview`, then `runtimeReady`, then `completed`,
//!   `pushed`, `pullRequest`, `checks`, `checksPassed` and `safeSeatMet`,
//!   then `runtimeStopped`, then `ready`.
//! - No runtime is up while the stages run or while the run waits for a
//!   person.
//! - Any stop after the bring-up started, an interrupt included, takes
//!   everything down without a line.
//! - A run paused at the gate never touches the cluster.
//! - The kind cluster is shared and stays for the next run: the owner's
//!   decision of 2026-11-11, as the Microruntimes page says. What a run
//!   creates in it is its namespace and the Deployments inside.
//! - Images are resolved by spec 0066 and never built here. Nothing here
//!   measures cpu or memory.
//!
//! ## Open
//!
//! - What uses the runtime between ready and stopped. No line shows anything
//!   reaching it. Until the owner decides, nothing in nsx uses it.
//!
//! Spec 0081.

pub mod cluster;
pub mod deployment;
pub mod error;
pub mod execution;
pub mod kind;
pub mod state;
