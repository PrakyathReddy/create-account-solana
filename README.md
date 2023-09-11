# create-account-solana

In this program, i am going to create an account on solana.

This account will be a system account, meaning it will be owned by the System Program. This means only the system program will be allowed to modify it's data.

Here, I will be using 2 methods to create the accounts - using CPI - and by calling the system program directly.

CPI means that I first send the transaction to create an account first to our deployed solana program, which then calls the System Program directly.

Calling the system program directly means that the client sends the transaction to create the program directly to the solana program.

In this program, the account will simply hold some sol.
