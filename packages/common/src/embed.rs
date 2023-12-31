use crate::determine_avatar;
use poise::serenity_prelude::{Member, User};
use serenity::{builder::CreateEmbed, utils::Colour};
use std::borrow::Cow;

#[allow(non_upper_case_globals)]
pub mod colours {
    use serenity::utils::Colour;

    pub const Default: Colour = Colour::from_rgb(192, 238, 255);
    pub const Success: Colour = Colour::from_rgb(95, 252, 198);
    pub const Failure: Colour = Colour::from_rgb(255, 171, 171);
}

pub fn generate_embed(user: &User, member: Option<Cow<'_, Member>>, colour: Colour) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    let name = match &member {
        Some(member) => member.display_name().to_string(),
        None => user.name.clone(),
    };

    let avatar = determine_avatar(user, member);

    embed
        .author(|author| author.name(name).icon_url(avatar))
        .color(colour)
        .clone()
}
