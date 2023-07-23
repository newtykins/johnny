use johnny::preludes::command::*;

/// Toggle autorole on or off
#[command(
    slash_command,
    category = "moderation",
    guild_only,
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn toggle(ctx: Context<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.guild().unwrap();
    let model = guild
        .update_db(&ctx.data().db, |model| {
            model.autorole = Set(!model.autorole.take().unwrap());
            model
        })
        .await?
        .unwrap();

    // create the embed
    let mut base_embed = generate_embed(
        ctx.author(),
        ctx.author_member().await,
        Some(if model.autorole {
            colours::SUCCESS
        } else {
            colours::FAILURE
        }),
    );

    set_guild_thumbnail(&mut base_embed, guild);

    base_embed
        .title(if model.autorole {
            "Enabled!"
        } else {
            "Disabled!"
        })
        .description(format!(
            "Autorole has been {}!",
            if model.autorole {
                "enabled"
            } else {
                "disabled"
            }
        ));

    // announce the toggle
    ctx.send(|msg| msg.embed(|embed| message_embed!(embed, base_embed)))
        .await?;

    Ok(())
}
