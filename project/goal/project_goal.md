---
id: GOAL-CORE
title: Build Replication Vector
status: active
owner: project maintainers
time_horizon: long
---

# Project Goal

## Objective
Build **Replication Vector**, a retro vector-graphics arcade survival game and Velumin dogfood project in which the player controls a self-replicating von Neumann probe that mines asteroids, builds defenses, survives anti-replicator attacks, constructs a child probe, and chooses when to launch the next generation.

## Intended Outcome
- A tight, readable, skill-based vector arcade game.
- A playable MVP proving the loop: mine -> defend -> allocate matter -> build child probe -> launch successor.
- A downstream validation project for Velumin's vector rendering APIs, packaging, test helpers, and documentation.
- A small but expandable project foundation for later progression, upgrades, sectors, and presentation polish.

## Intended Users / Stakeholders
- Players who enjoy retro vector arcade survival and tactical resource-pressure games.
- Project maintainers designing and implementing Replication Vector.
- Velumin maintainers and contributors using the game as downstream dogfood.
- Future agents using LRH artifacts to interpret priorities, constraints, and evidence.

## In Scope
- A parent von Neumann probe with slow, heavy arcade movement.
- Matter mining from asteroids.
- Shield arc construction and repair.
- Child-probe construction and player-triggered launch.
- Anti-replicator enemy pressure with a small MVP enemy set.
- Minimal vector UI for matter, integrity, shield state, child viability, and launch readiness.
- Render smoke tests and deterministic gameplay tests where practical.

## Out of Scope (Initial)
- Full campaign, galactic map, or roguelite metagame.
- Multiple resource economy.
- Large enemy roster.
- Multiplayer.
- Elaborate story, save/load, or asset pipeline.
- Centralized shared CI abstractions before local project needs are proven.

## Success Direction
- The MVP can answer whether it is fun to mine, defend, allocate matter, build a child probe, and choose when to launch under escalating attack.
- Velumin integration exposes concrete downstream rendering, packaging, and testing feedback.
- Future scope decisions remain anchored to the core replication-under-siege loop.
