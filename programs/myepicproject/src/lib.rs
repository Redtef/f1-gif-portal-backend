use anchor_lang::prelude::*;

declare_id!("BgMok6icXoS1kUs6qgpKKBPHouRcZsDVRFCoTLCcUSCd");

#[program]
pub mod myepicproject {
  use super::*;
  // initializes everything 
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // get a mutable reference to the account 
    // this helps us persist the changes to the basAccount 
    // instead of having a copy of it 
    let base_account = &mut ctx.accounts.base_account;
    //initialise total_gifs
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx:Context<AddGif>, gif_link:String)->ProgramResult{
      //get a mutable ref to the account 
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;

      // build the struct with tha data passed
      let item = ItemStruct{
          gif_link:gif_link.to_string(),
          user_address: *user.to_account_info().key,
          up_votes: 0,
        down_votes: 0,
      };
      base_account.gif_list.push(item);
      base_account.total_gifs += 1;

      Ok(())
  }

  pub fn up_vote(ctx:Context<UpVote>,index:u64)->ProgramResult{
    let i = index as usize;
    let base_account = &mut ctx.accounts.base_account;
    let mut item = &mut base_account.gif_list[i];
    item.up_votes += 1;
    Ok(())
  }
  pub fn down_vote(ctx:Context<DownVote>,index:u64)->ProgramResult{
    let i = index as usize;
    let base_account = &mut ctx.accounts.base_account;
    let mut item = &mut base_account.gif_list[i];
    item.down_votes += 1;
    Ok(())
  }

}

// we specify how to initialize the baseAccount
// and what to hold in the StartStuffOFf context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // init -> initialise a new baseACcount 
    // the payer fo the account is the user (rent)
    // allocate 9000 bytes of space for the account
    #[account(init,payer=user,space=8+1000)]
    pub base_account: Account<'info,BaseAccount>,
    // proves that the user actually owns their wallet account
    #[account(mut)]
    pub user: Signer<'info>,
    // references teh systemProgram
    // which is what runs solana 
    pub system_program: Program <'info,System>
}
// Specify what data you want in the AddGif context
// this context has acces to a mutable ref of base_account
#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpVote<'info>{
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DownVote<'info>{
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// custom struct to store data 
// this struct is an account , whichc is basically a file 
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct{
    pub gif_link: String,
    pub user_address: Pubkey,
    pub up_votes: u64,
    pub down_votes: u64,
}
// this is an account where we store data 
#[account]
pub struct BaseAccount{
    pub total_gifs: u64,
    // add a vector of type ItemStruct to the account 
    pub gif_list: Vec<ItemStruct>,
}

