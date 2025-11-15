# [@pr-note/pr-note](https://www.npmjs.com/package/@pr-note/pr-note)

A command-line tool to generate PR notes summarizing unmerged PRs on GitHub between two branches.

## Installation

```bash
# If you want to install globally
npm install -g @pr-note/pr-note
# If you want to install as a development dependency in a project
npm install -D @pr-note/pr-note
```

## Usage

For example, to create a release pull request from the `feature` to the `main` in the [octocat/Hello-World](https://github.com/octocat/Hello-World) repository, do the following:

```bash
pr-note \
--owner octocat \
--repo Hello-World \
--base main \
--head feature \
--token <github_token>
```

Alternatively, if environment variables are set in a CI/CD environment, corresponding arguments can be omitted as follows:

```bash
export PR_NOTE_GITHUB_TOKEN="<GITHUB_TOKEN>"

pr-note \
--owner octocat \
--repo Hello-World \
--base main \
--head feature
```

For more details on usage, options, and templates, please refer to the [Rust version documentation](/README.md).

## Options

```bash
$ pr-note --help
Generate or update a GitHub Pull Request (PR) note summarizing unmerged PRs between two branches.

Usage: pr-note [OPTIONS] --owner <OWNER> --repo <REPO> --base <BASE> --head <HEAD> --token <TOKEN>

Options:
      --host <HOST>
          API host domain for GitHub. Default is "api.github.com".
          For GitHub Enterprise, set your enterprise host domain.

          [env: PR_NOTE_GITHUB_HOST=]
          [default: api.github.com]

  -o, --owner <OWNER>
          The owner of the repository.

          [env: PR_NOTE_REPO_OWNER=]

  -r, --repo <REPO>
          The name of the repository.

          [env: PR_NOTE_REPO_NAME=]

  -b, --base <BASE>
          The name of the base branch.

          [env: PR_NOTE_BASE_BRANCH=]

  -a, --head <HEAD>
          The name of the head branch.

          [env: PR_NOTE_HEAD_BRANCH=]

  -t, --token <TOKEN>
          GitHub API token with appropriate repository permissions:
            - Contents: read
            - Metadata: read
            - Pull requests: write

          [env: PR_NOTE_GITHUB_TOKEN]

  -p, --template-path <TEMPLATE_PATH>
          Template file path for PR title and body.
          First line is used as the title, rest as the body.
          If not specified, the default template will be used.

          [env: PR_NOTE_TEMPLATE_PATH=]

  -g, --group-by <GROUP_BY>
          Grouping options for unmerged PRs.
          Options are "label" or "title".

          [env: PR_NOTE_GROUP_BY=]
          [possible values: label, title]

  -d, --dry-run
          Dry run mode.
          With this option, no PR will be created or updated, only output the generated text to stdout.

          [env: PR_NOTE_DRY_RUN=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
