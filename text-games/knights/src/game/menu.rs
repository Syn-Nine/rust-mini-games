extern crate console;
use console::style;
use console::Term;
use rand::Rng;

#[derive(PartialEq, Eq)]
pub enum MenuEnum {
    Arms,
    Train,
    Disband,
    Diplomacy,
    Warroom,
    Demand,
    Invade,
    Main,
    Realm,
    Quit,
    TributeDemanded,
    Notification,
}

use super::StatusEnum;

pub fn print_header(data: &super::GameData) {
    println!("\n{}:", print_name(data));
    println!(
        "{}",
        style("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
            .color256(8)
    );
    println!("{}\n", print_description(data));
}

fn print_name(data: &super::GameData) -> String {
    match data.menu {
        MenuEnum::Main => format!("Kingdom of Player, Year {}", data.year),
        MenuEnum::Arms => "Men at Arms".to_string(),
        MenuEnum::Train => "Men at Arms".to_string(),
        MenuEnum::Disband => "Men at Arms".to_string(),
        MenuEnum::Diplomacy => "Diplomacy".to_string(),
        MenuEnum::Demand => "Diplomacy".to_string(),
        MenuEnum::TributeDemanded => "Diplomacy".to_string(),
        MenuEnum::Warroom => "War Room".to_string(),
        MenuEnum::Invade => "War Room".to_string(),
        MenuEnum::Realm => "Kingdoms of the Realm".to_string(),
        MenuEnum::Quit => format!("Kingdom of Player, Year {}", data.year),
        MenuEnum::Notification => data.notifications[0].from.to_string(),
    }
}

fn print_description(data: &super::GameData) -> String {
    match data.menu {
        MenuEnum::Main => print_stats(data),
        MenuEnum::Arms => print_arms(data, false),
        MenuEnum::Train => print_arms(data, false),
        MenuEnum::Disband => print_arms(data, false),
        MenuEnum::Diplomacy => print_relations(data),
        MenuEnum::Demand => print_relations(data),
        MenuEnum::TributeDemanded => print_tribute_demanded(data),
        MenuEnum::Warroom => print_mights(data),
        MenuEnum::Invade => print_mights(data),
        MenuEnum::Realm => print_kingdoms(data, false),
        MenuEnum::Quit => print_quit(data),
        MenuEnum::Notification => format!(
            "Your Highness,\n{}",
            data.notifications[0].message.to_string()
        ),
    }
}

pub fn print_options(data: &super::GameData) -> String {
    match data.menu {
        MenuEnum::Main => {
            let mut ret = String::new();
            ret.push_str(" [A] Men at Arms\n");
            ret.push_str(" [D] Diplomacy\n");
            ret.push_str(" [W] War Room\n");
            ret.push_str(" [K] Kingdoms of the Realm\n");
            ret.push_str(" [H] Help\n");
            ret.push_str(" [Q] Quit Game\n");
            ret.push_str("\n [E] End Turn\n");
            ret
        }
        MenuEnum::Arms => {
            let mut ret = String::new();
            ret.push_str(" [T] Train\n");
            ret.push_str(" [D] Disband\n");
            ret.push_str("\n [R] Return\n");
            ret
        }
        MenuEnum::Train => {
            let mut ret = String::new();
            let eps = 1.0e-6;

            ret.push_str(" [B] Train Barbarians  -  10g\n");
            if data.kingdoms[0].pop > 25.0 - eps {
                ret.push_str(" [P] Train Pikemen     -  15g\n");
            }
            if data.kingdoms[0].pop > 50.0 - eps {
                ret.push_str(" [A] Train Archers     -  20g\n");
            }
            if data.kingdoms[0].pop > 100.0 - eps {
                ret.push_str(" [L] Train Arbalesters -  30g\n");
            }
            if data.kingdoms[0].pop > 200.0 - eps {
                ret.push_str(" [K] Train Knights     - 200g\n");
            }
            ret.push_str("\n [R] Return\n");
            ret
        }
        MenuEnum::Disband => {
            let mut ret = String::new();

            if 0 < data.kingdoms[0].barbarians {
                let mut n = data.kingdoms[0].barbarians;
                if 10 < n {
                    n = 10;
                }
                ret = format!("{} [B] Disband {} Barbarians\n", ret, n);
            }
            if 0 < data.kingdoms[0].pikemen {
                let mut n = data.kingdoms[0].pikemen;
                if 10 < n {
                    n = 10;
                }
                ret = format!("{} [P] Disband {} Pikemen\n", ret, n);
            }
            if 0 < data.kingdoms[0].archers {
                let mut n = data.kingdoms[0].archers;
                if 10 < n {
                    n = 10;
                }
                ret = format!("{} [A] Disband {} Archers\n", ret, n);
            }
            if 0 < data.kingdoms[0].arbalests {
                let mut n = data.kingdoms[0].arbalests;
                if 10 < n {
                    n = 10;
                }
                ret = format!("{} [L] Disband {} Arbalesters\n", ret, n);
            }
            if 0 < data.kingdoms[0].knights {
                let mut n = data.kingdoms[0].knights;
                if 10 < n {
                    n = 10;
                }
                ret = format!("{} [K] Disband {} Knights\n", ret, n);
            }
            ret.push_str("\n [R] Return\n");
            ret
        }
        MenuEnum::Diplomacy => {
            let mut ret = String::new();
            ret.push_str(" [D] Demand Tribute\n");
            ret.push_str("\n [R] Return");
            ret
        }
        MenuEnum::TributeDemanded => {
            let mut ret = String::new();
            ret.push_str(" [I] Ignore Demand\n");
            ret.push_str(" [P] Pay Tribute");
            ret
        }
        MenuEnum::Warroom => {
            let mut ret = String::new();
            ret.push_str(" [I] Invade\n");
            ret.push_str("\n [R] Return");
            ret
        }
        MenuEnum::Demand => print_demand_options(data),
        MenuEnum::Invade => print_invade_options(data),
        MenuEnum::Realm => " [R] Return".to_string(),
        MenuEnum::Notification => " [D] Dismiss".to_string(),
        _ => "".to_string(),
    }
}

fn print_quit(data: &super::GameData) -> String {
    let mut ret = "End of Game Stats:\n".to_string();

    ret = format!(
        "{}{:>16}: {}\n",
        ret,
        "Population",
        (data.kingdoms[0].pop * 100.0).floor()
    );
    ret = format!("{}{:>16}: {}\n", ret, "Gold", data.kingdoms[0].gold);
    ret = format!(
        "{}{:>16}: {}\n",
        ret, "From Plundering", data.kingdoms[0].gold_plundered
    );
    ret = format!(
        "{}{:>16}: {}\n",
        ret, "From Tribute", data.kingdoms[0].gold_tribute
    );
    ret = format!("{}{:>16}: {}\n", ret, "Lands", data.kingdoms[0].land);
    ret = format!(
        "{}{:>16}: {}\n",
        ret, "Total Might", data.kingdoms[0].might as i32
    );

    ret = format!("{}\nMilitary Stats:\n{}", ret, print_arms(data, true));

    ret = format!("{}\nEnemy Stats:\n{}", ret, print_arms_killed(data));

    ret = format!("{}\nKingdoms Destroyed:\n{}", ret, print_kings_killed(data));

    ret = format!("{}\nThank you for playing.", ret);
    ret
}

fn print_tribute_demanded(data: &super::GameData) -> String {
    let mut ret = String::new();

    if data.kingdoms[0].demands.is_empty() {
        return ret;
    }

    ret = format!(
        "Your Highness,\nYou have received a demand of {} gold tribute from {}\n",
        data.kingdoms[0].demands[0].tribute, data.kingdoms[data.kingdoms[0].demands[0].who].ruler
    );

    ret = format!(
        "{}{:>12}: {}\n",
        ret, "Their Might", data.kingdoms[data.kingdoms[0].demands[0].who].might as i32
    );
    ret = format!(
        "{}{:>12}: {}\n",
        ret, "Your Might", data.kingdoms[0].might as i32
    );
    ret = format!(
        "{}{:>12}: {}\n",
        ret, "Your Gold", data.kingdoms[0].gold as i32
    );
    ret
}

fn print_arms(data: &super::GameData, short: bool) -> String {
    let mut ret = String::new();

    ret = format!("{}{:>12} | {} | {}\n", ret, "  Unit    ", "Qty  ", "Might");
    ret = format!(
        "{}-------------------------------------------------------------------\n",
        ret
    );

    if data.kingdoms[0].barbarians > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Barbarians",
            data.kingdoms[0].barbarians,
            (data.kingdoms[0].barbarians as f64 * 2.0 * 0.30) as i32
        );
    }
    if data.kingdoms[0].pikemen > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Pikemen",
            data.kingdoms[0].pikemen,
            (data.kingdoms[0].pikemen as f64 * 3.0 * 0.40) as i32
        );
    }
    if data.kingdoms[0].archers > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Archers",
            data.kingdoms[0].archers,
            (data.kingdoms[0].archers as f64 * 3.0 * 0.50) as i32
        );
    }
    if data.kingdoms[0].arbalests > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Arbalests",
            data.kingdoms[0].arbalests,
            (data.kingdoms[0].arbalests as f64 * 5.0 * 0.60) as i32
        );
    }
    if data.kingdoms[0].knights > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Knights",
            data.kingdoms[0].knights,
            (data.kingdoms[0].knights as f64 * 20.0 * 1.0) as i32
        );
    }

    if false == short {
        ret = format!("{}\n{:>12}: {}\n", ret, "Gold", data.kingdoms[0].gold);
        ret = format!(
            "{}{:>12}: {}\n",
            ret, "Tot Might", data.kingdoms[0].might as i32
        );
    }

    ret
}

fn print_arms_killed(data: &super::GameData) -> String {
    let mut ret = String::new();

    ret = format!("{}{:>12} | {} | {}\n", ret, " Killed   ", "Qty  ", "Might");
    ret = format!(
        "{}-------------------------------------------------------------------\n",
        ret
    );

    if data.kingdoms[0].bkill > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Barbarians",
            data.kingdoms[0].bkill,
            (data.kingdoms[0].bkill as f64 * 2.0 * 0.30) as i32
        );
    }
    if data.kingdoms[0].pkill > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Pikemen",
            data.kingdoms[0].pkill,
            (data.kingdoms[0].pkill as f64 * 3.0 * 0.40) as i32
        );
    }
    if data.kingdoms[0].akill > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Archers",
            data.kingdoms[0].akill,
            (data.kingdoms[0].akill as f64 * 3.0 * 0.50) as i32
        );
    }
    if data.kingdoms[0].arkill > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Arbalests",
            data.kingdoms[0].arkill,
            (data.kingdoms[0].arkill as f64 * 5.0 * 0.60) as i32
        );
    }
    if data.kingdoms[0].kkill > 0 {
        ret = format!(
            "{}{:>11}  | {:<5} | {}\n",
            ret,
            "Knights",
            data.kingdoms[0].kkill,
            (data.kingdoms[0].kkill as f64 * 20.0 * 1.0) as i32
        );
    }

    ret
}

fn print_stats(data: &super::GameData) -> String {
    let mut ret = String::new();

    ret = format!(
        "{}{:>12}: {}\n",
        ret,
        "Population",
        (data.kingdoms[0].pop * 100.0).floor()
    );
    ret = format!("{}{:>12}: {}\n", ret, "Gold", data.kingdoms[0].gold);
    ret = format!("{}{:>12}: {}\n", ret, "Lands", data.kingdoms[0].land);
    ret = format!(
        "{}{:>12}: {}\n",
        ret, "Might", data.kingdoms[0].might as i32
    );

    ret
}

fn print_kingdoms(data: &super::GameData, borders_only: bool) -> String {
    let mut ret = String::new();

    ret = format!(
        "{}{:>19} | {:<5} | {:<8} | {:<8} | {:<7} | {}\n",
        ret, "Kingdom     ", "Lands", "Pop x100", "Might", "Status", "P Win"
    );
    ret = format!(
        "{}-------------------------------------------------------------------\n",
        ret
    );

    for i in 0..25 {
        let kingdom = &data.kingdoms[i];

        if 0 == kingdom.land {
            continue;
        }
        let mut s = " ".to_string();

        if data.kingdoms[0].borders[i] {
            s = "*".to_string();
        } else {
            if borders_only {
                continue;
            }
        }

        let mut status = "".to_string();

        if 0 < i {
            status = "Neutral".to_string();
            if StatusEnum::Peace == data.kingdoms[0].status[i] {
                status = "Peace".to_string();
            } else if StatusEnum::War == data.kingdoms[0].status[i] {
                status = "War".to_string();
            }
        }

        let mut pwin: f64 = 0.0;
        let mut swin = String::new();

        if 0 != i {
            let m0: f64 = data.kingdoms[0].might;
            let m1: f64 = kingdom.might;

            for _j in 0..20 {
                let r0 = 0.8 + rand::thread_rng().gen::<f64>() * 0.4;
                let r1 = 0.8 + rand::thread_rng().gen::<f64>() * 0.4;
                if m0 * r0 > m1 * r1 {
                    pwin = pwin + 1.0;
                }
            }

            pwin = pwin * 100.0 / 20.0;
            swin = format!("{}%", pwin);
        }

        ret = format!(
            "{}{:>20}| {:<5} | {:<8} | {:<8} | {:<7} | {}\n",
            ret,
            format!("{}{}", kingdom.ruler, s),
            kingdom.land,
            kingdom.pop.floor(),
            kingdom.might as i32,
            status,
            swin
        );
    }

    if borders_only {
        return ret;
    }

    ret = format!("{}\n\n    ", ret);

    for y in 0..5 {
        ret = format!("{}------------------------------------\n    ", ret);
        for x in 0..5 {
            let idx = data.map[y * 5 + x];
            ret = format!("{}| {} ", ret, data.kingdoms[idx].ruler[0..4].to_string());
        }
        ret = format!("{}|\n    ", ret);
        //
        for x in 0..5 {
            let idx = data.map[y * 5 + x];
            let n = data.kingdoms[idx].ruler.len();
            ret = format!(
                "{}| {} ",
                ret,
                data.kingdoms[idx].ruler[(n - 4)..n].to_string()
            );
        }
        ret = format!("{}|\n    ", ret);
        //
        for x in 0..5 {
            let idx = y * 5 + x;
            ret = format!("{}| {:>3}v ", ret, data.pop[idx].floor());
        }
        ret = format!("{}|\n    ", ret);
        //
        for x in 0..5 {
            let idx = y * 5 + x;
            ret = format!("{}| {:>3}g ", ret, (data.pop[idx].floor() * 0.30).floor());
        }
        ret = format!("{}|\n    ", ret);
    }
    ret = format!("{}------------------------------------\n    ", ret);

    ret
}

fn print_kings_killed(data: &super::GameData) -> String {
    let mut ret = String::new();

    for i in 0..data.kingdoms[0].civkill.len() {
        ret = format!(
            "{}  {}\n",
            ret, data.kingdoms[data.kingdoms[0].civkill[i]].ruler
        );
    }

    ret
}

fn print_relations(data: &super::GameData) -> String {
    print_kingdoms(data, true)
}

fn print_mights(data: &super::GameData) -> String {
    let mut ret = print_kingdoms(data, true);

    ret = format!("{}\n{:>10}: {}\n", ret, "Gold", data.kingdoms[0].gold);
    ret = format!("{}{:>10}: {}\n", ret, "Lands", data.kingdoms[0].land);
    ret = format!(
        "{}{:>10}: {:}\n",
        ret, "Might", data.kingdoms[0].might as i32
    );

    ret
}

fn print_demand_options(data: &super::GameData) -> String {
    let mut ret = String::new();

    let options = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ];

    let mut c = 0;

    for i in 0..25 {
        let kingdom = &data.kingdoms[i];

        if 0 == kingdom.land {
            continue;
        }

        if !data.kingdoms[0].borders[i] {
            continue;
        }

        let cost: i32 = (kingdom.pop * 0.1).ceil() as i32;

        ret = format!(
            "{} [{}] {:<20} - Demand {}g in tribute\n",
            ret, options[c], kingdom.ruler, cost
        );
        c = c + 1;
    }

    ret.push_str("\n [R] Return\n");
    ret
}

fn print_invade_options(data: &super::GameData) -> String {
    let mut ret = String::new();

    let options = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ];

    let mut c = 0;

    for i in 0..25 {
        let kingdom = &data.kingdoms[i];

        if 0 == kingdom.land {
            continue;
        }

        if !data.kingdoms[0].borders[i] {
            continue;
        }

        let cost: i32 = kingdom.land * 100;

        ret = format!(
            "{} [{}] {:<20} - {}g\n",
            ret, options[c], kingdom.ruler, cost
        );
        c = c + 1;
    }

    ret.push_str("\n [R] Return\n");
    ret
}
