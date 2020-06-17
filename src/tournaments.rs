use crate::database::{
    matches as matches_db, tournaments as tournaments_db, users as users_db, Tournament,
};
use crate::{AdminKey, Database};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::{get, post, State};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[get("/list")]
pub fn list(db: Database) -> JsonValue {
    // TODO: Mark tournaments as complete or not
    let tournaments = tournaments_db::table.load::<Tournament>(&*db).unwrap();
    let tournament_list = tournaments
        .into_iter()
        .map(|tournament| {
            json!({
                "name": tournament.name,
                "rounds": tournament.rounds,
                "duel_points_per_win": tournament.duel_points_per_win,
                "duel_points_per_loss": tournament.duel_points_per_loss,
                "duel_points_jackpot": tournament.duel_points_jackpot,
            })
        })
        .collect::<Vec<JsonValue>>();
    json!({
        "r.type": "response",
        "tournament_list": tournament_list,
    })
}

#[derive(Deserialize)]
pub struct StartRequest {
    name: String,
    duel_points_per_win: i32,
    duel_points_per_loss: i32,
    duel_points_jackpot: i32,
    admin_key: String,
}

#[post("/start", data = "<request>")]
pub fn start(
    db: Database,
    state_admin_key: State<AdminKey>,
    request: Json<StartRequest>,
) -> JsonValue {
    if request.admin_key == state_admin_key.0 {
        let tournament_already_exists = !tournaments_db::table
            .filter(tournaments_db::name.eq(&request.name))
            .load::<Tournament>(&*db)
            .unwrap()
            .is_empty();
        if !tournament_already_exists {
            let mut users = users_db::table
                .select(users_db::id)
                .load::<i32>(&*db)
                .unwrap();
            if users.len() > 1 {
                let rounds = (users.len() as i32 - 1).min(8);
                let tournament = diesel::insert_into(tournaments_db::table)
                    .values((
                        tournaments_db::name.eq(&request.name),
                        tournaments_db::rounds.eq(rounds),
                        tournaments_db::duel_points_per_win.eq(request.duel_points_per_win),
                        tournaments_db::duel_points_per_loss.eq(request.duel_points_per_loss),
                        tournaments_db::duel_points_jackpot.eq(request.duel_points_jackpot),
                    ))
                    .get_result::<Tournament>(&*db)
                    .unwrap();
                users.shuffle(&mut thread_rng());
                for round in 1..=rounds {
                    for pair in users.chunks(2) {
                        diesel::insert_into(matches_db::table)
                            .values((
                                matches_db::tournament.eq(tournament.id),
                                matches_db::round.eq(round),
                                matches_db::duelist1.eq(pair.get(0)),
                                matches_db::duelist2.eq(pair.get(1)),
                                matches_db::duelist1_reported_winning.eq(Option::<bool>::None),
                                matches_db::duelist2_reported_winning.eq(Option::<bool>::None),
                            ))
                            .execute(&*db)
                            .unwrap();
                    }
                    users[1..].rotate_right(1);
                }
                json!({
                    "r.type": "response",
                })
            } else {
                json!({
                    "r.type": "error",
                    "info": "Not enough duelists to form a tournament.",
                })
            }
        } else {
            json!({
                "r.type": "error",
                "info": "Tournament already exists.",
            })
        }
    } else {
        json!({
            "r.type": "error",
            "info": "Invalid admin key.",
        })
    }
}
