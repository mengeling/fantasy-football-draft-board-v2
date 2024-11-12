use crate::models::drafted_player::DraftedPlayer;
use sqlx::PgPool;

pub async fn draft_player(
    pool: &PgPool,
    user_id: i32,
    player_id: i32,
) -> Result<DraftedPlayer, sqlx::Error> {
    sqlx::query_as!(
        DraftedPlayer,
        r#"
        INSERT INTO drafted_players (user_id, player_id)
        VALUES ($1, $2)
        RETURNING id, user_id, player_id, drafted_at
        "#,
        user_id,
        player_id
    )
    .fetch_one(pool)
    .await
}

pub async fn undraft_player(
    pool: &PgPool,
    user_id: i32,
    player_id: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM drafted_players
        WHERE user_id = $1 AND player_id = $2
        "#,
        user_id,
        player_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_drafted_players(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<DraftedPlayer>, sqlx::Error> {
    sqlx::query_as!(
        DraftedPlayer,
        r#"
        SELECT id, user_id, player_id, drafted_at
        FROM drafted_players
        WHERE user_id = $1
        ORDER BY drafted_at
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}
