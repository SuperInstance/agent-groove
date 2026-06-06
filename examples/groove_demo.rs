//! Groove Demo — Agents at different rhythmic alignments.
//!
//! Shows how groove patterns affect agent work timing and how
//! pocket states evolve as agents sync up or drift apart.

use agent_groove::*;

fn main() {
    println!("🥁 ══════════════════════════════════════════════════════════");
    println!("🥁  GROOVE DEMO — Rhythmic Alignment & Pocket States");
    println!("🥁 ══════════════════════════════════════════════════════════\n");

    // Three agents with different groove patterns
    let mut grooves: Vec<(&str, Groove)> = vec![
        ("Straight Agent", Groove::straight(4)),
        ("Swing Agent",    Groove::swing(4)),
        ("Shuffle Agent",  Groove::shuffle(4)),
    ];

    println!("Groove Patterns:");
    for (name, g) in &grooves {
        println!("  {}: {:?}", name, g.pattern.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>());
    }
    println!();

    // Run 16 ticks and show how timing diverges
    println!("── Timing Over 16 Beats ──────────────────────────────────");
    println!("{:<20} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4}",
        "Agent", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

    let mut timings: Vec<Vec<Timing>> = vec![Vec::new(), Vec::new(), Vec::new()];
    for beat in 0..16 {
        for (i, (_, groove)) in grooves.iter_mut().enumerate() {
            timings[i].push(groove.tick());
        }
    }

    for (i, (name, _)) in grooves.iter().enumerate() {
        let timing_strs: Vec<String> = timings[i].iter().map(|t| {
            match t {
                Timing::Early => "E".to_string(),
                Timing::OnTime => "·".to_string(),
                Timing::Late => "L".to_string(),
            }
        }).collect();
        println!("{:<20} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4}",
            name, timing_strs[0], timing_strs[1], timing_strs[2], timing_strs[3],
            timing_strs[4], timing_strs[5], timing_strs[6], timing_strs[7],
            timing_strs[8], timing_strs[9], timing_strs[10], timing_strs[11],
            timing_strs[12], timing_strs[13], timing_strs[14], timing_strs[15]);
    }

    // Now show pocket states evolving
    println!("\n── Pocket State Evolution ────────────────────────────────");
    let mut pockets = vec![
        Pocket::new(0),
        Pocket::new(1),
        Pocket::new(2),
    ];
    let pocket_names = ["Agent A", "Agent B", "Agent C"];

    // Simulate varying quality inputs
    let quality_patterns = [
        [1, 1, 1, 0, 1, 1, 1, 1, 1, 1],     // Agent A: mostly good → deep pocket
        [-1, 1, 0, 1, -1, 1, 1, 1, 1, 1],    // Agent B: inconsistent → struggles
        [1, 1, 1, 1, 1, -1, 1, 1, 1, 1],     // Agent C: one blip then recovers
    ];

    for tick in 0..10 {
        for (i, pocket) in pockets.iter_mut().enumerate() {
            pocket.record(quality_patterns[i][tick] as i8);
        }

        if tick % 2 == 0 || tick == 9 {
            println!("  Tick {:>2}: {}", tick,
                pockets.iter().enumerate().map(|(i, p)| {
                    let state_str = match p.state {
                        PocketState::Out => "❌ Out".to_string(),
                        PocketState::Entering => "🔄 Entering".to_string(),
                        PocketState::InPocket => "🎯 In Pocket".to_string(),
                        PocketState::Deep => "🔥 DEEP".to_string(),
                    };
                    format!("{}: {} (autonomy: {:.0}%)", pocket_names[i], state_str, p.autonomy() * 100.0)
                }).collect::<Vec<_>>().join(" | ")
            );
        }
    }

    // Show the groove score (Feel metric)
    println!("\n── Groove Score (Feel) ───────────────────────────────────");
    let mut feel = Feel::new();
    let groove_feel = Groove::swing(8);
    let quality_sequence = [1, 1, 0, 1, -1, 1, 1, 0];

    let mut gf = groove_feel;
    for (i, &q) in quality_sequence.iter().enumerate() {
        let timing = gf.tick();
        feel.record(timing, q);
        if i % 2 == 0 {
            println!("  Beat {}: timing={:?}, quality={} → consistency={:.0}%, quality_ratio={:.0}%",
                i + 1, timing, q, feel.consistency() * 100.0, feel.quality_ratio() * 100.0);
        }
    }

    println!("\n  📊 Final Feel Score: {:.2} / 1.00", feel.feel_score());
    println!("     Consistency:      {:.0}%", feel.consistency() * 100.0);
    println!("     Dynamic Range:    {:.0}%", feel.dynamic_range() * 100.0);
    println!("     Quality Ratio:    {:.0}%", feel.quality_ratio() * 100.0);

    println!("\n💡 Agents that find the pocket need less oversight.");
    println!("   The groove pattern isn't just timing — it's flow.");
}
