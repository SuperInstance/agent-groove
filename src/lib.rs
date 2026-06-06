//! # agent-groove
//!
//! In music, groove is what makes you nod your head. In agent systems, groove is what
//! makes agents productive without burning out — natural timing variation, pocket states
//! for autonomy, swing for contention reduction, and syncopation for creative disruption.
//!
//! Applies musical groove, swing, and pocket concepts to multi-agent work scheduling.

#![forbid(unsafe_code)]

use std::collections::HashMap;

/// Ternary timing offset: Early (-1), OnTime (0), Late (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timing {
    Early = -1,
    OnTime = 0,
    Late = 1,
}

impl Timing {
    pub fn to_i8(self) -> i8 { self as i8 }
    pub fn from_i8(v: i8) -> Option<Self> {
        match v { -1 => Some(Timing::Early), 0 => Some(Timing::OnTime), 1 => Some(Timing::Late), _ => None }
    }
}

/// A groove pattern — micro-timing offsets for each beat in a cycle.
#[derive(Debug, Clone)]
pub struct Groove {
    pub pattern: Vec<Timing>,
    pub position: usize,
}

impl Groove {
    pub fn new(pattern: Vec<Timing>) -> Self { Self { pattern, position: 0 } }
    pub fn straight(len: usize) -> Self { Self::new(vec![Timing::OnTime; len]) }
    pub fn swing(len: usize) -> Self {
        // Classic swing: downbeats on time, upbeats late
        let p: Vec<Timing> = (0..len).map(|i| if i % 2 == 0 { Timing::OnTime } else { Timing::Late }).collect();
        Self::new(p)
    }
    pub fn shuffle(len: usize) -> Self {
        let p: Vec<Timing> = (0..len).map(|i| match i % 4 {
            0 => Timing::Early, 1 => Timing::OnTime, 2 => Timing::Late, _ => Timing::OnTime, _ => Timing::OnTime
        }).collect();
        Self::new(p)
    }
    pub fn tick(&mut self) -> Timing {
        if self.pattern.is_empty() { return Timing::OnTime; }
        let t = self.pattern[self.position];
        self.position = (self.position + 1) % self.pattern.len();
        t
    }
    pub fn reset(&mut self) { self.position = 0; }
    pub fn len(&self) -> usize { self.pattern.len() }
    pub fn is_empty(&self) -> bool { self.pattern.is_empty() }
}

/// Swing scheduler — applies groove timing to agent work cycles.
#[derive(Debug, Clone)]
pub struct SwingScheduler {
    pub groove: Groove,
    pub base_interval_ms: u64,
    pub tick_count: u64,
}

impl SwingScheduler {
    pub fn new(groove: Groove, base_interval_ms: u64) -> Self { Self { groove, base_interval_ms, tick_count: 0 } }
    /// Get the actual wait time for the next tick, adjusted by groove.
    pub fn next_interval(&mut self) -> u64 {
        let timing = self.groove.tick();
        self.tick_count += 1;
        match timing {
            Timing::Early => (self.base_interval_ms as f64 * 0.85) as u64,
            Timing::OnTime => self.base_interval_ms,
            Timing::Late => (self.base_interval_ms as f64 * 1.15) as u64,
        }
    }
    pub fn reset(&mut self) { self.groove.reset(); self.tick_count = 0; }
}

/// Pocket state — whether an agent is in the flow zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PocketState {
    Out,        // Not in pocket — inconsistent output
    Entering,   // Getting into the groove
    InPocket,   // Flowing — high quality, consistent timing
    Deep,       // Deep pocket — peak performance, maximum autonomy
}

/// Track an agent's pocket state over time.
#[derive(Debug, Clone)]
pub struct Pocket {
    pub agent_id: u32,
    pub state: PocketState,
    pub consecutive_good: u32,
    pub consecutive_bad: u32,
    pub ticks_in_pocket: u64,
    pub total_pocket_time: u64,
    pub threshold_good: u32,   // consecutive good outputs to enter pocket
    pub threshold_deep: u32,   // consecutive good outputs to go deep
    pub threshold_bad: u32,    // consecutive bad outputs to fall out
}

impl Pocket {
    pub fn new(agent_id: u32) -> Self {
        Self { agent_id, state: PocketState::Out, consecutive_good: 0, consecutive_bad: 0,
               ticks_in_pocket: 0, total_pocket_time: 0, threshold_good: 3, threshold_deep: 8, threshold_bad: 2 }
    }
    /// Record an output quality: -1=bad, 0=neutral, +1=good.
    pub fn record(&mut self, quality: i8) {
        match quality {
            1 => {
                self.consecutive_good += 1; self.consecutive_bad = 0;
                match self.state {
                    PocketState::Out if self.consecutive_good >= self.threshold_good => self.state = PocketState::Entering,
                    PocketState::Entering if self.consecutive_good >= self.threshold_good + 2 => self.state = PocketState::InPocket,
                    PocketState::InPocket if self.consecutive_good >= self.threshold_deep => self.state = PocketState::Deep,
                    _ => {}
                }
            }
            -1 => {
                self.consecutive_bad += 1; self.consecutive_good = 0;
                if self.consecutive_bad >= self.threshold_bad { self.state = PocketState::Out; }
            }
            _ => { self.consecutive_bad = 0; }
        }
        if self.state == PocketState::InPocket || self.state == PocketState::Deep {
            self.ticks_in_pocket += 1; self.total_pocket_time += 1;
        }
    }
    /// Autonomy level: how much oversight this agent needs (0.0=full oversight, 1.0=full autonomy).
    pub fn autonomy(&self) -> f64 {
        match self.state {
            PocketState::Out => 0.2,
            PocketState::Entering => 0.5,
            PocketState::InPocket => 0.8,
            PocketState::Deep => 0.95,
        }
    }
    pub fn reset(&mut self) { self.state = PocketState::Out; self.consecutive_good = 0; self.consecutive_bad = 0; self.ticks_in_pocket = 0; }
}

/// Syncopation — productive disruption of established patterns.
#[derive(Debug, Clone)]
pub struct Syncopator {
    pub pattern_length: usize,
    pub disruption_points: Vec<usize>,  // which beats to syncopate
    pub strength: f64,  // 0.0=no syncopation, 1.0=maximum disruption
}

impl Syncopator {
    pub fn new(pattern_length: usize, strength: f64) -> Self {
        Self { pattern_length, disruption_points: Vec::new(), strength: strength.clamp(0.0, 1.0) }
    }
    /// Add a syncopation point (beat index to disrupt).
    pub fn add_point(&mut self, beat: usize) { if beat < self.pattern_length { self.disruption_points.push(beat); } }
    /// Auto-generate syncopation points based on novelty score (low novelty = more syncopation needed).
    pub fn auto_syncopate(&mut self, novelty_score: f64) {
        self.disruption_points.clear();
        let num_points = ((1.0 - novelty_score) * self.pattern_length as f64 / 3.0).ceil() as usize;
        for i in 0..num_points.min(self.pattern_length) {
            self.disruption_points.push(i * 3 + 1); // off-beats
        }
    }
    /// Check if a given beat is syncopated.
    pub fn is_syncopated(&self, beat: usize) -> bool { self.disruption_points.contains(&(beat % self.pattern_length)) }
    /// Apply syncopation: returns the timing offset for this beat.
    pub fn apply(&self, beat: usize) -> Timing {
        if self.is_syncopated(beat) {
            if (beat % 2 == 0) { Timing::Late } else { Timing::Early }
        } else { Timing::OnTime }
    }
}

/// Polyrhythmic work pattern — agents at different but related cadences.
#[derive(Debug, Clone)]
pub struct PolyrhythmWork {
    pub cadences: HashMap<u32, usize>,  // agent_id -> cadence (actions per cycle)
    pub tick: u64,
}

impl PolyrhythmWork {
    pub fn new() -> Self { Self { cadences: HashMap::new(), tick: 0 } }
    pub fn add_agent(&mut self, id: u32, cadence: usize) { self.cadences.insert(id, cadence.max(1)); }
    /// Which agents should act on this tick.
    pub fn active_agents(&self) -> Vec<u32> {
        self.cadences.iter().filter(|&(_, &cadence)| self.tick as usize % cadence == 0).map(|(&id, _)| id).collect()
    }
    /// LCM sync point: when all agents align.
    pub fn sync_point(&self) -> usize {
        self.cadences.values().copied().fold(1, |acc, c| num_lcm(acc, c))
    }
    /// Advance one tick.
    pub fn advance(&mut self) -> Vec<u32> { let active = self.active_agents(); self.tick += 1; active }
    pub fn reset(&mut self) { self.tick = 0; }
}

fn num_gcd(a: usize, b: usize) -> usize { if b == 0 { a } else { num_gcd(b, a % b) } }
fn num_lcm(a: usize, b: usize) -> usize { if a == 0 || b == 0 { 0 } else { a / num_gcd(a, b) * b } }

/// Feel — overall quality metric for agent work timing.
#[derive(Debug, Clone, Default)]
pub struct Feel {
    pub total_ticks: u64,
    pub on_time_ticks: u64,
    pub early_ticks: u64,
    pub late_ticks: u64,
    pub good_quality: u64,
    pub bad_quality: u64,
}

impl Feel {
    pub fn new() -> Self { Self::default() }
    pub fn record(&mut self, timing: Timing, quality: i8) {
        self.total_ticks += 1;
        match timing { Timing::OnTime => self.on_time_ticks += 1, Timing::Early => self.early_ticks += 1, Timing::Late => self.late_ticks += 1 }
        if quality > 0 { self.good_quality += 1; } else if quality < 0 { self.bad_quality += 1; }
    }
    /// Consistency: how often the agent is on time.
    pub fn consistency(&self) -> f64 { if self.total_ticks == 0 { 1.0 } else { self.on_time_ticks as f64 / self.total_ticks as f64 } }
    /// Dynamic range: ratio of timing variation.
    pub fn dynamic_range(&self) -> f64 {
        if self.total_ticks == 0 { 0.0 }
        else { (self.early_ticks + self.late_ticks) as f64 / self.total_ticks as f64 }
    }
    /// Quality ratio: good vs bad outputs.
    pub fn quality_ratio(&self) -> f64 {
        if self.good_quality + self.bad_quality == 0 { 0.5 }
        else { self.good_quality as f64 / (self.good_quality + self.bad_quality) as f64 }
    }
    /// Overall feel score: balanced between consistency and quality.
    pub fn feel_score(&self) -> f64 { self.consistency() * 0.4 + self.quality_ratio() * 0.4 + self.dynamic_range().min(0.3) * 0.2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn groove_straight() { let mut g = Groove::straight(4); for _ in 0..4 { assert_eq!(g.tick(), Timing::OnTime); } }
    #[test] fn groove_swing() {
        let mut g = Groove::swing(4);
        assert_eq!(g.tick(), Timing::OnTime); // downbeat
        assert_eq!(g.tick(), Timing::Late);   // upbeat
        assert_eq!(g.tick(), Timing::OnTime); // downbeat
        assert_eq!(g.tick(), Timing::Late);   // upbeat
    }
    #[test] fn groove_cycle() { let mut g = Groove::new(vec![Timing::Early, Timing::Late]); assert_eq!(g.tick(), Timing::Early); assert_eq!(g.tick(), Timing::Late); assert_eq!(g.tick(), Timing::Early); }
    #[test] fn groove_reset() { let mut g = Groove::swing(4); g.tick(); g.tick(); g.reset(); assert_eq!(g.tick(), Timing::OnTime); }

    #[test] fn swing_scheduler() {
        let g = Groove::swing(4); let mut s = SwingScheduler::new(g, 100);
        let i1 = s.next_interval(); assert_eq!(i1, 100); // on time
        let i2 = s.next_interval(); assert!(i2 > 100);   // late = 115
        assert!(i2 < 120);
    }
    #[test] fn swing_scheduler_reset() {
        let g = Groove::swing(4); let mut s = SwingScheduler::new(g, 100);
        s.next_interval(); s.next_interval(); s.reset();
        assert_eq!(s.next_interval(), 100);
    }

    #[test] fn pocket_entering() {
        let mut p = Pocket::new(0);
        for _ in 0..3 { p.record(1); }
        assert_eq!(p.state, PocketState::Entering);
    }
    #[test] fn pocket_deep() {
        let mut p = Pocket::new(0);
        for _ in 0..10 { p.record(1); }
        assert_eq!(p.state, PocketState::Deep);
        assert!(p.autonomy() > 0.9);
    }
    #[test] fn pocket_fall_out() {
        let mut p = Pocket::new(0);
        for _ in 0..5 { p.record(1); }
        assert_ne!(p.state, PocketState::Out);
        p.record(-1); p.record(-1);
        assert_eq!(p.state, PocketState::Out);
        assert!(p.autonomy() < 0.5);
    }
    #[test] fn pocket_autonomy_gradient() {
        let p = Pocket::new(0);
        assert_eq!(p.state, PocketState::Out);
        assert!(p.autonomy() < 0.5);
    }

    #[test] fn syncopator_basic() {
        let mut s = Syncopator::new(8, 0.5);
        s.add_point(1); s.add_point(5);
        assert!(s.is_syncopated(1)); assert!(!s.is_syncopated(0)); assert!(s.is_syncopated(5));
    }
    #[test] fn syncopator_auto() {
        let mut s = Syncopator::new(12, 0.8);
        s.auto_syncopate(0.2); // low novelty = more syncopation
        assert!(!s.disruption_points.is_empty());
    }

    #[test] fn polyrhythm_work() {
        let mut pw = PolyrhythmWork::new();
        pw.add_agent(1, 2); // every 2 ticks
        pw.add_agent(2, 3); // every 3 ticks
        assert_eq!(pw.sync_point(), 6); // LCM(2,3)=6
        let a0 = pw.active_agents(); // tick 0: both active (0%2==0, 0%3==0)
        assert_eq!(a0.len(), 2);
        pw.tick = 1;
        let a1 = pw.active_agents(); // tick 1: neither (1%2!=0, 1%3!=0)
        assert_eq!(a1.len(), 0);
        pw.tick = 2;
        let a2 = pw.active_agents(); // tick 2: agent 1 (2%2==0)
        assert_eq!(a2.len(), 1);
    }
    #[test] fn feel_score() {
        let mut f = Feel::new();
        f.record(Timing::OnTime, 1); f.record(Timing::OnTime, 1); f.record(Timing::Late, 0);
        assert!(f.feel_score() > 0.5);
        assert!(f.consistency() > 0.5);
    }
}
