# Maskedu 🤖

Maskedu an automated Twitter scheduling bot built with Rust.

Step-by-step walkthrough video would be released [here](https://rust-telescope.live/hacking-with-rust/building-automated-twitter-schedule-bot).

> This is part of the [Rust telescope's Hacking With Rust](https://rust-telescope.live) video cast.

## Features

- Allow user schedule tweet to be posted at a later time.
- Scheduled tweets are bounded with a timeframe for which they should be posted.
- Once the scheduled time is attained, the tweet gets posted.

## Todo

- [x] Add tweet with a schedule
- [ ] Modify tweet scheduled but yet to be posted
- [ ] Get all tweets (by default, get but scheduled & unsheduled)
  - [ ] If category (scheduled or unscheduled) is provided, return that instead.

## Requirement

- Database (`Postgres`)
- Rust

## Contributing

If you find any issue, bug or missing feature, please kindly create an issue or submit a pull request, as it will go a long way in helping other [Rustaceans](https://www.rust-lang.org/community) grow.

## License

This repository is open-sourced software licensed under the [MIT](.LICENSE) license.
