# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.1.0](https://github.com/EgorHenek/ai-commit/compare/7021ab5a69e7b6fbe42aa36abe5610c8700a7b8b..0.1.0) - 2024-08-20
#### Bug Fixes
- set fetch-depth to 0 in release workflow - ([4c6c9a0](https://github.com/EgorHenek/ai-commit/commit/4c6c9a0ad82d1679728aae87c272d556e31e82b6)) - [@EgorHenek](https://github.com/EgorHenek)
- Update BaseProvider to use configurable base URL - ([98cdf3c](https://github.com/EgorHenek/ai-commit/commit/98cdf3c923f283012361c8eee151380b643ba3e8)) - [@EgorHenek](https://github.com/EgorHenek)
- correct default AI model in help text and improve error handling for API keys - ([19045e7](https://github.com/EgorHenek/ai-commit/commit/19045e73fdc83d10adf66e7e6fa4445759ac7b8d)) - [@EgorHenek](https://github.com/EgorHenek)
- resolve async_trait import and usage - ([458bbf6](https://github.com/EgorHenek/ai-commit/commit/458bbf671f103944972a3fb6a75c2dd80378b354)) - [@EgorHenek](https://github.com/EgorHenek)
- add error handling for API interaction - ([0146952](https://github.com/EgorHenek/ai-commit/commit/0146952e23af9675b23775b551dd59226d75f1c5)) - [@EgorHenek](https://github.com/EgorHenek)
- update environment variable name for API key - ([869effa](https://github.com/EgorHenek/ai-commit/commit/869effa018257354c69d4ef27f188ac0bae48ccd)) - [@EgorHenek](https://github.com/EgorHenek)
- update clap methods to resolve compilation errors - ([31af616](https://github.com/EgorHenek/ai-commit/commit/31af616df4ecadafccca29906f6891610dc8c504)) - [@EgorHenek](https://github.com/EgorHenek)
#### Documentation
- add README file in English - ([5151c4b](https://github.com/EgorHenek/ai-commit/commit/5151c4b7adb613514cae2119aaceac19b81e951b)) - [@EgorHenek](https://github.com/EgorHenek)
#### Features
- add workflow_dispatch trigger to release workflow - ([e37b173](https://github.com/EgorHenek/ai-commit/commit/e37b17340b3357d09b9478d8d0fa6478905bf212)) - [@EgorHenek](https://github.com/EgorHenek)
- add authors information to cog.toml - ([880c709](https://github.com/EgorHenek/ai-commit/commit/880c709b312c467a0c15dcb0929b14084dd6f766)) - [@EgorHenek](https://github.com/EgorHenek)
- add test configuration for set_base_url method - ([c0d8fa8](https://github.com/EgorHenek/ai-commit/commit/c0d8fa8e9f5760292057d9d5af0e4804b61fb004)) - [@EgorHenek](https://github.com/EgorHenek)
- Add GitHub Actions pipeline for Rust code checks - ([97e3c1c](https://github.com/EgorHenek/ai-commit/commit/97e3c1c373e70cb7cae0e01ed493d43f6ff884bc)) - [@EgorHenek](https://github.com/EgorHenek)
- Update providers and tests to resolve errors - ([6e4e61b](https://github.com/EgorHenek/ai-commit/commit/6e4e61bb60c0f7108e7756ccc82c56543db26e3e)) - [@EgorHenek](https://github.com/EgorHenek)
- Add new test cases for OpenAI and OpenRouter providers - ([fc572c9](https://github.com/EgorHenek/ai-commit/commit/fc572c99fb79ebf7ed8c036cc603b4b5f75d3ff4)) - [@EgorHenek](https://github.com/EgorHenek)
- Add support for listing available models - ([8bfbe21](https://github.com/EgorHenek/ai-commit/commit/8bfbe21f4028ff54f17e518c8b60bb6338a02874)) - [@EgorHenek](https://github.com/EgorHenek)
- Add command to list available models for the selected provider - ([497ec07](https://github.com/EgorHenek/ai-commit/commit/497ec077c143e2e2b3e0ac3e34f8723fba31ce46)) - [@EgorHenek](https://github.com/EgorHenek)
- Replace default model with gpt-4-mini and update help text - ([5a372e3](https://github.com/EgorHenek/ai-commit/commit/5a372e36531b399352cebb8787fe67232c70f0b8)) - [@EgorHenek](https://github.com/EgorHenek)
- Separate AI_API_KEY into OPENROUTER_API_KEY and OPENAI_API_KEY - ([167248d](https://github.com/EgorHenek/ai-commit/commit/167248db9bfb39f1313e50232605f6f5094dce7e)) - [@EgorHenek](https://github.com/EgorHenek)
- Add OpenRouter provider with custom settings and environment variables - ([873e22a](https://github.com/EgorHenek/ai-commit/commit/873e22abcd49d9fe0c187766b550aa6a8cfec464)) - [@EgorHenek](https://github.com/EgorHenek)
- add anyhow dependency for error handling - ([1d04108](https://github.com/EgorHenek/ai-commit/commit/1d04108a5d9f730d7acda5ecf01c9a096797c8f4)) - [@EgorHenek](https://github.com/EgorHenek)
#### Miscellaneous Chores
- update GitHub Actions workflow to include cross-platform binary publishing and add Cross.toml configuration - ([7e70920](https://github.com/EgorHenek/ai-commit/commit/7e70920ab1b381cf8428b961644b2b5ea62af213)) - [@EgorHenek](https://github.com/EgorHenek)
- update GitHub Actions permissions for release workflow - ([838728a](https://github.com/EgorHenek/ai-commit/commit/838728abf8af89d4929c15085d01e80a1739224f)) - [@EgorHenek](https://github.com/EgorHenek)
- update GitHub Actions checkout action to v4 and clean up CI workflow syntax - ([3b4fad9](https://github.com/EgorHenek/ai-commit/commit/3b4fad9756e0745d874a3bceb9273a9c60856a65)) - [@EgorHenek](https://github.com/EgorHenek)
- fix cog.toml - ([09700ff](https://github.com/EgorHenek/ai-commit/commit/09700ff997aff64ad8719c5a4ccd5c3ae02f5ce4)) - [@EgorHenek](https://github.com/EgorHenek)
- Add MIT license - ([e2c0138](https://github.com/EgorHenek/ai-commit/commit/e2c0138fa086b8f50b41e4d81d0ccc8bce5fc676)) - [@EgorHenek](https://github.com/EgorHenek)
- update dependencies and improve project metadata in Cargo.toml and Cargo.lock - ([0407ea9](https://github.com/EgorHenek/ai-commit/commit/0407ea9e7bd1bb0c0beeb17f5dce95b91b2842f9)) - [@EgorHenek](https://github.com/EgorHenek)
- init - ([7021ab5](https://github.com/EgorHenek/ai-commit/commit/7021ab5a69e7b6fbe42aa36abe5610c8700a7b8b)) - [@EgorHenek](https://github.com/EgorHenek)
#### Refactoring
- Simplify mock setup in provider tests - ([18ca8eb](https://github.com/EgorHenek/ai-commit/commit/18ca8eb9b8f3f8419534e5371bb5cbe84a8b0b77)) - [@EgorHenek](https://github.com/EgorHenek)
- Extract common logic to BaseProvider - ([b824364](https://github.com/EgorHenek/ai-commit/commit/b824364c2a935042ffa78f963c3c492f377ab106)) - [@EgorHenek](https://github.com/EgorHenek)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).