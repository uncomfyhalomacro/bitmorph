# `BitMorph`

> Taking back decentralised Git hosting

A forge or VCS hosting for Git in [Internet Computer](https://internetcomputer.org).

# Project Rationale

This project is like GitHub. However, the difference is it
can be
- **Self-hosted**
- **Community hosted**
- **Decentralised**

You can even use it commercially as the code will be
licensed as Apache 2.0 and MIT.

# Architectural Documentation

```mermaid
---
title: "BitMorph"
---
flowchart BT
ic(((Internet Computer Blockchain)))
web2((Web 2.0 Interface))
wallet{{ic compatible wallet}}
db[(Database Canisters)]
db1[(Database Canister for User 1)]
db2[(Database Canister for User 2)]
db3[(Database Canister for User 3)]
u[users]
g{{Git Repositories}}
m[\BitMorph Frontend/]
db -.- ic
m -.- ic
subgraph Backend Canister
direction BT
db --contains--> g
db --contains--> userdata{{User Data}}
end
db -. contains a map to .-> db1
db -. contains a map to .-> db2
db -. contains a map to .-> db3
subgraph Frontend Canister
u -- login web3 route --> wallet
wallet <-.-> m
web2 <-.-> m 
u -- login web 2.0 route --> web2
wallet <-. can be linked login .-> web2
end
```

## Web 2.0 Integration

### Social Logins

### ActivityPub Integration

## Why IC?
I want to use IC as the platform to host this project
because canisters can scale really well at a reasonable
cost.

---

# Technical Specifications

## Frontend

## Backend

---

# Why the name?

**Morph** sounds like a good name for
- **moving over to decentralisation**
- **git repositories are almost-ever-changing**
- **it sounds cool and I have a life science background**
- **prefixed with *Bit* so it sounds techy**

