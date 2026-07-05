# Kagolist - Design Document

> v1.0

---

## Overview

### Functional Requirements
- On start, the user should already have a `Personal Household` automatically created, with only himself as a member, and an empty `Personal List` created inside it.
- The owner of a household must be able to add or remove a member from it (there is no self-service "join" flow, membership is granted by the owner).
- A member can insert, remove or update an item in a `Shopping List`.
- When an item is added, removed, updated, claimed, released or marked as bought, the other members of the household receive the event via WebSocket in realtime.
- A user can query the purchase history for a product's price.
- A user should be able to `claim` an item for purchase, to alert other members of the same household not to buy it, and should also be able to manually release that claim, returning the item to `pending`.

### Non-Functional Requirements
- Users outside a household cannot access its data.
- WebSocket updates must arrive in <= 1s.
- There shouldn't be a big concern for scalability and availability for now.
- The list should still be visible even if the WebSocket connection drops.

*(Client-side local caching and how it syncs with WebSocket updates is an implementation/architecture decision, not a non-functional requirement - see the Architecture section below.)*

---

## Architecture

*(to be written: local cache sync strategy, atomic claim update, etc.)*
