# STATEMENT ON LLM USE
This project differs from my other respositories in the scope of LLM usage. While I myself am a LLM skeptic / hater I find it necessary to keep up with and evaluate the capabilites and scope these tools can handle. Unlike my other repo's this project is to be written paritially or fully by LLMs. In general I avoid using _any_ LLM generated code in personal projects unless for extremely tedious or simple tasks, as I believe it is best for my professional development. While the code my be AI written the action work flow is entirely of my own design. [Please see my statement on LLM use for my opinions](https://github.com/Givlucas/LLM-instructions/blob/main/Statement%20on%20LLM%20use.md)

# action-lite

An agile, file-based task tracking system that uses acyclic directed metagraphs to track task requirements, state, strategy, and dependency.

## Overview

action-lite directly links task management to architecture design. Actions are markdown documents that declare expected outcomes and how to achieve them in plain English. By storing architecture requirements as actionable tasks, there's no need to translate large architecture documents into individual work items.

## Key Concepts

- **Actions** - Markdown files stored in the `actions/` directory with frontmatter metadata
- **Metagraphs** - Directories that decompose complex actions into sub-actions
- **States** - Actions progress through: discovery → design → implementation → test → document → published
- **Inputs** - Actions declare dependencies on other actions via markdown links

## Getting Started

Actions are stored as markdown files in the `actions/` directory. Each action includes:

- Frontmatter metadata (owners, state, priority, continuous)
- Notes section for research and learnings
- Statement of Action describing the task
- Statement of Inputs listing dependencies
- Statement of Specifications defining completion criteria
- Statement of Design detailing the implementation approach

## Documentation

For the complete architecture and workflow documentation, see:

- [Create action-lite protocol](actions/Create%20action-lite%20protocol.md) - Full system design and workflow specification
