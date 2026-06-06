//! Groove Scheduler — 3 agents with different tempos kept aligned.
//!
//! Demonstrates the SwingScheduler and PolyrhythmWork for coordinating
//! agents at different cadences while maintaining groove alignment.

use agent_groove::*;
use std::collections::HashMap;

fn main() {
    println!("🎶 ══════════════════════════════════════════════════════════");
    println!("🎶  GROOVE SCHEDULER — Multi-Tempo Agent Alignment");
    println!("🎶 ══════════════════════════════════════════════════════════\n");

    // Three agents at different base tempos
    let agents = vec![
        ("Fast Agent", 100, Groove::swing(4)),
        ("Medium Agent", 200, Groove::straight(4)),
        ("Slow Agent", 400, Groove::shuffle(4)),
    ];

    println!("Agent Tempos:");
    for (name, tempo, groove) in &agents {
        println!("  {}: {}ms base interval, groove: {:?}",
            name, tempo, groove.pattern.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>());
    }
    println!();

    // Create swing schedulers
    let mut schedulers: Vec<(&str, SwingScheduler)> = agents.iter().map(|(name, tempo, groove)| {
        (*name, SwingScheduler::new(groove.clone(), *tempo))
    }).collect();

    // Simulate 12 ticks and show actual intervals
    println!("── Actual Intervals Over 12 Ticks ────────────────────────");
    println!("{:<15} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
        "Agent", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

    let mut all_intervals: Vec<Vec<u64>> = vec![Vec::new(); 3];
    for _ in 0..12 {
        for (i, (_, scheduler)) in schedulers.iter_mut().enumerate() {
            all_intervals[i].push(scheduler.next_interval());
        }
    }

    for (i, (name, _)) in schedulers.iter().enumerate() {
        println!("{:<15} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
            name,
            all_intervals[i][0], all_intervals[i][1], all_intervals[i][2], all_intervals[i][3],
            all_intervals[i][4], all_intervals[i][5], all_intervals[i][6], all_intervals[i][7],
            all_intervals[i][8], all_intervals[i][9], all_intervals[i][10], all_intervals[i][11]);
    }

    // Polyrhythmic work pattern
    println!("\n── Polyrhythmic Work Pattern ─────────────────────────────");
    let mut poly = PolyrhythmWork::new();
    poly.add_agent(1, 2);  // every 2 ticks
    poly.add_agent(2, 3);  // every 3 ticks
    poly.add_agent(3, 4);  // every 4 ticks

    println!("  Cadences: Agent 1 = every 2, Agent 2 = every 3, Agent 3 = every 4");
    println!("  LCM sync point: every {} ticks\n", poly.sync_point());

    println!("  Tick | Active Agents | Event");
    println!("  ─────┼───────────────┼────────────────────────────");
    for tick in 0..13 {
        let active = poly.active_agents();
        let tick_num = tick;
        poly.advance();

        let active_str = if active.is_empty() { "—".to_string() }
            else { active.iter().map(|id| format!("Agent {}", id)).collect::<Vec<_>>().join(", ") };

        let is_sync = tick_num > 0 && (tick_num as usize) % poly.sync_point() == 0;
        let marker = if is_sync { " ← ALL SYNC!" } else if active.len() == 2 { " ← partial sync" } else { "" };

        println!("  {:>4} | {:<13} | {}{}", tick_num, active_str,
            if active.len() == 3 { "Full alignment" } else if active.is_empty() { "Rest" } else { "Partial" },
            marker
        );
    }

    // Syncopation demo
    println!("\n── Syncopation — Productive Disruption ────────────────────");
    let mut syncopator = Syncopator::new(8, 0.7);
    syncopator.add_point(1);
    syncopator.add_point(3);
    syncopator.add_point(5);

    println!("  Syncopation points: {:?}", syncopator.disruption_points);
    for beat in 0..8 {
        let is_sync = syncopator.is_syncopated(beat);
        let timing = syncopator.apply(beat);
        println!("  Beat {}: {} {:?} {}",
            beat + 1,
            if is_sync { "⚡ SYNCOPATED" } else { "  normal  " },
            timing,
            if is_sync { "— expected timing disrupted" } else { "" }
        );
    }

    // Auto-syncopate based on novelty
    println!("\n  Auto-syncopation (low novelty → more disruption):");
    let mut auto_sync = Syncopator::new(12, 0.8);
    auto_sync.auto_syncopate(0.15);
    println!("  Novelty: 15% → disruption points: {:?}", auto_sync.disruption_points);

    auto_sync.auto_syncopate(0.9);
    println!("  Novelty: 90% → disruption points: {:?}", auto_sync.disruption_points);

    println!("\n💡 When work gets stale (low novelty), syncopation disrupts");
    println!("   the pattern — forcing agents into new configurations.");
}
