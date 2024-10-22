use crate::database::pool::DB_POOL;
use crate::models::player::Player;
use anyhow::Result;

pub async fn save_player(player: &Player) -> Result<()> {
    // Insert player
    sqlx::query(
        "INSERT INTO players (id, name, team, position, overall_ranking, position_ranking, bye_week, bio_url)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (id) DO UPDATE SET
         name = EXCLUDED.name,
         team = EXCLUDED.team,
         position = EXCLUDED.position,
         overall_ranking = EXCLUDED.overall_ranking,
         position_ranking = EXCLUDED.position_ranking,
         bye_week = EXCLUDED.bye_week,
         bio_url = EXCLUDED.bio_url"
    )
    .bind(player.id)
    .bind(&player.name)
    .bind(&player.team)
    .bind(&player.position)
    .bind(player.ranking.overall)
    .bind(player.ranking.position)
    .bind(player.bye_week)
    .bind(&player.bio_url)
    .execute(&*DB_POOL)
    .await?;

    // Insert player bio
    sqlx::query(
        "INSERT INTO player_bios (player_id, image_url, height, weight, age, college)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (player_id) DO UPDATE SET
         image_url = EXCLUDED.image_url,
         height = EXCLUDED.height,
         weight = EXCLUDED.weight,
         age = EXCLUDED.age,
         college = EXCLUDED.college",
    )
    .bind(player.id)
    .bind(&player.bio.image_url)
    .bind(&player.bio.height)
    .bind(&player.bio.weight)
    .bind(player.bio.age)
    .bind(&player.bio.college)
    .execute(&*DB_POOL)
    .await?;

    // Insert player stats
    for (stat_name, stat_value) in &player.stats {
        sqlx::query(
            "INSERT INTO player_stats (player_id, stat_name, stat_value)
             VALUES ($1, $2, $3)
             ON CONFLICT (player_id, stat_name) DO UPDATE SET
             stat_value = EXCLUDED.stat_value",
        )
        .bind(player.id)
        .bind(stat_name)
        .bind(stat_value)
        .execute(&*DB_POOL)
        .await?;
    }

    Ok(())
}
