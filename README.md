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
* [ ] Local account creation, deletion and modification (@connor.bryan)
* [ ] Local account login (@connor.bryan)
* [ ] Access control (@connor.bryan)
* [ ] Admin role (for those who manage Zulu) (@connor.bryan)
* [ ] LDAP account syncing to local accounts (@jacob.eva)
* [ ] LDAP account login (@jacob.eva)
* [ ] JWT auth on successful login (@jacob.eva & @connor.bryan)

#### Frontend
* [ ] Login functionality via frontend (@hugo.lelubre & @connor.bryan)
### v0.2
#### User roles
* [ ] Lecturer role (for those who make challenges) (@jacob.eva)
* [ ] Integration of roles with LDAP groups (@jacob.eva)
* [ ] Store role in JWT (@connor.bryan)

#### Flags
* [ ] Flag creation, deletion and modification (@jacob.eva)
* [ ] Flag string submission and deletion for users (@jacob.eva)
* [ ] Access control (@jacob.eva)

#### Leaderboard
* [ ] Generate leaderboard (most points) for top 30 users (@connor.bryan)

#### Frontend
* [ ] Ability to manage own flag strings as a user (@hugo.lelubre & @connor.bryan)
* [ ] Ability to manage own flags as a lecturer (@hugo.lelubre & @connor.bryan)
* [ ] Ability to view leaderboard (@hugo.lelubre & @connor.bryan)

## Running Zulu
Simply `git clone` this repo and run `cargo run` in your terminal :)

## Useful Resources.
[Rocket](https://rocket.rs/)
