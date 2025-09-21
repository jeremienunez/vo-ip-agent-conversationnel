# Changelog

All notable changes to the VoIP Server project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### To Add
- Real SIP signalling implementation (REGISTER, INVITE, BYE)
- RTP media relay functionality
- NATS event bus connection
- OpenTelemetry with new API
- Service discovery with Consul
- Authentication and authorization
- Database persistence

## [0.1.0] - 2025-09-21

### Added
- Initial project structure with 5 active crates
- Basic HTTP API server with health/info endpoints
- Signalling service with heartbeat monitoring
- Protocol buffer definitions for all services
- Common error handling framework
- Basic telemetry with JSON logging
- Event bus abstraction (not connected)
- Media relay structure (stub)
- Build and deployment scripts

### Changed
- Migrated from Rust 1.82 to 1.90 (September 2025)
- Updated all dependencies to latest versions:
  - tokio 1.41 → 1.47
  - axum 0.7
  - tonic 0.12
  - async-nats 0.38
  - opentelemetry 0.27
- Simplified workspace from 20 planned crates to 5 active
- Unified Result type to use `voip_common::Result`
- Consolidated telemetry implementation in common crate

### Fixed
- 212 compilation errors from outdated API usage
- tonic-build: `compile()` → `compile_protos()`
- async-nats error handling with proper Boxing
- Proto module structure with correct imports
- ConfigError conversion to VoipError
- Lifetime issues in EventBus publish/subscribe

### Removed
- Legacy `/src` directory structure
- Duplicate telemetry implementation in core
- Serde derives from proto types (prost_types incompatibility)
- 15 non-existent crates from workspace
- anyhow dependency from service crates

### Security
- All dependencies updated to latest secure versions
- No known vulnerabilities in current stack

## [0.0.1] - 2025-09-20

### Added
- Initial project scaffolding
- Basic Cargo workspace setup
- Proto definitions for common types
- README with project vision
- Architecture documentation structure

---

## Version History Summary

- **0.1.0** (2025-09-21): First functional version with basic services running
- **0.0.1** (2025-09-20): Initial project creation and planning