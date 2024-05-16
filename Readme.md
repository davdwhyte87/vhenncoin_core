
# Kuracoin
 digital payment network for the Internet. 



It is difficult and expensive to get payments 
done over the internet. Imagine you wanted to pay 
a blogger $0.1 for reading a post. We believe that
there is a need for an internet currency. 

Kuracoin is a decentralized payment network for the new 
financial system built on love. 



# The protocol
kuracoin is built on the tcp protocol primarily, but has an http layer. Miners can configure a node to choose which type of network base protocol they want. 
Our main protocol for communication is text based. Servers exchange encrypted text. 
Message format: 

- 0 = **Action name**(GetWalletBalance etc..)
- 1 =  **Data** ( {"address":"xxxxxxxx", "wallet_type":"0"}  for example or "UIOJNDJNKABA 988 u#*udCAHOUI Y (&*Y@YFIHudhjkdssdkjsb yt8uygbshbdf" for encrypted data)
- 2= **Message Signature**
- 3 =**Sender Node public key** 
- 4 = is_broadcasted? This is either 0 or 1 string. It lets the server know if this is a client request or broadcast. 

**Message format for response**
- 0 = Response code
- 1 = Message
- 2 = Data


**Response codes**
- 0 = Error
- 1 = Success
- 2 = Background exchange message

# Testing 
You can test the code in development by running:
- Windows: deploy.bat 
You will need a folder called test_servers. This folder contains 
nodes which have their own folder "server1, server2, ...". 
- Linux:
XXXXXXXXXX

Once you have raised a PR and it is approved, it will be moreged into development. 
The development environment is wired to a test network. 
If all test passes on development branch, we package a bunch of new features, bug fixes, and launch a release. 
Releases are not done frequently because of the nature of the technology and it's application in finance. 



