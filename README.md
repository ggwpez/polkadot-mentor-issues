# Polkadot Mentor Issue Board

Lists some good issues for people that want to work on the Polkadot SDK. The list is updated every 6 hours.

# Deployment

It is a single binary without the need for a static asset directory. Build and start with:  
```bash
cargo install --path .
polkadot-mentor-issues --endpoint 0.0.0.0 --port 8080
```

# Columns

Explanation for each row that you can see on [mentor.tasty.limo](https://mentor.tasty.limo):

![Overview](static/twitter.png)

## Title

The title as given by the author of the issue.

## Difficulty

An estimated difficulty as given by the author.

## Status

Whether this issues is `Free` or `Taken`. It currently does not know whether there is a Merge Request attached, since the GitHub API does not easily expose this.

## Type

A best-effort classification of the type of this issue.

## Author

Who initially created this issue.

# License 

The SPDX license identifier is GPL-3.0-only. See [LICENSE](LICENSE).
