# Contributing

Thanks for taking the time to contribute!

Open source thrives on community input, so whether you're fixing a typo, squashing a bug, or adding a new feature - you're awesome

This guide will help you get started

## How to Contribute

It's pretty simple

1. Fork the repository
2. Create your branch:
    ```Shell
    git checkout -b my-cool-feature
    ```
3. Make changes
4. Commit:
    ```Shell
    git commit -m "feat: add cool button"
    ```
5. Push:
    ```
    git push origin my-cool-feature
    ```
6. Open a Pull Request with a clear description

We'll review, discuss if needed, and merge once it's ready

## What You Can Contribute

Basically everything:

* Bug reports
* Fixes and refactors
* New features
* Documentation improvements
* Tests and test fixes

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org) standard, so your commit messages should follow this format:

```
<type>[optional scope]: <description>
```

Preferred type labels and their use cases:

* `feat` - A new feature implementation
* `impr` - An existing feature improvement
* `fix` - A bug fix
* `docs` - Documentation updates
* `refactor` - A code change that neither fixes a bug nor adds a feature
* `build` - Build system changes or dependencies changes
* `chore` - Everything else

Example commit messages:

* `feat: add cool button`
* `impr: optimize cool button so it will work faster`
* `fix: fix cool button didn't work on Mondays`
* `docs(README.md): add "Configuration" section`
* `refactor: remove unused block of code`
* `build(Cargo.toml): add "serde" dependency`
* `chore: update demo_1.png`

## Pull Request Guidelines

* One feature per PR
* Keep your PR description short but informative
* Tag relevant issues
* Make sure it builds

We'll review as soon as possible!

## License

By contributing, you agree that your code will be licensed under the [GPL-3.0-or-later](/LICENSE)
