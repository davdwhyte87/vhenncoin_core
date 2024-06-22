use bigdecimal::ToPrimitive;
use itertools::Itertools;

use serde::{Deserialize, Serialize};

use crate::models::{balance_pack::{BalanceCPack, BalancePack, WalletCPack}, wallet::WalletC};

use super::wallet::Wallet;

pub struct Concensus{

}

#[derive(Debug, Clone)]
pub struct Vote {
       pub balance:f32,
       pub vote:i32,
       pub http_address:String
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct VoteWallet {
          pub wallet:WalletC,
          pub vote:i32,
          pub ip_address:String
      }
impl Concensus{

    pub  fn vote_wallet(balance_pack_list:Vec<WalletCPack >)->VoteWallet{
        //
        let total_count = balance_pack_list.len().to_i32();
        let total_count = match  total_count {
            Some(data)=>{data},
            None=>{0}
        };
    
       
    
        let mut  votes:Vec<VoteWallet> = vec![];
        
        for balance_pack in balance_pack_list{
           
            Self::add_vote_wallet(balance_pack.wallet,  &mut votes, balance_pack.ip_address)
        }


        print!("{:#?}", votes);

        // select highest vote
        let mut highest_vote:VoteWallet = VoteWallet { wallet: WalletC::default(), vote: 0, ip_address: "".to_string() };

        for vote in votes{
            if vote.vote > highest_vote.vote {
                highest_vote = vote;
            }
        }

        print!("highest vote {:#?}", highest_vote);

        return highest_vote
    
    }

    pub  fn vote_balance_c(balance_pack_list:Vec<BalanceCPack >)->Vote{
        //
        let total_count = balance_pack_list.len().to_i32();
        let total_count = match  total_count {
            Some(data)=>{data},
            None=>{0}
        };
    
       
    
        let mut  votes:Vec<Vote> = vec![];
        
        for balance_pack in balance_pack_list{
           
            Self::add_vote(balance_pack.balance,  &mut votes, balance_pack.ip_address)
        }


        print!("{:#?}", votes);

        // select highest vote
        let mut highest_vote:Vote = Vote{balance:0.0, vote:0,http_address:"".to_string()};

        for vote in votes{
            if vote.vote > highest_vote.vote {
                highest_vote = vote;
            }
        }

        print!("highest vote {:#?}", highest_vote);

        return highest_vote
    
    }
    pub  fn vote_balance(balance_pack_list:Vec<BalancePack >)->Vote{
        //
        let total_count = balance_pack_list.len().to_i32();
        let total_count = match  total_count {
            Some(data)=>{data},
            None=>{0}
        };
    
       
    
        let mut  votes:Vec<Vote> = vec![];
        
        for balance_pack in balance_pack_list{
           
            Self::add_vote(balance_pack.balance,  &mut votes, balance_pack.server_http_address)
        }


        print!("{:#?}", votes);

        // select highest vote
        let mut highest_vote:Vote = Vote{balance:0.0, vote:0,http_address:"".to_string()};

        for vote in votes{
            if vote.vote > highest_vote.vote {
                highest_vote = vote;
            }
        }

        print!("highest vote {:#?}", highest_vote);

        return highest_vote
    
    }
    
  
    fn contains(data:f32, votes:&Vec<Vote>)->bool{
        for vote in votes{
            if vote.balance == data{
                return true
            }
        }
    
        return false
    }

    fn contains_wallet(data:WalletC, votes:&Vec<VoteWallet>)->bool{
        for vote in votes{
            if vote.wallet == data{
                return true
            }
        }
    
        return false
    }

    fn add_vote_wallet(data:WalletC, votes: &mut Vec<VoteWallet>, address:String){
        // go through votes, if the item has been voted for before, 
        // go through votes and update vote count 
        // if not voted for before, add
        if Self::contains_wallet(data.to_owned(), &votes) {
            for  (i, vote) in votes.iter_mut().enumerate(){
                if (vote.wallet == data){
                   vote.vote = vote.vote +1; 
                }   
            }       
        }else {
            let new_vote = VoteWallet{wallet:data, vote:1, ip_address:address};
            votes.push(new_vote);
        }

      
    }
    fn add_vote(data:f32, votes: &mut Vec<Vote>, address:String){
        // go through votes, if the item has been voted for before, 
        // go through votes and update vote count 
        // if not voted for before, add
        if Self::contains(data, &votes) {
            for  (i, vote) in votes.iter_mut().enumerate(){
                if (vote.balance == data){
                   vote.vote = vote.vote +1; 
                }   
            }       
        }else {
            let new_vote = Vote{balance:data, vote:1, http_address:address};
            votes.push(new_vote);
        }

      
    }

    

  
}