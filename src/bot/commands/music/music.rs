use super::super::{Context, Error};
use crate::{get_config, messager};
use std::collections::HashMap;

/// Adds song to queue
#[poise::command(slash_command, prefix_command, aliases("m"), category="Music", guild_only)]
pub async fn music(
    ctx: Context<'_>,
    #[description = "Keyword for wanted video/playlist"] keyword: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    // TODO: use proper database
    let data = HashMap::from([
      ("gachi",      "https://www.youtube.com/watch?v=vysM33WCieE"),
      ("trump",      "https://www.youtube.com/watch?v=y5ki_VGlmiM"),
      ("pirate",     "https://www.youtube.com/watch?v=iKJlhhf_lhs"),
      ("chillexy",   "https://open.spotify.com/playlist/7sTDKMewsaANlRSGUQVzPU?si=TeOQQjUeTimMw6Jd9mVEFQ"),
      ("can_fav",    "https://www.youtube.com/playlist?list=PLqbnwol7YwR4SKldGdb1-43rZB6PljLe7"),
      ("svetlana",   "https://open.spotify.com/playlist/6gHJvb4rlKdUNW4Q9DjXRw?si=DWU8slGTQ9yeyr0uDmu0RA"),
      ("kpop1",      "https://www.youtube.com/watch?v=citgluw97m8"),
      ("kpop2",      "https://www.youtube.com/watch?v=18nDrsoii5M&list=RDCLAK5uy_mHW5bcduhjB-PkTePAe6EoRMj1xNT8gzY&start_radio=1"),
      ("pentakill3", "https://www.youtube.com/watch?v=VXtaMAN9zX4"),
      ("songul",     "https://www.youtube.com/watch?v=hIiAJ69o3Zw"),
      ("anime1",     "https://www.youtube.com/playlist?list=PLqbnwol7YwR7WGvjEDjjd9GeLx-KRmipa"),
      ("anime2",     "https://open.spotify.com/playlist/49ZreaOgQ0dMirgecTPh0n?si=8f88241e9a7b4a01"),
      ("anime3",     "https://www.youtube.com/watch?v=Mn7Bv8rGRzg"),
      ("anime4",     "https://www.youtube.com/watch?v=J0S6tc6dIK8"),
      ("yuki",       "https://www.youtube.com/watch?v=KO-G5DVNlw4"),
      ("ayaya",      "https://www.youtube.com/watch?v=9wnNW4HyDtg"),
    ]);

    // TODO: help for available keywords
    if let Some(url) = data.get(keyword.as_str()) {
        server.player.play(&ctx, url.to_string()).await;
    } else {
        messager::send_error(&ctx, "Invalid keyword", true).await;
    }

    Ok(())
}
