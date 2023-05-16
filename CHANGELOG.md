# Changelog

## [1.0.0](https://github.com/nvim-neorg/neorg-breeze/compare/v0.1.0...v1.0.0) (2023-05-16)


### ⚠ BREAKING CHANGES

* move back to rusty_pool as it has no memory leaks
* move to `threadpool`, fix excess memory usage

### Features

* add `c_functions.rs` ([4323904](https://github.com/nvim-neorg/neorg-breeze/commit/4323904d73804087229189b557378730b7308e82))
* add preliminary `c_functions.rs` code ([2060eb6](https://github.com/nvim-neorg/neorg-breeze/commit/2060eb6c63388f7a8df635fad8ac769d99ed27f4))
* **breeze.rs:** make `parse_files` take in a number of jobs ([db2c87c](https://github.com/nvim-neorg/neorg-breeze/commit/db2c87c78218b3b4b96e8cc7219701635a1105ec))


### Bug Fixes

* **c_functions:** correct ManuallyDrop scope ([9c289c3](https://github.com/nvim-neorg/neorg-breeze/commit/9c289c3da8f2e7fb65818193b29ab6a51663af2b))
* **c_functions:** fix memory leak in test ([3488255](https://github.com/nvim-neorg/neorg-breeze/commit/34882556a55606f9c34ef9b1e23e50b5c7b2ac34))


### Performance Improvements

* don't create many `norg` languages ([4f8bca8](https://github.com/nvim-neorg/neorg-breeze/commit/4f8bca83e46d9a2e2c45afa115985bf917f2d48d))
* move to `threadpool`, fix excess memory usage ([7c158ae](https://github.com/nvim-neorg/neorg-breeze/commit/7c158ae80677515f73e80ae5bd5b47949687f502))


### Code Refactoring

* move back to rusty_pool as it has no memory leaks ([3683c0c](https://github.com/nvim-neorg/neorg-breeze/commit/3683c0c5619acb02f0e3b23bab61afa4682e0077))

## 0.1.0 (2023-03-11)


### Features

* initial commit ([e5644ef](https://github.com/nvim-neorg/neorg-breeze/commit/e5644efc652520825c235aa239a4ab5519e23a7c))
