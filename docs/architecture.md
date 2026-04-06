# Architecture Overview

## System Layers

```
┌─────────────────────────────────────────────┐
│              Game Layer (Bevy App)          │
├─────────────────────────────────────────────┤
│  UI Systems    │  Economy  │  AI Systems   │
│  Input Systems │  Combat   │  Animation     │
├─────────────────────────────────────────────┤
│         Shared Components (ECS)             │
│  - Hero, Enemy, Building, Merchant, etc.   │
├─────────────────────────────────────────────┤
│         Shared Events                       │
│  - BountyCompleted, HeroDeath, etc.        │
├─────────────────────────────────────────────┤
│         Resources (Global State)            │
│  - GameEconomy, GameTime, KingdomState     │
└─────────────────────────────────────────────┘
```

## Data Flow

1. **Player Input** → UI Events → System Queries
2. **Systems** read components, modify world, spawn events
3. **Events** trigger other systems, create effects
4. **Resources** provide global state access

## Core Systems

| System | File | Purpose |
|--------|------|---------|
| Hero AI | `hero.rs` | Hero decision-making |
| Enemy AI | `enemy.rs` | Enemy behavior |
| Combat | `combat.rs` | Attack/heal logic |
| Economy | `economy.rs` | Taxes, bounties |
| Spawning | `features.rs` | Dynamic entity creation |
| UI Update | `ui.rs` | HUD rendering |
