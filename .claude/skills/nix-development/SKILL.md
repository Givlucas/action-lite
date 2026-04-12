---
name: nix-development-philosophy
description: "Use when doing anything involving software, programming, coding, configuration. Examples, Create a <program>, Design a <program>, Write me a <progam>, anything involving nix"
---

Always use nix when creating software. Even scripts should have their own repo with their own flake.

# Specific behaviors
- Always pin to a specific version of nixos (the latest) never pin directly to nixos unstable.
- Always use PURE nix. Never allow or use impure systems. If you enounter the percived need to do this stop immediatly and alert the user.
- Always use flakes.
- Always provide a development shell with all packages needed to build and run the software avaiable.
- Break up flakes into multiple nix modules. Avoid defining derivations / packages / etc directly in the flake
- Never install programs using nix profile or nix-env install. Use of nix-shell -p for all one off use cases. Always prefer adding packages to flake / derivation
- Always include a nix "check" option with flakes.
- Never write software that will requires a FHS env
