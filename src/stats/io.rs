use std::cmp::min;
use std::path::Path;
use std::{sync::Arc, collections::HashMap};

use chrono::{Local, Datelike, Timelike, DateTime};
use tokio::sync::Mutex;

use crate::DONT_MESSAGE_ON_PRIVATE_REPOS;
use crate::util::{write_file, read_file_utf8};
use crate::web::discord::request::model::Webhook;
use crate::web::discord::request::post::post;
use crate::web::github::model::{GithubStats, GithubRepoStats, GithubConfig};

use std::time::{Duration, SystemTime};
use std::thread::sleep;

const STATS_PATH: &str = "repo.stats";

pub async fn collect(stats: Arc<Mutex<GithubConfig>>, data: HashMap<String, serde_json::Value>)
{

    let mut held_stats = stats.lock().await;

    let mut name = match data["repository"]["name"].as_str()
    {
        Some(s) => s.to_string(),
        None => return
    };

    let push = if data.contains_key("pusher")
    {
        1
    }
    else 
    {
        0
    };

    if data["repository"]["private"].as_bool().is_some_and(|x|x)
    {
        name = format!("{}_private", name);
    }

    let stars = data["repository"]["stargazers_count"].as_u64().unwrap();

    let new_stats = GithubRepoStats {stars: stars, pushes: push};

    if !held_stats.get_stats().repos.contains_key(&name)
    {
        held_stats.get_stats().repos.insert(name.to_string(), GithubRepoStats::new());
    }

    held_stats.get_stats().repos.get_mut(&name).unwrap().update(new_stats);


    if Path::new(STATS_PATH).exists()
    {
        match std::fs::copy(STATS_PATH, format!("{}.bk",STATS_PATH))
        {
            Ok(_) => {},
            Err(why) => 
            {
                crate::debug(format!("error {} copying stats to {}.bk", why, STATS_PATH), None);
                return
            }
        }
    }

    match serde_json::to_string_pretty(held_stats.get_stats())
    {
        Ok(se) => 
        {
            write_file(STATS_PATH, se.as_bytes())
        },
        Err(why) => 
        {
            crate::debug(format!("error {} writing stats to {}", why, STATS_PATH), None);
            return
        }
    }

    crate::debug(format!("wrote data"), None);
}

pub async fn watch(disc: Webhook)
{
    let mut last_message = SystemTime::UNIX_EPOCH;
    loop 
    {
        let date = Local::now();

        if date.weekday() == chrono::Weekday::Fri && last_message.elapsed().unwrap().as_secs() > 24*60*60
        {
            last_message = SystemTime::now();
            
            let data = match read_file_utf8(STATS_PATH)
            {
                Some(d) => d,
                None =>
                {
                    crate::debug(format!("error reading stats at {}", STATS_PATH), None);
                    break
                }
            };

            let stats: GithubStats = match serde_json::from_str(&data)
            {
                Ok(data) => {data},
                Err(why) => 
                {
                    crate::debug(format!("error {} reading stats at {}", why, STATS_PATH), None);
                    break
                }
            };

            let mut pushes: Vec<(u64, u64, String)> = Vec::new();

            for repo in stats.repos.into_iter()
            {
                if repo.0.contains("private") && DONT_MESSAGE_ON_PRIVATE_REPOS
                {
                    continue;
                }

                pushes.push((repo.1.pushes, repo.1.stars, repo.0));
            }

            if pushes.len() == 0
            {
                break;
            }

            pushes.sort_by(| a:&(u64, u64, String), b:&(u64, u64, String) | b.0.partial_cmp(&a.0).unwrap());

            if pushes[0].0 == 0 
            {
                continue;
            }

            let mut msg = "Top activity this week :bar_chart:\n".to_string();

            for i in 0..min(pushes.len(), 3)
            {
                msg.push_str(format!("    {} | {} pushes | {} stars\n", pushes[i].2, pushes[i].0, pushes[i].1).as_str());
            }
            
            match post(disc.clone(), msg).await
            {
                Ok(_) => {},
                Err(e) => {crate::debug(format!("error posting message to discord {}", e), Some("stats watch".to_string()))}
            }
        }

        sleep(Duration::from_secs(60*60));
    }
}