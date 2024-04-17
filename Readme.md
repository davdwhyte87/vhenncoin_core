# Vhenn_coin
 digital payment network for the Internet

It is difficult and expensive to get payments 
done over the internet. Imagine you wanted to pay 
a blogger $0.1 for reading a post. We believe that
there is a need for an internet currency. 

We have developed this payment network using rust
and we hope the internet and creatives will adopt
it in their software applications.


# The protocol
kuracoin is built on the tcp protocol primarily, but has an http layer. Miners can configure a node to choose which type of network base protocol they want. 
Our main protocol for communication is text based. Servers exchange encrypted text. 
Message format: 
- Action name (GetWalletBalance etc..)
- Data ( {"address":"xxxxxxxx", "wallet_type":"0"}  for example or "UIOJNDJNKABA 988 u#*udCAHOUI Y (&*Y@YFIHudhjkdssdkjsb yt8uygbshbdf" for encrypted data)
- Message Signature
- Sender Node public key 





