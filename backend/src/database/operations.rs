use crate::database::pool::DB_POOL;
use crate::models::player::Player;
use anyhow::Result;

pub async fn save_player(player: &Player) -> Result<()> {
    sqlx::query(
        "INSERT INTO players (id, name, position, team, bye_week, height, weight, age, college)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         ON CONFLICT (id) DO UPDATE SET
         name = EXCLUDED.name,
         position = EXCLUDED.position,
         team = EXCLUDED.team,
         bye_week = EXCLUDED.bye_week,
         height = EXCLUDED.height,
         weight = EXCLUDED.weight,
         age = EXCLUDED.age,
         college = EXCLUDED.college",
    )
    .bind(player.id)
    .bind(&player.name)
    .bind(&player.position)
    .bind(&player.team)
    .bind(player.bye_week)
    .bind(&player.height)
    .bind(&player.weight)
    .bind(player.age)
    .bind(&player.college)
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
