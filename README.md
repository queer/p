# p

A very opinionated, zero-configuration shell prompt

![preview](https://cdn.mewna.xyz/2021/10/28/c0KkyIcwCR981.png)

## Format

```
# Normal
# Arrow turns yellow on non-zero last exit status, red otherwise
▶

# In a git repo
# Branch name is shown in red, with a blue git indicator
# Number of uncommitted changes is in yellow if applicable
git:(mistress) ▶
git:(mistress) 1 ▶
```

## Usage

`p` only supports `fish`.

Add the following to your `config.fish`:

```fish
p source | source
```