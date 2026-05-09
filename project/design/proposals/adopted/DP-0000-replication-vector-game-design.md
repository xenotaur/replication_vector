# Replication Vector — Project Bootstrap Design Summary

## Working Title

**Replication Vector**

Candidate tagline:

> **Mine. Defend. Replicate.**

Optional subtitle / descriptor:

> **A VNP arcade siege game**

## Executive Summary

**Replication Vector** is a retro vector-graphics arcade survival game designed both as a compelling standalone game and as a dogfood / stress-test project for the **Velumin** vector graphics library.

The player controls a **von Neumann probe**: a self-replicating spacecraft that arrives in hostile asteroid fields, mines raw material, builds defenses, survives attacks by anti-replicator machines, constructs a child probe, and chooses when to launch that successor into the next sector.

The concept began as an inversion of *Star Castle*. In *Star Castle*, the player controls a nimble ship trying to break through the shields of a heavily defended central fortress. **Replication Vector** inverts that relationship: the player is the heavy central entity under siege, building and maintaining shields while faster attackers try to destroy it.

The design later crystallized around a stronger science-fiction premise: rather than escaping a level, the player is trying to **reproduce**. The level ends when the player decides that the child probe is ready enough to launch.

The core design thesis is:

> The level does not end because the player escapes. The level ends because the player chooses to reproduce.

This gives the game a distinctive identity and ties together mining, defense, resource allocation, enemy pressure, and progression.

---

## Design Goals

### Primary Game Design Goals

1. Create a tight, readable, skill-based vector arcade game.
2. Invert the classic *Star Castle* premise while avoiding a direct clone.
3. Make mining, defense, and replication feel like parts of one coherent loop.
4. Give the player meaningful agency over when a level ends.
5. Create strategic tension between present survival and future success.
6. Keep the MVP small enough to implement and test rapidly.
7. Leave room for later expansion into roguelite progression, upgrades, and procedural sectors.

### Velumin / Technical Dogfood Goals

1. Exercise Velumin’s core vector-rendering capabilities in a real downstream project.
2. Test rotating vector geometry, segmented arcs, shield rings, procedural asteroid outlines, particles, beams, and projectile trails.
3. Validate Velumin’s usability from an external game repo rather than only from internal examples.
4. Identify missing APIs, awkward abstractions, packaging issues, and documentation gaps.
5. Develop reusable rendering-test helpers that can later live in Velumin.
6. Provide a visually appealing showcase project for Velumin-style tools.

---

## High-Level Pitch

**Replication Vector** is a vector arcade survival game where the player controls a self-replicating von Neumann probe under attack by Berserker-like anti-replicator machines. The probe must mine asteroids, build shield arcs, deploy limited defenses, construct a child probe, and choose the right moment to launch its successor before the system becomes unsurvivable.

Short pitch:

> Mine asteroids. Build shields. Survive the Berserkers. Launch your successor.

One-sentence pitch:

> **Replication Vector** is a retro vector arcade game about defending a self-replicating spacecraft long enough to build and launch its next generation.

---

## Core Inspiration

### Star Castle Inversion

The original conceptual seed was an inverse *Star Castle*:

- In *Star Castle*, the player is the nimble attacker.
- The enemy is a heavy central fortress protected by rotating shields.
- The player destroys shield rings to expose and destroy the central enemy.
- The fortress launches dangerous mines and fires a powerful central cannon.

**Replication Vector** reverses this:

- The player controls the heavy central probe / fortress.
- Enemy ships are fast, Asteroids-like attackers.
- The player builds and repairs shield segments instead of destroying them.
- The player is under siege while trying to complete a larger objective.

The game should preserve the vector-combat DNA of *Star Castle* without depending on its title, exact premise, or mechanical structure.

### Asteroids Influence

The game also borrows from *Asteroids*:

- vector graphics visual language;
- asteroid fields as spatial hazards;
- drifting inertial motion;
- small fast ships and projectiles;
- large rocks that can be mined, depleted, fractured, or used as cover.

Unlike *Asteroids*, the player is not primarily a small fighter clearing the screen. The player is a slow, powerful, vulnerable industrial probe trying to survive a replication cycle.

### Yars' Revenge Influence

The design also loosely echoes *Yars’ Revenge* in the idea that shields, barriers, and decisive weapons can define the rhythm of combat. In **Replication Vector**, however, the player is the one building and preserving the defensive shell rather than cutting through it.

### Von Neumann Probe Fiction

The game’s core fiction is based on self-replicating spacecraft:

- A von Neumann probe arrives in a new system.
- It harvests local matter, especially asteroids.
- It uses the material to build a copy of itself.
- It launches the successor onward.
- The player continues as that successor in the next sector.

This fiction elegantly explains the game loop.

### Berserker-Style Enemy Fiction

Enemy machines are inspired by Fred Saberhagen-style Berserkers: hostile autonomous machines dedicated to preventing replication, spreading, or life-like expansion.

In this game, the enemies can be interpreted as:

- anti-replicator machines;
- galactic immune systems;
- sterilization drones;
- ancient war machines;
- predator machines that hunt self-replicators;
- rival machine ecologies.

The MVP does not need a heavy story, but the enemy behavior should reinforce the idea that they are trying to stop replication, not merely shoot at the player.

---

## Core Fantasy

The player fantasy is:

> I am a powerful but vulnerable self-replicating machine under siege. I must mine, fortify, and build my successor before hostile machines destroy me.

This is not a pure shooter fantasy and not a pure base-building fantasy. It is an arcade survival fantasy built around defensive construction and reproduction under pressure.

The emotional tone should be tense, focused, and high-stakes:

- The parent probe is powerful but not agile.
- The child probe is precious and vulnerable.
- Every defensive choice has an opportunity cost.
- Waiting longer can produce a stronger successor, but also attracts greater danger.
- Launching is a dramatic commitment, not an automatic level transition.

---

## Core Loop

The core loop is:

```text
Arrive → Mine → Defend → Allocate resources → Build child probe → Launch → Continue as successor
```

Expanded loop:

```text
Enter sector
↓
Locate and approach resource asteroid
↓
Mine matter while under light attack
↓
Spend matter on shields, repairs, drones, weapons, launch charge, or child-probe construction
↓
Enemy attacks escalate as replication progresses or time passes
↓
Player decides whether to stay and improve the child probe or launch now
↓
Child probe launches
↓
Next sector begins with inheritance from launch outcome
```

The design should continually ask the player:

> Do I spend this resource helping the parent survive now, or helping the child probe succeed later?

---

## Main Player Decisions

The most important recurring decisions are:

1. **Mine or maneuver**
   - Continue extracting matter from the asteroid.
   - Break off docking or mining to reposition and avoid danger.

2. **Defend now or build the future**
   - Spend matter on shields and repairs.
   - Spend matter on the child probe.

3. **Attack or conserve**
   - Charge and fire the main weapon.
   - Deploy drones.
   - Save matter / energy for construction or launch.

4. **Launch early or stay longer**
   - Launch a minimum viable child probe.
   - Stay to complete or overbuild the successor.
   - Risk escalating attacks and parent destruction.

5. **Protect parent or child**
   - Defend the core probe.
   - Defend the child-probe construction cradle.
   - Accept damage to one in order to preserve the other.

---

## Player Entity

The player controls a large parent probe / factory ship.

### Desired Feel

The probe should feel:

- slow;
- heavy;
- industrial;
- powerful;
- vulnerable when exposed;
- capable of impressive defensive and offensive actions;
- more like a mobile fortress than a nimble fighter.

However, it must not feel passive. Even if the ship is heavy, the player should be actively doing things every few seconds:

- aiming;
- rotating or placing shield arcs;
- mining;
- firing;
- allocating resources;
- deploying drones;
- managing launch timing;
- responding to shield breaches.

### Possible Player Capabilities

MVP capabilities:

- slow thrust and rotation;
- mining beam;
- build shield segment;
- repair shield segment;
- main weapon / cannon;
- construct child-probe module;
- launch child probe.

Possible early additions:

- deploy attack drone;
- emergency defensive pulse;
- temporary shield overcharge;
- limited short-burn repositioning;
- targeting reticle for drones or main cannon;
- resource-routing mode.

### Movement Model

The movement model should support the fantasy of mass and inertia.

Options:

1. **Asteroids-style inertial movement**
   - Pros: matches vector arcade heritage.
   - Cons: can make a heavy probe frustrating if too sluggish.

2. **Slow thrust with rotational control**
   - Pros: gives a strong sense of weight.
   - Cons: requires careful tuning to avoid passivity.

3. **Mostly stationary with local repositioning**
   - Pros: emphasizes fortress-defense gameplay.
   - Cons: risks feeling like a turret defense game rather than a space game.

Recommendation for MVP:

> Use slow inertial movement and rotation, but make most tactical agency come from mining, shield placement, aiming, and launch timing rather than high-speed evasion.

---

## Resource Model

### MVP Resource

The MVP should use one primary resource:

- **matter**;
- **mass**;
- **ore**;
- **raw material**.

Recommended term: **matter** or **mass**.

Why one resource:

- easier to balance;
- easier to communicate;
- supports rapid prototyping;
- keeps the focus on tactical allocation rather than economy management;
- avoids premature complexity.

### Resource Uses

Matter can be spent on:

1. shield construction;
2. shield repair;
3. child-probe construction;
4. weapon charge or ammo;
5. drones;
6. launch preparation.

### Future Resource Expansion

Later versions might add:

- energy;
- rare elements;
- computation;
- antimatter;
- memory / information;
- integrity / structural mass.

But the first playable version should prove the single-resource loop first.

---

## Mining System

Mining is not just a background economy. It is a spatial and tactical activity.

### Mining Purposes

Mining should:

- provide matter for construction;
- force the player to approach or dock with asteroids;
- create vulnerability windows;
- create positioning decisions;
- make asteroids central to gameplay;
- give the level a natural resource limit.

### Asteroid Roles

Asteroids can serve as:

- resource sources;
- cover;
- collision hazards;
- terrain-like obstacles;
- enemy navigation constraints;
- visual anchors;
- depletion timers;
- strategic objectives.

### Mining Interaction Options

Potential approaches:

1. **Dock-and-mine**
   - The player must attach to or remain near an asteroid.
   - Strong risk/reward.
   - Good for creating siege tension.

2. **Beam mining at range**
   - Player points a mining beam at asteroid surfaces.
   - More active and visually expressive.
   - Less restrictive than docking.

3. **Drone mining**
   - Mining drones extract resources while the player defends them.
   - Adds complexity and may be better for later versions.

Recommended MVP:

> Use a short-range mining beam that works best when close or docked, with clear visual resource flow from asteroid to probe.

### Asteroid Depletion

Asteroids should visibly change as they are mined:

- outline cracks;
- shrinking regions;
- glowing cut lines;
- exposed resource veins;
- fragmenting shells;
- depleted gray/dim vector outlines.

This is valuable both for gameplay readability and Velumin rendering tests.

---

## Shield System

The shield system is the clearest inheritance from the inverse *Star Castle* concept.

### Shield Concept

The player constructs and repairs geometric shield segments around the parent probe and possibly around the child-probe cradle.

Shields are not abstract hit points. They are spatial vector objects.

### Shield Properties

Each shield segment may have:

- angular position;
- arc length;
- radius;
- thickness or strength;
- current integrity;
- ownership / layer;
- repair state;
- construction state;
- damage state.

### Shield Gameplay

Shields should:

- block enemy projectiles;
- absorb collisions or breacher attacks;
- create gaps that enemies can exploit;
- possibly block or interfere with player weapons;
- require matter to build and repair;
- decay or overload under sustained fire;
- force spatial planning.

### Shield Design Tensions

Shields must be strong enough to matter but not so strong that they trivialize the game.

Possible balancing constraints:

- shield segments cost matter;
- shields have gaps;
- some enemies specialize in cutting shields;
- shield repairs take time;
- shield arcs can block player shots;
- outer shields cost more;
- overbuilt shields make the player safe now but delay the child probe.

### MVP Shield Implementation

Recommended MVP:

- one or two shield radii around the parent probe;
- shield segments built as arcs;
- segments take damage and can be repaired;
- gaps are intentionally visible;
- enemies can target gaps or damaged arcs;
- shield construction competes with child-probe construction.

---

## Child-Probe Construction

The child probe is the central objective.

### Design Principle

The child probe should be physically visible and mechanically meaningful. It should not be only a progress bar.

The player should see the successor being assembled piece by piece.

### Possible Child-Probe Modules

MVP or near-MVP modules:

- seed core;
- hull spine;
- engine;
- reactor;
- shield seed;
- memory core;
- factory ring;
- launch frame.

### Construction States

Possible states:

1. **Seeded** — minimal core exists.
2. **Structurally viable** — can survive launch but starts weak.
3. **Operational** — normal launch quality.
4. **Fortified** — starts next level with bonus protection.
5. **Overbuilt** — stronger but launch takes longer or draws more enemies.

### Launch Thresholds

Potential thresholds:

- below 60%: cannot launch;
- 60–79%: weak launch;
- 80–99%: viable launch;
- 100%: complete launch;
- above 100%: overbuilt launch with bonus and extra risk.

These numbers are placeholders for tuning.

### Launch Decision

The launch decision should be the dramatic climax of each level.

The player should ask:

> Can I survive long enough to improve the child probe, or should I launch now before the parent collapses?

### Launch Sequence

A launch should be visually and mechanically significant:

- launch path appears;
- countdown begins;
- enemies intensify or retarget the child;
- shields may need to open a corridor;
- the child probe accelerates along a vector path;
- the parent may be abandoned, destroyed, or become irrelevant.

Possible future variant:

> The player must protect the child probe during a short launch window, after which control transfers to the child in the next sector.

---

## Enemy Design

Enemies are anti-replicator machines. Their behavior should express their purpose: prevent the child probe from launching.

### Enemy Design Goals

Enemies should:

- be readable as vector silhouettes;
- have distinct tactical roles;
- create pressure on different parts of the player system;
- force the player to balance defense, offense, and construction;
- scale in intensity without requiring many enemy types.

### MVP Enemy Types

Recommended first three enemy types:

#### 1. Needle / Skirmisher

Fast attacker that circles, strafes, and fires through shield gaps.

Purpose:

- tests player aiming;
- punishes gaps;
- creates constant pressure.

#### 2. Cutter / Breacher

Slower enemy that attacks shield segments directly, perhaps with a cutting beam or collision saw.

Purpose:

- prevents static turtling;
- makes shield maintenance active;
- creates priority targets.

#### 3. Seeker / Hunter

Small mine-like or drone-like enemy that targets the child-probe construction cradle or weak spots.

Purpose:

- links enemy behavior to replication objective;
- forces the player to defend something other than the parent core;
- echoes *Star Castle* hunter particles.

### Later Enemy Types

Potential later additions:

- **Bomber** — destroys asteroids, resource flows, or docks.
- **Jammer** — interferes with mining or launch systems.
- **Carrier** — releases smaller hunter drones.
- **Sterilizer** — slowly contaminates or destroys the asteroid field.
- **Lancer** — charges a line attack that can pierce shield arcs.
- **Snare** — tries to trap or slow the launch sequence.
- **Assimilator** — consumes matter before the player can mine it.

### Enemy Escalation

Enemy intensity can scale by:

- time spent in sector;
- amount of matter mined;
- child-probe completion level;
- launch preparation;
- shield strength;
- sector number;
- prior player success.

A good design should avoid making enemy escalation feel arbitrary. The fiction can explain it as detection:

> The more the VNP mines and builds, the brighter its replication signature becomes.

---

## Level Structure

Each level represents one sector, system, or asteroid field.

### Recommended Level Rhythm

1. **Arrival**
   - Player enters the sector.
   - Asteroids and threats are visible or partially scanned.

2. **Approach / Survey**
   - Player chooses a mining position.
   - Early enemies may appear.

3. **Extraction**
   - Mining begins.
   - Matter flows into the parent probe.
   - Shield and child construction begin.

4. **Siege**
   - Enemy pressure escalates.
   - Player must repair, build, and fight under stress.

5. **Launch Window**
   - Child probe reaches minimum viability.
   - Player chooses when to launch.
   - Launch may trigger a final attack spike.

6. **Transition**
   - Child probe escapes.
   - The next sector begins.
   - Successor inherits properties based on launch outcome.

### Level-End Condition

The level ends when:

- the child probe successfully launches; or
- the parent and/or child probe is destroyed.

Optional future conditions:

- sector sterilized by enemies;
- asteroid depleted before minimum child viability;
- launch path blocked;
- parent survives but child fails to escape.

---

## Progression

### MVP Progression

The MVP does not need a full campaign. It can be an endless arcade progression with increasing difficulty.

Potential MVP progression:

- each launched child becomes the player in the next sector;
- each sector is harder;
- launch quality affects next-sector starting conditions;
- score is based on generations survived, launch quality, matter efficiency, and parent survival.

### Inheritance Ideas

The child probe might inherit:

- starting shield strength;
- initial matter reserve;
- mining efficiency;
- weapon strength;
- drone capacity;
- launch engine quality;
- damage or defects from early launch;
- one selected module from the parent.

### Upgrade Model

Avoid a full tech tree in the MVP.

Possible early upgrade structure:

- one upgrade choice after each successful launch;
- upgrade determined by modules completed before launch;
- launch quality modifies the next probe rather than unlocking a separate menu tree.

Examples:

- better shield seed;
- faster mining beam;
- cheaper repairs;
- stronger main weapon;
- faster construction;
- drone bay;
- emergency pulse.

### Future Campaign Possibilities

Later versions could add:

- galactic map;
- branching sectors;
- rival replicators;
- long-term mutation;
- enemy adaptation;
- story events;
- persistent lineages;
- archive / memory mechanics;
- philosophical tension around replication and survival.

These should be deferred until the core arcade loop is proven.

---

## Visual Style

### Overall Style

The game should use a retro vector-arcade look:

- black background;
- bright vector lines;
- geometric ship silhouettes;
- crisp arcs and rings;
- minimal filled shapes;
- glowing or flickering line effects;
- procedural asteroid outlines;
- readable projectile trails;
- sparse, high-contrast UI.

### Visual Priorities

Readability is critical. The player must immediately distinguish:

- parent probe;
- child probe;
- shield arcs;
- enemy shots;
- player shots;
- mining beam;
- resource flow;
- launch path;
- enemy types;
- damaged versus intact geometry.

### Velumin Rendering Features to Exercise

The game should stress-test:

- line rendering;
- transformed vector shapes;
- rotated geometry;
- segmented arcs;
- concentric rings;
- procedural polygons;
- line joins;
- line caps;
- dashed or flickering lines;
- particle trails;
- glow approximation;
- collision/debug overlays;
- deterministic scene rendering;
- golden vector-scene tests.

### Visual Grammar Ideas

Potential visual grammar:

- player structures use smooth arcs and symmetric forms;
- Berserker enemies use angular jagged forms;
- mining uses steady beam lines and particle flow;
- launch uses long converging vectors;
- damage uses broken, flickering, or fragmented lines;
- construction uses ghosted outlines that become solid.

---

## Audio Style

Audio was not deeply discussed yet, but likely direction:

- retro arcade tones;
- vector-era inspired beeps, hums, drones, and alarms;
- mining beam hum;
- shield impact pings;
- construction pulses;
- launch charge crescendo;
- harsh machine tones for Berserkers.

Potential future technical direction:

- simple generated audio;
- Python-controlled chiptune / synth effects;
- minimal dependency sound system;
- no requirement for elaborate music in MVP.

---

## User Interface

The UI should remain minimal and diegetic where possible.

### Essential UI Elements

MVP UI needs to show:

- current matter reserve;
- child-probe completion / viability;
- parent integrity;
- shield status;
- launch readiness;
- selected build mode;
- warning indicators for incoming threats.

### Diegetic UI Possibilities

Instead of heavy HUD panels:

- child completion is visible in the constructed geometry;
- shield damage is visible as broken arcs;
- matter reserve appears as orbiting particles or a simple numeric counter;
- launch readiness appears as a vector corridor or charge ring;
- enemy detection appears as radar pips.

The UI should support fast arcade decisions rather than spreadsheet-like management.

---

## MVP Definition

The first playable prototype should be deliberately small.

### MVP Scope

The MVP should include:

- one parent VNP;
- one asteroid field;
- one visible child-probe construction site;
- one resource type;
- mining beam;
- shield arc construction / repair;
- main weapon;
- three enemy types;
- escalating attack pressure;
- player-triggered launch;
- next-sector transition or score screen;
- minimal vector UI;
- basic render/test harness.

### MVP Non-Goals

The MVP should avoid:

- full tech tree;
- galactic map;
- multiple resources;
- extensive story;
- complex save/load;
- many enemy types;
- multiplayer;
- elaborate asset pipeline;
- procedural campaign complexity;
- publishing workflow beyond basic smoke tests.

### MVP Success Question

The MVP should answer:

> Is it fun to mine, defend, allocate matter, build a child probe, and choose when to launch under escalating attack?

If the answer is yes, later systems can be layered on top.

---

## Possible Implementation Architecture

This section is preliminary and intended to seed future design work.

### Candidate Runtime Concepts

Core game systems may include:

- game loop;
- entity/component model or lightweight object model;
- vector scene graph or draw-command layer;
- collision system;
- input system;
- resource economy;
- shield system;
- mining system;
- child-probe construction system;
- enemy spawner;
- enemy AI behaviors;
- level / sector state;
- launch state machine;
- scoring / progression system;
- render snapshot tests.

### Possible Python Package Layout

If implemented as a Python project:

```text
replication-vector/
  README.md
  pyproject.toml
  src/
    replication_vector/
      __init__.py
      main.py
      game.py
      model.py
      render.py
      input.py
      physics.py
      collision.py
      resources.py
      mining.py
      shields.py
      child_probe.py
      enemies.py
      levels.py
      progression.py
      testing.py
  tests/
    replication_vector_tests/
      test_resources.py
      test_shields.py
      test_child_probe.py
      test_enemy_behaviors.py
      test_launch_rules.py
      test_render_smoke.py
  scripts/
    develop
    lint
    format
    test
    smoke
    render-smoke
  project/
    README.md
    goal.md
    design/
    roadmap.md
    current_focus.md
    work_items/
```

Actual layout should follow whatever packaging conventions are chosen for the project and the wider Logical Robotics / Velumin ecosystem.

---

## Relationship to Velumin

### Repository Decision

**Replication Vector** should live in its own repository rather than inside the Velumin source tree.

Rationale:

- it has different release needs;
- it will include game-specific assets, docs, tuning, and tests;
- it should validate Velumin as an external dependency;
- it should avoid cluttering the Velumin library repo;
- it can evolve as a game without destabilizing the library;
- it can reveal packaging and API issues that internal examples might hide.

### What Should Stay in Velumin

Velumin should keep small focused examples such as:

- vector ship demo;
- shield arcs demo;
- asteroid outline demo;
- collision overlay demo;
- particle trail demo.

These examples should demonstrate Velumin APIs, not implement the full game.

### What Should Live in Replication Vector

Replication Vector should own:

- gameplay loop;
- enemy AI;
- child-probe construction;
- mining mechanics;
- game-specific vector assets;
- level tuning;
- scoring;
- game UI;
- game-specific tests;
- design documents;
- release packaging.

### Dependency Pattern

Replication Vector should depend on Velumin like a real downstream project.

Possible local development approach:

```bash
python -m pip install -e ../velumin
python -m pip install -e .
```

Possible CI approach:

```bash
python -m pip install "velumin @ git+https://github.com/xenotaur/velumin.git@main"
python -m pip install -e .
```

Exact commands should be adapted to the final package and repository names.

---

## CI and Testing Strategy

### Initial Strategy

The new game may copy Velumin’s CI/scripts initially for speed, but should avoid letting them drift indefinitely.

Recommended approach:

1. Copy a lightweight version of Velumin’s CI and scripts.
2. Parameterize obvious project names and paths.
3. Keep local scripts as the stable developer interface.
4. Use the game to identify reusable Velumin test helpers.
5. Extract reusable helpers only after duplication becomes real and stable.

### Repo-Local Scripts

Recommended scripts:

```text
scripts/develop
scripts/lint
scripts/format
scripts/test
scripts/smoke
scripts/render-smoke
```

These match the user’s broader project style and should be easy to run locally and in CI.

### Reusable CI Candidates

Potential reusable pieces:

- Python environment setup;
- lint/format/test conventions;
- render smoke testing;
- vector-scene golden tests;
- workflow YAML validation;
- artifact upload for failed render diffs;
- deterministic replay testing.

### Preferred Long-Term Reuse Pattern

Share testing helpers and conventions first, not entire CI workflows.

Velumin might eventually expose:

```text
velumin.testing
velumin.testing.fixtures
velumin.testing.goldens
velumin.testing.geometry
velumin.testing.headless
```

Replication Vector can then use those helpers while keeping its own project-specific workflow files.

Long-term ideal:

```text
velumin/
  src/velumin/testing/

replication-vector/
  tests/test_render_scenes.py  # uses velumin.testing
```

### Avoid Premature Centralization

Do not centralize all CI immediately. Game CI and library CI will differ.

Velumin CI should focus on:

- library tests;
- API stability;
- rendering primitives;
- examples;
- packaging.

Replication Vector CI should focus on:

- gameplay tests;
- render smoke tests;
- deterministic simulation tests;
- asset validation;
- downstream integration with Velumin.

---

## LRH Project Bootstrap Considerations

This design can bootstrap a Logical Robotics Harness `project/` directory.

### Candidate `project/goal.md`

Potential project goal:

> Build **Replication Vector**, a retro vector-graphics arcade survival game and Velumin dogfood project in which the player controls a self-replicating von Neumann probe that mines asteroids, builds defenses, survives anti-replicator attacks, constructs a child probe, and chooses when to launch the next generation.

### Candidate Core Principles

Potential principles:

1. **Core loop first** — prioritize the mine/defend/replicate/launch loop before adding campaign depth.
2. **Readable vector gameplay** — every important mechanic should be visually legible in vector form.
3. **Dogfood Velumin honestly** — use Velumin as a downstream library, not through private shortcuts.
4. **Small MVP, expandable architecture** — avoid premature tech trees, maps, and complex economies.
5. **Meaningful launch decision** — the player’s choice of when to launch must affect risk and future state.
6. **Present versus future tradeoff** — resources spent on parent survival should compete with child-probe investment.
7. **Testable systems** — gameplay systems should support deterministic tests where practical.
8. **Arcade immediacy** — avoid UI and simulation complexity that weakens moment-to-moment play.

### Candidate Roadmap Phases

#### Phase 0 — Project Bootstrap

- Create repository.
- Add package skeleton.
- Add LRH `project/` directory.
- Add initial design docs.
- Add scripts: `develop`, `lint`, `format`, `test`, `smoke`.
- Add initial CI.
- Add Velumin dependency path.

#### Phase 1 — Rendering Spike

- Render parent probe.
- Render asteroid outlines.
- Render shield arcs.
- Render bullets and simple enemies.
- Verify Velumin integration.
- Add render smoke tests.

#### Phase 2 — Core Simulation

- Implement movement.
- Implement mining beam.
- Implement matter resource.
- Implement shield construction and damage.
- Implement basic collision.
- Implement parent integrity.

#### Phase 3 — Child-Probe Objective

- Add visible child-probe construction.
- Add construction states.
- Add launch thresholds.
- Add launch sequence.
- Add next-sector transition or scoring.

#### Phase 4 — Enemy Pressure

- Add three MVP enemy types.
- Add enemy spawning and escalation.
- Add simple AI behaviors.
- Add target priorities: parent, shields, child probe.

#### Phase 5 — Playable MVP

- Integrate systems into a complete loop.
- Tune resource costs and enemy pressure.
- Add basic UI.
- Add game-over and launch-success states.
- Add smoke tests and deterministic simulation tests.

#### Phase 6 — Post-MVP Expansion

- Add upgrade inheritance.
- Add sector variety.
- Add additional enemy types.
- Add better audio and visual effects.
- Add packaging/distribution targets.

### Candidate Workstreams

Potential LRH workstreams:

1. **Project Infrastructure**
   - repository setup;
   - package skeleton;
   - scripts;
   - CI;
   - LRH project files.

2. **Velumin Integration**
   - external dependency setup;
   - rendering adapters;
   - render smoke tests;
   - feedback to Velumin APIs.

3. **Core Gameplay Loop**
   - mining;
   - resource allocation;
   - shield construction;
   - child-probe construction;
   - launch.

4. **Combat and Enemies**
   - weapons;
   - collision;
   - enemy AI;
   - enemy spawning;
   - escalation.

5. **Visual and Audio Presentation**
   - vector art style;
   - effects;
   - UI;
   - audio cues.

6. **Testing and Determinism**
   - unit tests;
   - simulation tests;
   - render smoke tests;
   - replay tests;
   - future golden-scene tests.

7. **Design and Tuning**
   - balancing;
   - playtest notes;
   - difficulty curves;
   - launch thresholds;
   - enemy pressure.

### Candidate Initial Work Items

Initial work items could include:

1. Create project repository skeleton.
2. Add LRH `project/` bootstrap files.
3. Add Velumin dependency and minimal render window.
4. Implement vector parent probe and asteroid rendering demo.
5. Implement shield arc rendering and damage state demo.
6. Implement basic game loop and input handling.
7. Implement matter resource and mining beam.
8. Implement shield construction / repair model.
9. Implement child-probe construction state model.
10. Implement launch threshold and launch sequence prototype.
11. Implement first enemy type: skirmisher.
12. Implement second enemy type: cutter / breacher.
13. Implement third enemy type: seeker / hunter.
14. Integrate MVP loop.
15. Add render smoke tests.
16. Add deterministic simulation tests.
17. Tune MVP difficulty.

---

## Naming Decision

The selected title is:

> **Replication Vector**

### Why This Title Works

It has a useful double meaning:

- **Replication** references the von Neumann probe and the central game objective.
- **Vector** references the visual style and Velumin’s vector graphics focus.

It is more distinctive than generic names such as `Starfort`, `Star Siege`, or `Shieldwar`, and it avoids being too directly tied to *Star Castle*.

### Naming Notes

Other considered titles included:

- VNP;
- VNP: Von Neumann Probe;
- VNP: Replicator Under Siege;
- Childprobe;
- The Last Replicator;
- Neumann Siege;
- Probe Prime;
- Precious Cargo;
- Life Preserver;
- Life Preservation Engine.

Strong alternatives:

- **VNP: Replicator Under Siege** — clear but long.
- **The Last Replicator** — emotionally accessible but more generic.
- **Childprobe** — memorable but tonally risky and potentially confusing.

Final recommendation:

```text
REPLICATION VECTOR
Mine. Defend. Replicate.
```

---

## Key Risks

### Scope Creep

The premise invites many expansions: tech trees, galactic maps, mutation, enemy adaptation, rival replicators, and story campaigns. These should be deferred until the core loop is proven.

Mitigation:

- define a strict MVP;
- keep one resource initially;
- use three enemy types initially;
- avoid full tech tree;
- make the first milestone a playable loop, not a full campaign.

### Passive Player Feel

A slow fortress-like ship could feel inert or boring.

Mitigation:

- keep the player actively aiming, mining, repairing, building, and launching;
- make shield placement spatial and time-sensitive;
- include emergency actions;
- keep enemy pressure dynamic.

### Visual Clutter

Vector shields, bullets, asteroids, enemies, beams, particles, and UI could become unreadable.

Mitigation:

- establish a clear visual grammar;
- limit simultaneous effects in MVP;
- use distinct line styles and shapes;
- prioritize gameplay readability over visual flourish.

### Unclear Player Motivation

Self-replication can read as invasive or morally ambiguous.

Mitigation:

- decide whether the VNP is an explorer, archive, survivor, or ambiguous replicator;
- keep the MVP fiction simple;
- optionally frame the probe as a preservation or life-archive system.

### Launch Decision Becomes Obvious

If optimal play is always “wait until 100%,” the central decision loses meaning.

Mitigation:

- escalate enemy pressure over time;
- make launch take time and attract attacks;
- let partial launches be viable;
- make overbuilding useful but risky;
- allow parent damage and resource depletion to pressure timing.

### CI / Test Reuse Complexity

Velumin and Replication Vector may share CI needs, but premature centralization could create brittle abstractions.

Mitigation:

- copy and adapt lightly at first;
- parameterize obvious project paths;
- extract reusable test helpers later;
- keep workflows local until reuse is proven.

---

## Open Design Questions

These questions should be resolved through prototypes and design docs:

1. Is the parent probe mostly stationary, slow-moving, or fully Asteroids-like?
2. Does mining require docking, close-range beam contact, or remote beam targeting?
3. Where is the child probe built: inside the parent, beside it, or in an external cradle?
4. Can enemies directly damage the child probe before launch?
5. Do shields protect only the parent, or also the child construction site?
6. Does the main weapon require shield gaps or firing lanes?
7. What exactly carries over between generations?
8. Is the player morally a preservation probe, neutral replicator, or ambiguous machine lifeform?
9. Should the game be endless arcade, roguelite run, or structured campaign?
10. What minimum rendering APIs must Velumin provide for this game to feel clean?
11. What is the target runtime platform: desktop Python, browser, packaged executable, or multiple?
12. What is the minimum viable audio system?
13. How deterministic should gameplay simulation be for testing and replay?
14. Should render tests compare images, vector command streams, or both?
15. How should LRH work items be structured across game, Velumin integration, and design tuning?

---

## Recommended Next Steps

### Immediate Next Step

Create the new project repository and seed its LRH `project/` directory using this document as the source design summary.

### Suggested First PR

A first PR could:

- create repository skeleton;
- add README.md;
- add package skeleton;
- add `project/goal.md`;
- add `project/design/replication_vector_design_summary.md`;
- add `project/roadmap.md`;
- add `project/current_focus.md`;
- add initial work item directory structure;
- add basic scripts;
- add minimal CI placeholder.

### Suggested First Technical Spike

After project bootstrap, the first technical spike should render:

- parent VNP outline;
- asteroid outline;
- shield arcs;
- simple projectile lines;
- minimal input loop.

The goal should be to verify Velumin integration before implementing too much gameplay.

### Suggested First Gameplay Spike

The first gameplay spike should implement:

- mining beam;
- matter counter;
- shield build/repair;
- one enemy type;
- parent damage;
- child-probe progress;
- launch command.

The goal should be to prove the loop:

> mine → build/repair → survive → construct child → launch.

---

## Final Design Thesis

**Replication Vector** should be a tight vector arcade survival game about defending a self-replication cycle under escalating attack.

Its strongest idea is the player-triggered level ending:

> You do not win a sector by clearing the screen or reaching an exit. You win by deciding that your successor is ready, opening a launch window, and surviving long enough for the child probe to escape.

This makes the design distinct, mechanically coherent, and well aligned with Velumin’s strengths.
