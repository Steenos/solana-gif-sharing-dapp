use anchor_lang::prelude::*;

declare_id!("3mpuJnKhhoF9UHArfumAgiBE34X8rpLPtSF2Gpqh4gyH");

#[program]
pub mod mysolproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {

        //get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        //initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    //the function now accepts a gif_link param from the user.
    //we also reference the user from the context.
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
        //get reference to the account and increment total_gifs
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        //build the struct.
        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        //Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}
//attach certain variables to the startstuffoff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init,payer = user, space=10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,

}

#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

//create a custom struct for us to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

//tell solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    //Attach a vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
