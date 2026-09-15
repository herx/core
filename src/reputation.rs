use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerReputation {
    pub peer_id: String,
    pub score: f64,
    pub successful_requests: u64,
    pub flagged_infractions: u64,
}

impl PeerReputation {
    pub fn new(peer_id: &str) -> Self {
        Self {
            peer_id: peer_id.to_string(),
            score: 100.0, // Baseline trust score
            successful_requests: 0,
            flagged_infractions: 0,
        }
    }

    /// Applies reputation decay based on a verified infraction,
    /// factoring in an anti-collusion adjustment to mitigate coordinated false reporting.
    pub fn apply_infraction(&mut self, is_coordinated_suspected: bool) {
        let penalty = if is_coordinated_suspected { 5.0 } else { 25.0 };
        self.flagged_infractions += 1;
        self.score = (self.score - penalty).max(0.0);
    }

    pub fn record_success(&mut self) {
        self.successful_requests += 1;
        // Minor natural trust compounding
        self.score = (self.score + 0.5).min(100.0);
    }
}

pub fn evaluate_mesh_trust(reputation: &PeerReputation) -> bool {
    // Threshold for node interaction eligibility
    reputation.score >= 40.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_reputation_state() {
        let rep = PeerReputation::new("node_alpha_99");
        assert_eq!(rep.score, 100.0);
        assert!(evaluate_mesh_trust(&rep));
    }

    #[test]
    fn test_infraction_penalty_normal() {
        let mut rep = PeerReputation::new("node_beta_88");
        rep.apply_infraction(false);
        assert_eq!(rep.score, 75.0);
        assert!(evaluate_mesh_trust(&rep));
    }

    #[test]
    fn test_collusion_resistant_penalty() {
        let mut rep = PeerReputation::new("node_gamma_77");
        // Simulate a suspected coordinated swarm flagging the node
        rep.apply_infraction(true);
        assert_eq!(rep.score, 95.0); // Softened penalty prevents mob-rule censorship
    }

    #[test]
    fn test_trust_threshold_revocation() {
        let mut rep = PeerReputation::new("node_delta_66");
        for _ in 0..3 {
            rep.apply_infraction(false); // Drops score by 75 points total (100 -> 25)
        }
        assert_eq!(rep.score, 25.0);
        assert!(!evaluate_mesh_trust(&rep)); // Falls below eligibility threshold
    }
}
