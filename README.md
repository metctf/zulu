# Zulu
## What is Zulu?
Zulu is a CTF server currently in development by the Cardiff Met CTF society
for use with our challenges produced by members of our society. It will allow
us to keep track of the number of flags and points associated with them for
each member of our society, which will be displayed on a society-wide
leaderboard. Zulu will be designed to be university agnostic and highly
configurable, allowing other university societies to use our software for their
own uses if they so wish, with their own branding and such. Zulu is licensed
under the GPLv3 and is free software.

## Roadmap
### v0.1
#### Users
* [x] Local account creation, deletion and modification (@connor.bryan)
* [x] Local account login (@connor.bryan)
* [x] Access control (@connor.bryan)
* [x] Admin role (for those who manage Zulu) (@connor.bryan)
* [ ] LDAP account syncing to local accounts (@jacob.eva)
* [ ] LDAP account login (@jacob.eva)
* [x] JWT auth on successful login (@jacob.eva & @connor.bryan)

#### API
* [ ] Mutual TLS (@jacob.eva)
* [x] Fix CORS header (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS/Errors/CORSMissingAllowOrigin) (@connor.bryan)

#### Frontend
* [x] Login & registration functionality via frontend (@connor.bryan)
* [ ] Mutual TLS support (@connor.bryan)
### v0.2
#### User roles
* [ ] Lecturer role (for those who make challenges) (@jacob.eva)
* [ ] Integration of roles with LDAP groups (@jacob.eva)
* [x] Store role in JWT (@connor.bryan)

#### Challenges
* [ ] Challenge creation, deletion and modification (@jacob.eva)
* [ ] Challenge string submission for users (@jacob.eva)
* [ ] Challenge file downloads (@jacob.eva)
* [ ] Access control (@jacob.eva)

#### Leaderboard
* [x] Generate leaderboard (most points) for top 30 users (@connor.bryan)
* [ ] Implement caching in drift to cache the current leaderboard status to reduce
expensive database calls, have it refreshed on correct flag submission.
If same frontend requests leaderboard and it hasnt changed use the http status
code 304 unmodified.

#### Frontend
* [ ] Ability to submit flags as a user (@connor.bryan)
* [ ] Ability to manage own flags as a lecturer (@connor.bryan)
* [x] Ability to view leaderboard (@connor.bryan)
* [ ] Search bar for entities

## Running Zulu
Simply `git clone` this repo and run `cargo run` in your terminal :)

To run the frontend use these commands:

```
cargo install trunk

rustup target add wasm32-unknown-unknown

trunk serve --open
```

## Useful Resources.
[Rocket](https://rocket.rs/)
