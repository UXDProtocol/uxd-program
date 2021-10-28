# MANGO API IN BRIEF

## shit we care about

- init account (good for a whole group aka a set of markets that can be xmargined)
- deposit (coin into account)
- withdraw (coin from account)
- place perp order (self explanatory. order type comes from serum i think)
- cancel perp order (their id and our id versions exist)
- settle pnl (takes two accounts and trues up)
  settle is necessary but kinda weird in that like, you need to find a loser to match your winner

## shit we might

- add to basket ("add a spot market to account basket" never made clear wtf this is)
- borrow (unclear if we need to borrow to short? prolly not...)
- place spot order (this is just a serum passthrough)
- cancel spot order (as above)
- settle funds ("settle funds from serum dex open orders" maybe just serum passthrough?)
- settle borrow (only if we use borrow
  the point of serum calls is they can use the money in mango accounts
  but... i dont think we need to mess w spot

### flow

user deposits btc, we send to mango
open a equiv sized short position sans whatever amount for liquidation protection
once the position is open it theoretically has a fix dollar value
(sans execution risk, sans funding, sans liquidation buffer)
this is the amount of uxd we mint and return to the user
then redemption of uxd for the underlying means we... burn uxd
close out an equivalent amount of position in the coin they want
settle pnl, withdraw coin, deliver to depository, give user redeemables
important that all trasaction costs and price differences _must_ be passed onto the user
otherwise we open ourselves up to all kind of insane arbitrage attacks
since uxd _must_ be fungible we cannot maintain accounts for individuals

oook so... mint has to go like. for a particular depository...
we accept redeemable, proxy transfer coin to us, move coin onto mango (deposit)
create an opposite position on mango (place perp order). and then give uxd to user
for now we take fro granted that all deposited coins have a corresponding perp
if we want to take more esoteric forms of capital we may need to swap on serum

im not sure controller should create uxd... idk what if we redeploy to a new address?
we should have liek... a function new, to set up the controller with state and owner
and a function register depository to whitelist a depository address
and create the mango account and such
