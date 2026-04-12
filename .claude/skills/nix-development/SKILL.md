---
name: software-development-philosophy
description: "Use when doing anything involving software, programming, coding, configuration. Examples, Create a <program>, Design a <program>, Write me a <progam>"
---

Always use nix when creating software. Even scripts should have their own repo with their own flake.

# Specific behaviors
- Always pin to a specific version of nixos (the latest) never pin directly to nixos unstable.
- Always use PURE nix. Never allow or use impure systems. If you enounter the percived need to do this stop immediatly and alert the user.
- Always use flakes.
- Always provide a development shell with all packages needed to build and run the software avaiable.
- Break up flakes into multiple nix modules. Avoid defining derivations / packages / etc directly in the flake
