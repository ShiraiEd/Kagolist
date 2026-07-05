# Kagolist
*A shared shopping interactive checklist manager*
> Status: Currently on design

### Problem/Motivation
The person who actually requested this was my girlfriend. \
She usually forgets what she has to buy, and doesn't know if anyone has bought it already, \
so sometimes the same thing gets bought twice by mistake.

### What this will do?
A household can have multiple shopping lists (e.g. "Pharmacy", "Groceries"). \
Each list has its own items, and a user can add, remove or check off items within a list. \
All lists are shared between members of the same household, so everyone can monitor in realtime \
whether something still needs to be bought, or if someone is already taking care of it.

There will also be a purchase history: every time an item is bought, the price paid is \
recorded manually against that product, so you can query how much was spent over time.
> For v1 I won't be doing scraping, \
> nor realtime price change alerts.

### Stack
- Rust with the [Rapina](https://userapina.com/) framework
- Db and frontend still to be defined in `DESIGN.md`

### DESIGN
Still to be done inside the `DESIGN.md` file.

### Roadmap
1. Login + household endpoints
2. `ShoppingList` + `Product` + `ShoppingListItem`
3. WebSocket to broadcast events on the lists to household members
4. Purchase history (manual price entry per product)

### License
See [`LICENSE`](./LICENSE).
