// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use lazy_static;
use libra_metrics::{Histogram, IntGauge, OpMetrics};
use prometheus::{HistogramVec, IntCounterVec, IntGaugeVec};

lazy_static::lazy_static! {
    pub static ref LIBRA_NETWORK_PEERS: IntGaugeVec = register_int_gauge_vec!(
        // metric name
        "libra_network_peers",
        // metric description
        "Libra network peers counter",
        // metric labels (dimensions)
        &["role_type", "state"]
    ).unwrap();

    pub static ref LIBRA_NETWORK_DISCOVERY_NOTES: IntGaugeVec = register_int_gauge_vec!(
        // metric name
        "libra_network_discovery_notes",
        // metric description
        "Libra network discovery notes",
        // metric labels (dimensions)
        &["role_type"]
    ).unwrap();

    pub static ref LIBRA_NETWORK_RPC_MESSAGES: IntCounterVec = register_int_counter_vec!(
        "libra_network_rpc_messages",
        "Libra network rpc messages counter",
        &["type", "state"]
    ).unwrap();

    pub static ref LIBRA_NETWORK_RPC_BYTES: HistogramVec = register_histogram_vec!(
        "libra_network_rpc_bytes",
        "Libra network rpc bytes histogram",
        &["type", "state"]
    ).unwrap();

    pub static ref LIBRA_NETWORK_RPC_LATENCY: Histogram = register_histogram!(
        "libra_network_rpc_latency_seconds",
        "Libra network rpc latency histogram"
    ).unwrap();

    pub static ref LIBRA_NETWORK_DIRECT_SEND_MESSAGES: IntCounterVec = register_int_counter_vec!(
        "libra_network_direct_send_messages",
        "Libra network direct send messages counter",
        &["state"]
    ).unwrap();

    pub static ref LIBRA_NETWORK_DIRECT_SEND_BYTES: HistogramVec = register_histogram_vec!(
        "libra_network_direct_send_bytes",
        "Libra network direct send bytes histogram",
        &["state"]
    ).unwrap();

    /// Counters(queued,dequeued,dropped) related to inbound network notifications for RPCs and
    /// DirectSends.
    pub static ref PENDING_NETWORK_NOTIFICATIONS: IntCounterVec = register_int_counter_vec!(
        "libra_network_pending_network_notifications",
        "Counters(queued,dequeued,dropped) related to pending inbound network notifications",
        &["state"]
    ).unwrap();

    /// Counter of pending requests in Network Provider
    pub static ref PENDING_NETWORK_REQUESTS: IntCounterVec = register_int_counter_vec!(
        "libra_pending_network_requests",
        "Counters(queued,dequeued,dropped) related to pending outbound network requests",
        &["state"]
    ).unwrap();

    /// Counter of pending network events to Consensus
    pub static ref PENDING_CONSENSUS_NETWORK_EVENTS: IntCounterVec = register_int_counter_vec!(
        "pending_consensus_network_events",
        "Pending network event notifications to Consensus",
        &[]
    ).unwrap();

    /// Counter of pending network events to Mempool
    pub static ref PENDING_MEMPOOL_NETWORK_EVENTS: IntCounterVec = register_int_counter_vec!(
        "pending_mempool_network_events",
        "Pending network event notifications to Mempool",
        &[]
    ).unwrap();

    /// Counter of pending network events to State Synchronizer
    pub static ref PENDING_STATE_SYNCHRONIZER_NETWORK_EVENTS: IntCounterVec = register_int_counter_vec!(
        "pending_state_sync_network_events",
        "Pending network event notifications to State Synchronizer",
        &[]
    ).unwrap();

    /// Counter of pending network events to Admission Control
    pub static ref PENDING_ADMISSION_CONTROL_NETWORK_EVENTS: IntCounterVec = register_int_counter_vec!(
        "pending_admission_control_network_events",
        "Pending network event notifications to Admission Control",
        &[]
    ).unwrap();

    /// Counter of pending network events to Health Checker.
    pub static ref PENDING_HEALTH_CHECKER_NETWORK_EVENTS: IntCounterVec = register_int_counter_vec!(
        "pending_health_checker_network_events",
        "Pending network event notifications to HealthChecker",
        &[]
    ).unwrap();

    /// Counter of pending network events to Discovery.
    pub static ref PENDING_DISCOVERY_NETWORK_EVENTS: IntCounterVec = register_int_counter_vec!(
        "pending_discovery_network_events",
        "Pending network event notifications to Mempool",
        &[]
    ).unwrap();

    /// Counter of pending requests in Peer Manager
    pub static ref PENDING_PEER_MANAGER_REQUESTS: IntCounterVec = register_int_counter_vec!(
        "pending_peer_manager_requests",
        "Pending requests to PeerManager",
        &[]
    ).unwrap();

}

lazy_static::lazy_static! {
    pub static ref OP_COUNTERS: OpMetrics = OpMetrics::new_and_registered("network");
}

lazy_static::lazy_static! {
    ///
    /// Channel Counters
    ///

    /// Counter of pending requests in Connectivity Manager
    pub static ref PENDING_CONNECTIVITY_MANAGER_REQUESTS: IntGauge = OP_COUNTERS.gauge("pending_connectivity_manager_requests");

    /// Counter of pending Connection Handler notifications to PeerManager.
    pub static ref PENDING_CONNECTION_HANDLER_NOTIFICATIONS: IntGauge = OP_COUNTERS.gauge("pending_connection_handler_notifications");

    /// Counter of pending dial requests in Peer Manager
    pub static ref PENDING_PEER_MANAGER_DIAL_REQUESTS: IntGauge  = OP_COUNTERS.gauge("pending_peer_manager_dial_requests");

    /// Counter of pending requests in Direct Send
    pub static ref PENDING_DIRECT_SEND_REQUESTS: &'static str = "pending_direct_send_requests";

    /// Counter of pending Direct Send notifications to Network Provider
    pub static ref PENDING_DIRECT_SEND_NOTIFICATIONS: &'static str = "pending_direct_send_notifications";

    /// Counter of pending requests in RPC
    pub static ref PENDING_RPC_REQUESTS: &'static str = "pending_rpc_requests";

    /// Counter of pending RPC notifications to Network Provider
    pub static ref PENDING_RPC_NOTIFICATIONS: &'static str = "pending_rpc_notifications";

    /// Counter of pending requests for each remote peer
    pub static ref PENDING_PEER_REQUESTS: &'static str = "pending_peer_requests";

    /// Counter of pending outbound messages in Direct Send for each remote peer
    pub static ref PENDING_DIRECT_SEND_OUTBOUND_MESSAGES: &'static str = "pending_direct_send_outbound_messages";

    /// Counter of pending RPC events from Peer to Rpc actor.
    pub static ref PENDING_PEER_RPC_NOTIFICATIONS: &'static str = "pending_peer_rpc_notifications";

    /// Counter of pending DirectSend events from Peer to DirectSend actor..
    pub static ref PENDING_PEER_DIRECT_SEND_NOTIFICATIONS: &'static str = "pending_peer_direct_send_notifications";

    /// Counter of pending connection notifications from Peer to NetworkProvider.
    pub static ref PENDING_PEER_NETWORK_NOTIFICATIONS: &'static str = "pending_peer_network_notifications";
}
