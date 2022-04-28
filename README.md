# Spring is NEAR Challenge 6: Zoo NFT by NEAR Ukraine

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
Mostly copied from [this link](https://github.com/near-examples/nft-tutorial), but add some missing functionality like `add_sale`, and `nft_transfer_payout`. 

## References
- https://github.com/near-examples/nft-tutorial
