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
* [x] Admin role (for those who manage Zulu) (@connor.bryan)
* [x] LDAP account syncing to local accounts (@jacob.eva)
* [x] LDAP account login (@jacob.eva)
* [x] JWT auth on successful login (@jacob.eva & @connor.bryan)

#### API
* [x] Fix CORS header (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS/Errors/CORSMissingAllowOrigin) (@connor.bryan)

#### Frontend
* [x] Login & registration functionality via frontend (@connor.bryan)
### v0.2
#### User roles
* [x] Lecturer role (for those who make challenges) (@jacob.eva)
* [x] Integration of roles with LDAP groups (@jacob.eva)
* [x] Store role in JWT (@connor.bryan)

#### Challenges
* [x] Challenge creation, deletion and modification (@jacob.eva)
* [x] Flag string submission for users (@jacob.eva)
* [x] Challenge file downloads (@jacob.eva)
* [x] Hashed flags for challenges in DB (@jacob.eva)

#### Leaderboard
* [x] Generate leaderboard (most points) for top 30 users (@connor.bryan)

#### Frontend
* [x] Ability to submit flags as a user (@connor.bryan)
* [ ] Ability to manage own flags as a lecturer (@connor.bryan)
* [x] Ability to view leaderboard (@connor.bryan)
* [x] Search bar for entities

## Long term goals
* [ ] Federation between different Zulu instances a la [ActivityPub](https://activitypub.rocks/)
* [ ] Community made challenges
* [ ] 4 letter identifiers for instances of Zulu
* [ ] Challenge tags
* [ ] Challenge descriptions
* [ ] Challenge images
* [ ] Public and private challenges
* [ ] Implement caching in drift to cache the current leaderboard status to reduce
expensive database calls, have it refreshed on correct flag submission.
If same frontend requests leaderboard and it hasnt changed use the http status
code 304 unmodified.

## Whitepaper (Running Zulu)
Read our whitepaper [here](files/whitepaper.pdf).

## Useful Resources.
[Rocket](https://rocket.rs/)
