use serenity::prelude::*;
use serenity::model::prelude::*;
pub struct TextCommands; 

impl TextCommands {

    pub async fn message( ctx: Context, msg: Message) {
        if msg.content == "!ping"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        
        if msg.content == "!dil"
        { 
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("<@{}> has a massive SCHLONG ", 235454585068716032u64)).await
            { 
                println!("Error sending message: {:?}", why);
            }
        }
        
        if msg.content == "!deedee"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("<@{}> will smack you if you step out of line ", 1070730136112078938u64)).await
            { 
                println!("Error sending message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, "https://tenor.com/view/dwight-the-office-theoffice-gif-27712963").await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        
        if msg.content == "!puno"
        { 
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("<@{}> has the best taste in music of all the officers!", 151694881281015809u64)).await
            { 
                println!("Error sending message: {:?}", why);
            }
        }
        
        if msg.content == "!sobs"
        { 
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("<@{}> has the thickest thighs in the guild",191920897831993344u64)).await
            { 
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
}