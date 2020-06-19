use crate::database::{matches, tournaments, users, Match, Tournament};
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
    let tournaments = tournaments::table.load::<Tournament>(&*db).unwrap();
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
        let tournament_already_exists = tournaments::table
            .filter(tournaments::name.eq(&request.name))
            .first::<Tournament>(&*db)
            .is_ok();
        if !tournament_already_exists {
            let mut users = users::table.select(users::id).load::<i32>(&*db).unwrap();
            if users.len() > 1 {
                let rounds = (users.len() as i32 - 1).min(6);
                let tournament = diesel::insert_into(tournaments::table)
                    .values((
                        tournaments::name.eq(&request.name),
                        tournaments::rounds.eq(rounds),
                        tournaments::duel_points_per_win.eq(request.duel_points_per_win),
                        tournaments::duel_points_per_loss.eq(request.duel_points_per_loss),
                        tournaments::duel_points_jackpot.eq(request.duel_points_jackpot),
                    ))
                    .get_result::<Tournament>(&*db)
                    .unwrap();
                users.shuffle(&mut thread_rng());
                for round in 1..=rounds {
                    for pair in users.chunks(2) {
                        diesel::insert_into(matches::table)
                            .values((
                                matches::tournament.eq(tournament.id),
                                matches::round.eq(round),
                                matches::duelist1.eq(pair.get(0)),
                                matches::duelist2.eq(pair.get(1)),
                                matches::duelist1_reported_winning.eq(Option::<bool>::None),
                                matches::duelist2_reported_winning.eq(Option::<bool>::None),
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

#[derive(Deserialize)]
pub struct ReportMatchRequest {
    match_id: i32,
    won: bool,
    username: String,
    password: String,
}

#[post("/report_match", data = "<request>")]
pub fn report_match(db: Database, request: Json<ReportMatchRequest>) -> JsonValue {
    if let Ok(user) = db.authenticate_user_succeeded(&request.username, &request.password) {
        let _match = matches::table
            .filter(matches::id.eq(request.match_id))
            .first::<Match>(&*db);
        if let Ok(_match) = _match {
            // TODO: Update duel points when both users agree on the outcome
            if Some(user.id) == _match.duelist1 {
                if _match.duelist1_reported_winning.is_none() {
                    diesel::update(matches::table)
                        .set(matches::duelist1_reported_winning.eq(Some(request.won)))
                        .execute(&*db)
                        .unwrap();
                    json!({
                        "r.type": "response",
                    })
                } else {
                    json!({
                        "r.type": "error",
                        "info": "You may not resubmit a match result",
                    })
                }
            } else if Some(user.id) == _match.duelist2 {
                if _match.duelist2_reported_winning.is_none() {
                    diesel::update(matches::table)
                        .set(matches::duelist2_reported_winning.eq(Some(request.won)))
                        .execute(&*db)
                        .unwrap();
                    json!({
                        "r.type": "response",
                    })
                } else {
                    json!({
                        "r.type": "error",
                        "info": "You may not resubmit a match result",
                    })
                }
            } else {
                json!({
                    "r.type": "error",
                    "info": "User not in match",
                })
            }
        } else {
            json!({
                "r.type": "error",
                "info": "Invalid match id.",
            })
        }
    } else {
        json!({
            "r.type": "error",
            "info": "Invalid username/password.",
        })
    }
}
