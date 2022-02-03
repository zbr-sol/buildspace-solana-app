# Simple Solana GIF Portal Project

### **Welcome 👋**
This project started out with the "Build a Web3 app on Solana with React and Rust" project template from [Buildspace](https://buildspace.so/).

I made a pretty simple front-end that integrates with the Solana blockchain. You can connect to the site with a Phantom wallet, and after that you are able to view the content of the site. I've been rewatching Scrubs recently, so right now all the GIFs are Scrubs themed. However, any user who connects their wallet can also upload any GIFs they choose.

You can see the current version of the application [here](https://scrubs-gif-solana-app.vercel.app/). Make sure your Phantom wallet is on devnet! No real SOL is being transferred.

After going through the setup process and the first couple lessons on Buildspace, I also extended the project to include the ability to upvote, as well as the ability to send a tip in SOL to whoever uploaded a particular GIF.


### **What else could I add?**
This is absolutely an MVP, and I know there's a few things I could add to improve the experience. Some that immediately come to mind:
1. Clean up the UX
2. Instead of just making `upvotes` a number, I could store an array of wallet IDs that have upvoted a particular GIF. This would do 2 things: 1) prevent a user from upvoting their own submission and 2) only let a user upvote a submission once.
3. Right now the tip is always 0.01 SOL. I could update the `tip_small_sol` function to allow any amount of tip, and the user could enter this amount on the front-end.
4. Add even more testing on the rust/Anchor side of things.
