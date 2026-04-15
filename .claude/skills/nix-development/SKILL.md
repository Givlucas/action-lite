---
name: nix-development-philosophy
description: "REQUIRED before creating ANY Nix flakes, derivations, or NixOS configurations. MUST invoke FIRST for all Nix-related work. Contains mandatory Nix standards including pinning, purity, and flake structure requirements. Do NOT proceed without reading."
---

Always use nix when creating software. Even scripts should have their own repo with their own flake.

# Specific behaviors
- Always use flakes.
- Always pin to a specific version of nixos (the latest) never pin directly to nixos unstable.
- Always include a nix "check" option with flakes. Check option should run project standard unit tests
- Always build for all systems using the flakeutils repo
- Break up flakes into multiple nix modules. Avoid defining derivations / packages / etc directly in the flake. de-composed modules should go into nix/. 
- Always provide a development shell with all packages needed to build and run the software avaiable.
- Never install programs using nix profile or nix-env install. Use of nix-shell -p for all one off use cases. Always prefer adding packages to flake / derivation
- Always use PURE nix. Never allow or use impure systems. If you enounter the percived need to do this stop immediatly and alert the user.
- Never write software that will requires a FHS env
