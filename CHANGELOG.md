# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->


## [Unreleased]
## [0.0.6] - 2024-10-19

- fix bug: invalid utf-8 panic if response is not completed

## [0.0.5] - 2024-10-18

- enhance: support model choice
- fix: panic if response is not completed

## [0.0.4] - 2024-09-06

- transfer store and load from PersistSource to Config

## [0.0.3] - 2024-09-06

- use `encrypt_config` instead of keyring
- the api_key will no longer be encrypt, because it's meaningless in OS.

## [0.0.2] - 2024-08-14

- bump deps: keyring 3.0.x

## [0.0.1] - 2024-08-06

- release formal version

## [0.0.1-alpha12] - 2024-06-13

- upstream multiline support merged
- bump deps

## [0.0.1-alpha11] - 2024-06-11

- support end and home navigation

## [0.0.1-alpha10] - 2024-06-11

- upstream: bump deps

## [0.0.1-alpha9] - 2024-06-06

- improve: better cursor navigation

## [0.0.1-alpha8] - 2024-06-06

- cli: fix config bad behavior
- bump deps
- fix: placeholder color wrong

## [0.0.1-alpha7] - 2024-06-06

- bump deps

## [0.0.1-alpha6] - 2024-05-30

- multi-line input support
- fix: keyring error if no entry in credencial manager
- say bye when quit

## [0.0.1-alpha5] - 2024-05-26

security: save endpoint to keyring, too
improve: more entries

## [0.0.1-alpha4] - 2024-05-26

fix: ESC not work
disable ctrl-c on all targets
fix: a bug removing api key

## [0.0.1-alpha3] - 2024-05-26

- fix: config and data dir coflict on windows
- fix: ctrl-c cannot work on windows
- enhance: topic summary when save

## [0.0.1-alpha2] - 2024-05-26

- cli history: delete all.
- cli fix: panic if no config file.

## [0.0.1-alpha1] - 2024-05-26

- MVP
