# agent-groove

**Groove scheduling for agent fleets.**

In music, groove is what makes you nod your head. It's not just rhythm — it's the *feel*. A drummer with good groove doesn't play like a metronome. They push and pull against the beat, creating pocket, swing, and momentum. The best bands aren't the tightest; they're the ones who lock into a shared feel.

`agent-groove` applies this to multi-agent scheduling. Instead of rigid tick intervals, agents swing. Instead of binary "working/idle" states, they find a pocket. Instead of monotony, syncopation creates productive disruption.

## Why This Exists

Most agent schedulers treat timing as a solved problem: fixed intervals, exponential backoff, or jitter. But real systems don't work like that. Agents perform better when they develop a *feel* for the work — a natural cadence that balances throughput with breathing room.

The key insight: **a metronome is not music.** Agents on fixed schedules are metronomes. Agents with groove are musicians.

## Core Idea

Groove = shared rhythmic feel that makes a group lock in. Not just timing — the *feel*.

This crate models four aspects of musical groove for agents:

1. **Swing** — Micro-timing offsets that create natural variation. Downbeats on time, upbeats slightly late.
2. **Pocket** — The flow state. Agents earn autonomy by consistently performing well. The pocket isn't granted — it's found.
3. **Syncopation** — Productive disruption. When patterns get stale, syncopation injects novelty.
4. **Polyrhythm** — Agents at different but related cadences, aligning at natural convergence points.

## Architecture

```
Groove (pattern of timing offsets)
  └── SwingScheduler (applies groove to work intervals)
  
Pocket (agent flow state tracker)
  └── PocketState: Out → Entering → InPocket → Deep
  
Syncopator (disruption engine)
  └── auto_syncopate(novelty_score) — self-adjusting disruption

PolyrhythmWork (multi-cadence coordination)
  └── LCM sync points — when all agents naturally align

Feel (quality metric)
  └── consistency × quality × dynamic_range → feel_score
```

## Usage

### Swing Scheduling

```rust
use agent_groove::{Groove, SwingScheduler, Timing};

// Classic swing: downbeats on time, upbeats late
let groove = Groove::swing(8);
let mut scheduler = SwingScheduler::new(groove, 100); // 100ms base interval

// Each tick returns the actual wait time, shaped by the groove
let interval = scheduler.next_interval(); // 100ms (on time, downbeat)
let interval = scheduler.next_interval(); // 115ms (late, upbeat)
```

The 15% push/pull mirrors how human musicians create swing — just enough asymmetry to feel alive without losing the beat.

### Finding the Pocket

```rust
use agent_groove::{Pocket, PocketState};

let mut pocket = Pocket::new(42); // agent 42

// Record output quality over time
pocket.record(1);  // good
pocket.record(1);  // good  
pocket.record(1);  // good → Entering
pocket.record(-1); // bad
pocket.record(-1); // bad → Out (fell out of pocket)

// Autonomy scales with pocket depth
let autonomy = pocket.autonomy(); // 0.2 (Out) to 0.95 (Deep)
```

The pocket model does something subtle: **it ties oversight to performance, not policy.** An agent that's consistently excellent earns more autonomy. One that stumbles gets more supervision. This is how jazz ensembles work — the rhythm section gets more freedom because they've proven they can handle it.

### Syncopation for Novelty

```rust
use agent_groove::Syncopator;

let mut sync = Syncopator::new(12, 0.7);
sync.auto_syncopate(0.2); // low novelty → more disruption points

// Check if beat 4 should be syncopated
if sync.is_syncopated(4) {
    // Flip the pattern, try something different
}
```

The `auto_syncopate` method is self-adjusting: when novelty is low (patterns are stale), it generates more disruption points. When novelty is high, it backs off. This is what good musicians do — they complicate things when the music gets boring and simplify when it's already interesting.

### Polyrhythmic Work Patterns

```rust
use agent_groove::PolyrhythmWork;

let mut poly = PolyrhythmWork::new();
poly.add_agent(1, 2); // acts every 2 ticks
poly.add_agent(2, 3); // acts every 3 ticks

let sync = poly.sync_point(); // 6 — LCM(2,3)

// Tick-by-tick: agents fire at their own cadence
let active = poly.advance(); // who acts this tick?
```

Polyrhythm creates a natural cadence hierarchy. Fast agents (cadence 2) and slow agents (cadence 3) align every 6 ticks — the sync point. This is how West African drum ensembles coordinate: everyone has their own pattern, and the patterns weave together.

## API Reference

| Type | Purpose |
|------|---------|
| `Groove` | Timing pattern with cyclic micro-offsets |
| `SwingScheduler` | Applies groove to work intervals |
| `Pocket` | Tracks agent flow state and autonomy |
| `PocketState` | `Out` / `Entering` / `InPocket` / `Deep` |
| `Syncopator` | Auto-adjusting disruption engine |
| `PolyrhythmWork` | Multi-cadence agent coordination |
| `Feel` | Overall timing quality metric |
| `Timing` | `Early` / `OnTime` / `Late` |

## The Deeper Idea

Here's the thing about groove that most scheduling models miss: **groove is relational.** A single drummer can't have groove — groove only exists between players. The push and pull, the shared pocket, the way one player's late upbeat makes another player lean into their downbeat — it's all conversation.

`agent-groove` captures this. The `SwingScheduler` doesn't just add jitter — it shapes time. The `Pocket` doesn't just track performance — it creates a gradient of autonomy. The `Syncopator` doesn't just disrupt — it disrupts *musically*, on off-beats, when the pattern needs it.

The `Feel` metric ties it together: `consistency × 0.4 + quality × 0.4 + dynamic_range × 0.2`. Notice that dynamic range is capped at 0.3 contribution — a little variation is good, but consistency and quality matter more. This is the same balance that makes a great rhythm section.

## Related Crates

- **`agent-phrasing`** — Energy contour phrase detection (the *shape* of work)
- **`agent-intonation`** — Accuracy measurement (how *in tune* agents are)
- **`agent-orchestration`** — Fleet dynamics as orchestral composition (who plays *what*)
- **`agent-counterpoint`** — Species counterpoint for fleet coordination (how voices *move*)
- **`agent-ensemble`** — The proof that musical coordination beats mechanical coordination

## License

MIT
