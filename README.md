# Rails JS Template (Under Development)

The first thing is to recreate master key and credentials. 

```
EDITOR="code ." bin/rails credentials:edit
```

**Close the file.** Then run migrations:

```
rails db:migrate
```

Everything should be fine after that. Try to start `rails s` and see if it starts or not. 

---

## Improvements Required
- There are quite a lot of tests required for this repository. 
- Handling ajax to send request to backend; or just do sign and send transactions from the backend if user not scared of hacked database (which most probably lead to small sum stolen per user, given the limit of the private key stored). 
- Waiting for `near_api` gem to improve... 