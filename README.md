# Spring is NEAR Challenge 6: Zoo NFT by NEAR Ukraine
# Instructions


## Frontend
First, go to "movie ticket" or "future entrance ticket" and mint an NFT. After minting, you'll be redirect to the page. 
On localhost, it reloads; but on Heroku, it doesn't seems so. If you didn't see the mint number changes, **try reloading the page**. 

Then, you won't be redirect to the page (even with a promise, this fails). You'll **see your NFT in your inventory**. On the NavBar, click on your wallet address (after logging in) which redirects you to your inventory. On your Inventory tab, you can see your NFT. Click on it to redirect to your NFT page. 
From there, if it's a Non-Ownership F-NFT (see below for explanation), you can change the names in the form field to whom you share with, and click the "Share with" button. 
If it's a single ticket (for your own use only), this fields will be hidden. 

Then, from your **inventory**, you can click on the Listing tab. Supposingly this is for users to transfer, sell their bought NFTs. However, one didn't implement that functionality int he frontend yet. Here, we only make a view that the owner can see for you to play with (by not restricting it to only the owner can see what it is). 
You can create an NFT template, and specify how many mints this NFT have to mint before the template is exhausted. 

**Note that the contract restrict the template_id to be unique**, so if it's already exist the template_id, either use another one, or if you'd enter because you thought you haven't create the template, check that the template exist. 
Unfortunately, we haven't create a view function to search for template yet, due to time restrictions. That could be done in a few minutes in the future: just retrieving it from LookupMap. 

One minted 2 templates only, one for movie tickets (template_id: `movie_tickets`) and one for future entrance tickets (`entrance_ticket(s?)`) (unsure whether have 's' or not). You can use any others to try create a template. Ultimately this is only for zoo owners to navigate and "hardcoded" to their website for minting, not intended for end users to enter their template. 
WE don't restrict, however, which account can create template; anyone could use it to create template, make a website of their own, and use the backend, if they want. 

That's it for the frontend. WE'll talk about techinical instructions in the next section, before coming to contract. So scroll down! 

## Contract (see below, search with CTRL+F "Contract")

---
## Technical Instructions

The first thing is to bundle install stuff. We only want non production. 

```
bundle config set --local without production
bundle install
```

The second thing is to recreate master key and credentials. 

```
EDITOR="code ." bin/rails credentials:edit
```

**Close the file.** Then run migrations:

```
rails db:migrate
```

Then we need to install bootstrap. (Ignore the error, it'll auto install upon cannot find bootstrap). 
This requires yarn and node js. 

```
bash rebuild.sh
```

Everything should be fine after that. Try to start `rails s` and see if it starts or not. 

```
rails s
```

---

## Problem Statement: Challenge #6. Zoo NFT by NEAR Ukraine
The war in Ukraine has affected each Ukrainian and a lot of people around the globe. At the moment the war is being actively waged and is in the hot phase, but when Ukraine wins, a new stage will begin - the stage of restoration of our country.  

Ukrainian zoos are in severe need of help now and will be in need after the war ends. There are no visitors in the zoos, which results in no budgeting.

You need to create an NFT project which will aid in attracting funds that will be allocated to buy food for animals or help zoos restore their work after the war is over.

This challenge is a part of the “For Ukraine” hackathon by 🇺🇦near-ua . Your submission will be eligible for both NEAR Spring and “For Ukraine” bounties. More info here: https://eventornado.com/event/near-hackathon-for-ukraine/

---


## Contract
Mostly copied from [this link](https://github.com/near-examples/nft-tutorial), but added extra functionality like `pay_and_mint` (and the unsafe but save gas version) in `sale.rs` of `market-contract`, and `set_accounts` in the `nft_core.rs` of `nft-contract` to set the **non-ownership Fractionalized NFT (NO-F-NFT)**. A "Non-ownership" means the owners sharing the NFTs could see it in their wallet, but they can't transfer their ownership to others. All ownership still retains with the owner, and they only share the 
ownership so they could see it from their wallet. 

This is useful as we're selling tickets. Nowadays, a single "ticket for family of 4" either have to enter the Zoo in a group of 4, or we need 4 pieces of tickets. 
For us, if 3 people go into the zoo, but one come late (because travelling not together perhaps), the latecomer could still enter the zoo as it shares the ticket, so don't need someone to come out and pick him up with the ticket, and don't need a record on the 
gatekeeper's site saying this still have 1 person haven't entered yet. 

To save storage space, we don't transfer and pay the NFT, we pay and mint. When the ticket is not needed at all, it doesn't need to lie somewhere; just mint as you go. Minter will pay for storage space when they request for ticket. 
The "traditional" way of having the owner mint all ticket first then `transfer_and_pay` just doesn't sounds right. Owner have to first pay for the storage, then after transfer, the storage is transferred. 
What says the tickets that are never used? THey're lying around in the blockchain like garbage. 

Hence, pay and mint will mint as needed. 

### Not yet Implemented Functionality
Because it's not gonna use now, we haven't implement two things: 
- the invalidation of ticket after it's being used, and
- checking for valid tickets. 
- NFT hide, show, and (perhaps) delete. 
- NFT Transfer

We do have a Vector of length "size" mentioning whether they're used, which when minted, they're false in the first place. Then when it's used, it'll be set to true, and the ticket is "invalidated" for that sector. Supposingly there will be comparing how many have been used, but this is another (quite complicated) logic that needs some testing and 
experimentation, which we don't have time during this hackathon to deal with. 

And checking for valid tickets? Well, this isn't too difficult. When you `pay_and_mint`, supposingly (which we haven't implemented) we save the `token_id` a copy on the marketplace contract. Hence, if anyone bypass and mint their ticket on `nft-contract` 
but zoo owner can't find it in the marketplace contract, the ticket is not a valid ticket. 

If the ticket is one use only, it's annoying (from one's perspective) to have it lying around in the inventory; hence, hiding it is a viable option. THe NFT is still stored on blockchain, but we "remove" it from owner's view. Whenever they want to view again, they could make a function call to the contract to "show" it back. 
This uses the `internal_remove_....` and `internal_add...` functions in `nft-contract internal.rs`. 

Finally, (single-use/multiple-use) used NFTs don't have to lie around. ONe heard about some "proper wayt o burn NFT is transfer it to a rubbish account". One isn't sure that's right. After all, why not just delete it away? If you need its history, you can search the blockchain's 
explorer for whether it exists before or not; the token itself does not have to exist for eternity, just free up the space by deleting it! 

## References
- https://github.com/near-examples/nft-tutorial
