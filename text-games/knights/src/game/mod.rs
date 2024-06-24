use rand::Rng;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error, Write};

mod gen;
mod menu;

use menu::MenuEnum;

#[derive(PartialEq, Eq)]
pub enum StatusEnum {
    Neutral,
    Peace,
    War,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CommandEnum {
    Skip,
    TrainBarbarians,
    TrainPikemen,
    TrainArchers,
    TrainArbalesters,
    TrainKnights,
    InvadeLesser,
    InvadeEqual,
    InvadeHigher,
    DemandLesser,
    DemandEqual,
    DemandHigher,
    //
    NumCommands,
}

struct TributeDemanded {
    tribute: i32,
    who: usize,
}

struct CommandLog {
    land: i32,
    might: f64,
    gold: i32,
    command: CommandEnum,
}

struct Notification {
    from: String,
    message: String,
}

struct Kingdom {
    ruler: String,
    land: i32,
    might: f64,
    gold: i32,
    pop: f64,
    barbarians: i32,
    pikemen: i32,
    archers: i32,
    arbalests: i32,
    knights: i32,
    borders: Vec<bool>,
    status: Vec<StatusEnum>,
    gold_plundered: i32,
    gold_tribute: i32,
    bkill: i32,
    pkill: i32,
    akill: i32,
    arkill: i32,
    kkill: i32,
    civkill: Vec<usize>,
    bstv: i32,
    pstv: i32,
    astv: i32,
    arstv: i32,
    kstv: i32,
    net_gold_per_turn: i32,
    demands: VecDeque<TributeDemanded>,
}

impl Kingdom {
    fn new(name: &String) -> Kingdom {
        let mut b: Vec<bool> = Vec::new();
        for _i in 0..25 {
            b.push(false);
        }

        let mut s: Vec<StatusEnum> = Vec::new();
        for _i in 0..25 {
            s.push(StatusEnum::Peace);
        }

        Kingdom {
            ruler: name.to_string(),
            land: 1,
            pop: 0.0,
            might: 30.0,
            gold: 10,
            barbarians: 25,
            pikemen: 0,
            archers: 0,
            arbalests: 0,
            knights: 0,
            borders: b,
            status: s,
            gold_plundered: 0,
            gold_tribute: 0,
            bkill: 0,
            pkill: 0,
            akill: 0,
            arkill: 0,
            kkill: 0,
            civkill: Vec::new(),
            bstv: 0,
            pstv: 0,
            astv: 0,
            arstv: 0,
            kstv: 0,
            net_gold_per_turn: 0,
            demands: VecDeque::new(),
        }
    }
}

pub struct GameData {
    kingdoms: Vec<Kingdom>,
    menu: MenuEnum,
    year: i32,
    map: Vec<usize>,
    pop: Vec<f64>,
    notifications: VecDeque<Notification>,
    log: Vec<CommandLog>,
}

pub fn new() -> GameData {
    GameData::new()
}

impl GameData {
    fn new() -> GameData {
        GameData {
            kingdoms: Vec::new(),
            menu: MenuEnum::Main,
            year: 0,
            map: gen::gen_map(),
            pop: gen::gen_pop(),
            notifications: VecDeque::new(),
            log: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.initialize();

        loop {
            menu::print_header(self);

            if self.menu == MenuEnum::Quit {
                break;
            }

            self.get_valid_input();
        }

        // dump player choices to file
        {
            let mut outfile = File::create("choices.dat")?;

            //println!("Game Log:");
            for log in &self.log {
                /*println!(
                    "Land: {:<3}  Gold: {:<8}  Might: {:<8}  Command: {}",
                    log.land, log.gold, log.might, log.command as i32
                );*/
                write!(
                    outfile,
                    "{}",
                    format!(
                        "{},{},{},{}\n",
                        log.land, log.gold, log.might, log.command as i32
                    )
                )?;
            }
        }

        Ok(())
    }

    fn get_valid_input(&mut self) {
        println!("Options:\n{}", menu::print_options(self));

        loop {
            let option = GameData::get_player_input();
            if self.perform_action(&option) {
                return;
            }
            println!("Try Again:");
        }
    }

    fn push_log(&mut self, command: CommandEnum) {
        self.log.push(CommandLog {
            land: self.kingdoms[0].land,
            might: self.kingdoms[0].might,
            gold: self.kingdoms[0].gold,
            command: command,
        });
    }

    fn perform_action(&mut self, option: &u8) -> bool {
        match self.menu {
            MenuEnum::Main => match *option {
                b'e' => {
                    self.push_log(CommandEnum::Skip);
                    self.end_of_turn();
                }
                b'a' => self.menu = MenuEnum::Arms,
                b'd' => self.menu = MenuEnum::Diplomacy,
                b'w' => self.menu = MenuEnum::Warroom,
                b'k' => self.menu = MenuEnum::Realm,
                b'q' => self.menu = MenuEnum::Quit,
                _ => return false,
            },
            MenuEnum::Arms => match *option {
                b't' => self.menu = MenuEnum::Train,
                b'd' => self.menu = MenuEnum::Disband,
                b'r' => self.menu = MenuEnum::Main,
                _ => return false,
            },
            MenuEnum::Train => match *option {
                b'b' => {
                    if self.train_barbarian() {
                        self.push_log(CommandEnum::TrainBarbarians);
                        self.end_of_turn();
                    }
                }
                b'p' => {
                    if self.kingdoms[0].pop < 25.0 {
                        return false;
                    }
                    if self.train_pikeman() {
                        self.push_log(CommandEnum::TrainPikemen);
                        self.end_of_turn();
                    }
                }
                b'a' => {
                    if self.kingdoms[0].pop < 50.0 {
                        return false;
                    }
                    if self.train_archer() {
                        self.push_log(CommandEnum::TrainArchers);
                        self.end_of_turn();
                    }
                }
                b'l' => {
                    if self.kingdoms[0].pop < 100.0 {
                        return false;
                    }
                    if self.train_arbalester() {
                        self.push_log(CommandEnum::TrainArbalesters);
                        self.end_of_turn();
                    }
                }
                b'k' => {
                    if self.kingdoms[0].pop < 200.0 {
                        return false;
                    }
                    if self.train_knight() {
                        self.push_log(CommandEnum::TrainKnights);
                        self.end_of_turn();
                    }
                }
                b'r' => self.menu = MenuEnum::Arms,
                _ => return false,
            },
            MenuEnum::Disband => match *option {
                b'b' => {
                    let mut n = self.kingdoms[0].barbarians;
                    if 10 < n {
                        n = 10;
                    }
                    if 0 < n {
                        println!("{} Barbarians disbanded!\n", n);
                        self.kingdoms[0].barbarians = self.kingdoms[0].barbarians - n;
                    } else {
                        return false;
                    }
                }
                b'p' => {
                    let mut n = self.kingdoms[0].pikemen;
                    if 10 < n {
                        n = 10;
                    }
                    if 0 < n {
                        println!("{} Pikemen disbanded!\n", n);
                        self.kingdoms[0].pikemen = self.kingdoms[0].pikemen - n;
                    } else {
                        return false;
                    }
                }
                b'a' => {
                    let mut n = self.kingdoms[0].archers;
                    if 10 < n {
                        n = 10;
                    }
                    if 0 < n {
                        println!("{} Archers disbanded!\n", n);
                        self.kingdoms[0].archers = self.kingdoms[0].archers - n;
                    } else {
                        return false;
                    }
                }
                b'l' => {
                    let mut n = self.kingdoms[0].arbalests;
                    if 10 < n {
                        n = 10;
                    }
                    if 0 < n {
                        println!("{} Arbalesters disbanded!\n", n);
                        self.kingdoms[0].arbalests = self.kingdoms[0].arbalests - n;
                    } else {
                        return false;
                    }
                }
                b'k' => {
                    let mut n = self.kingdoms[0].knights;
                    if 10 < n {
                        n = 10;
                    }
                    if 0 < n {
                        println!("{} Knights disbanded!\n", n);
                        self.kingdoms[0].knights = self.kingdoms[0].knights - n;
                    } else {
                        return false;
                    }
                }
                b'r' => self.menu = MenuEnum::Arms,
                _ => return false,
            },
            MenuEnum::Diplomacy => match *option {
                b'd' => self.menu = MenuEnum::Demand,
                b'r' => self.menu = MenuEnum::Main,
                _ => return false,
            },
            MenuEnum::Demand => {
                if !self.check_demand(*option) {
                    return false;
                }
            }
            MenuEnum::TributeDemanded => match *option {
                b'i' => self.ignore_tribute(self.kingdoms[0].demands[0].who),
                b'p' => self.pay_tribute(self.kingdoms[0].demands[0].who),
                _ => return false,
            },
            MenuEnum::Warroom => match *option {
                b'i' => self.menu = MenuEnum::Invade,
                b'r' => self.menu = MenuEnum::Main,
                _ => return false,
            },
            MenuEnum::Invade => {
                if !self.check_invade(*option) {
                    return false;
                }
            }
            MenuEnum::Realm => match *option {
                b'r' => self.menu = MenuEnum::Main,
                _ => return false,
            },
            MenuEnum::Notification => match *option {
                b'd' => {
                    self.notifications.pop_front();
                    if 0 == self.notifications.len() {
                        self.menu = MenuEnum::Main;
                        if 0 == self.kingdoms[0].land || 25 == self.kingdoms[0].land {
                            self.menu = MenuEnum::Quit;
                        }
                    }
                }
                _ => return false,
            },
            _ => return false,
        }
        true
    }

    fn check_demand(&mut self, option: u8) -> bool {
        if 'r' == option as char {
            self.menu = MenuEnum::Warroom;
            return true;
        }

        let options = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        ];

        let mut c = 0;

        for i in 1..25 {
            if 0 == self.kingdoms[i].land {
                continue;
            }

            if !self.kingdoms[0].borders[i] {
                continue;
            }

            if option as char == options[c] {
                self.demand(0, i);
                return true;
            }

            c = c + 1;
        }
        return false;
    }

    fn ignore_tribute(&mut self, who: usize) {
        if StatusEnum::Peace == self.kingdoms[0].status[who] {
            self.kingdoms[0].status[who] = StatusEnum::Neutral;
            self.kingdoms[who].status[0] = StatusEnum::Neutral;
        }

        self.kingdoms[0].demands.pop_front();

        if self.kingdoms[0].demands.is_empty() {
            self.menu = MenuEnum::Main;
        }
    }

    fn pay_tribute(&mut self, who: usize) {
        if self.kingdoms[0].demands[0].tribute <= self.kingdoms[0].gold {
            self.kingdoms[0].gold = self.kingdoms[0].gold - self.kingdoms[0].demands[0].tribute;
            self.kingdoms[who].gold = self.kingdoms[who].gold + self.kingdoms[0].demands[0].tribute;
        } else {
            println!("You give everything you have!\n");
            self.kingdoms[who].gold = self.kingdoms[who].gold + self.kingdoms[0].gold;
            self.kingdoms[0].gold = 0;
        }

        self.kingdoms[0].status[who] = StatusEnum::Peace;
        self.kingdoms[who].status[0] = StatusEnum::Peace;

        self.kingdoms[0].demands.pop_front();

        if self.kingdoms[0].demands.is_empty() {
            self.menu = MenuEnum::Main;
        }
    }

    fn demand(&mut self, atk: usize, who: usize) {
        let mut tribute = (self.kingdoms[who].pop * 0.1).ceil() as i32;

        if 0 == who {
            self.kingdoms[0].demands.push_back(TributeDemanded {
                tribute: tribute,
                who: atk,
            });
            return;
        }

        let mut ret = format!(
            "You demand tribute of {} gold from the Kingdom of {}\n",
            tribute, self.kingdoms[who].ruler
        );

        if 0 == atk {
            if self.kingdoms[who].might < self.kingdoms[atk].might * 0.8 {
                self.push_log(CommandEnum::DemandLesser);
            } else if self.kingdoms[who].might > self.kingdoms[atk].might / 0.8 {
                self.push_log(CommandEnum::DemandHigher);
            } else {
                self.push_log(CommandEnum::DemandEqual);
            }
        }

        let k = &mut self.kingdoms;

        let m0 = k[atk].might;
        let m1 = k[who].might;

        let mut success = false;
        if m0 * (0.8 + rand::thread_rng().gen::<f64>() * 0.4) > m1 * 1.5 {
            success = true;
        }

        if success {
            if 0 == k[who].gold {
                ret = format!(
                    "{}  Unfortunately, they have no gold in their coffers.\n",
                    ret
                );
            } else if k[who].gold <= tribute {
                tribute = k[who].gold;
                ret = format!(
                    "{}  You take every last coin they have, {} gold.\n",
                    ret, tribute
                );
                k[who].gold = 0;
                k[atk].gold = k[atk].gold + tribute;
                k[atk].gold_tribute = k[atk].gold_tribute + tribute;
            } else {
                ret = format!("{}  Tribute has been granted, {} gold.\n", ret, tribute);
                k[who].gold = k[who].gold - tribute;
                k[atk].gold = k[atk].gold + tribute;
                k[atk].gold_tribute = k[atk].gold_tribute + tribute;
            }
            k[atk].status[who] = StatusEnum::Peace;
            k[who].status[atk] = StatusEnum::Peace;
        } else {
            ret = format!("{}  They ignore your demands!\n", ret);

            if StatusEnum::Peace == k[atk].status[who] {
                k[atk].status[who] = StatusEnum::Neutral;
                k[who].status[atk] = StatusEnum::Neutral;
            }
        }

        if 0 == atk {
            self.notifications.push_back(Notification {
                from: "Diplomacy Advisor".to_string(),
                message: ret,
            });

            self.end_of_turn();
            self.menu = MenuEnum::Notification;
        }
    }

    fn check_invade(&mut self, option: u8) -> bool {
        if 'r' == option as char {
            self.menu = MenuEnum::Warroom;
            return true;
        }

        let options = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        ];

        let mut c = 0;

        for i in 1..25 {
            if 0 == self.kingdoms[i].land {
                continue;
            }

            if !self.kingdoms[0].borders[i] {
                continue;
            }

            let cost: i32 = self.kingdoms[i].land * 100;

            if option as char == options[c] {
                if cost <= self.kingdoms[0].gold {
                    self.invade(0, i);
                    return true;
                } else {
                    println!(
                        "You don't have enough gold to invade {}",
                        self.kingdoms[i].ruler
                    );
                    return false;
                }
            }

            c = c + 1;
        }
        return false;
    }

    fn invade(&mut self, atk: usize, who: usize) {
        // already killed this round
        if 0 == self.kingdoms[atk].land {
            return;
        }
        if 0 == self.kingdoms[who].land {
            return;
        }

        self.kingdoms[atk].gold = self.kingdoms[atk].gold - self.kingdoms[who].land * 100;
        if 0 > self.kingdoms[atk].gold {
            self.kingdoms[atk].gold = 0;
        }

        let mut ret = format!(
            "You have invaded the Kingdom of {}\n",
            self.kingdoms[who].ruler
        );

        if 0 == who {
            ret = format!(
                "You have been invaded by the Kingdom of {}\n",
                self.kingdoms[atk].ruler
            );
        }

        println!("{} has invaded {}.\n", atk, who);

        if 0 == atk {
            if self.kingdoms[who].might < self.kingdoms[atk].might * 0.8 {
                self.push_log(CommandEnum::InvadeLesser);
            } else if self.kingdoms[who].might > self.kingdoms[atk].might / 0.8 {
                self.push_log(CommandEnum::InvadeHigher);
            } else {
                self.push_log(CommandEnum::InvadeEqual);
            }
        }

        let k = &mut self.kingdoms;

        k[atk].status[who] = StatusEnum::War;
        k[who].status[atk] = StatusEnum::War;

        let mut m0 = k[atk].might;
        let mut m1 = k[who].might;
        let mpl0 = m0 / k[atk].land as f64;
        let mpl1 = m1 / k[who].land as f64;

        let m0s = m0;
        let m1s = m1;

        for _i in 0..5 {
            let dm0 = m0 * (0.08 + rand::thread_rng().gen::<f64>() * 0.04);
            let dm1 = m1 * (0.08 + rand::thread_rng().gen::<f64>() * 0.04);

            m0 = m0 - dm1;
            if m0 < 0.0 {
                m0 = 0.0;
                break;
            }
            m1 = m1 - dm0;
            if m1 < 0.0 {
                m1 = 0.0;
                break;
            }
        }

        let mut kill0 = false;
        let mut kill1 = false;

        let mut nb = 0;
        let mut na = 0;
        let mut np = 0;
        let mut nar = 0;
        let mut nk = 0;

        loop {
            let mut cm = GameData::calc_might(&k[atk]);
            let mut pass = false;
            if k[atk].barbarians > 0 && cm - m0 >= 0.6 {
                k[atk].barbarians = k[atk].barbarians - 1;
                nb = nb + 1;
                cm = cm - 0.6;
                pass = true;
            }
            if k[atk].pikemen > 0 && cm - m0 >= 1.2 {
                k[atk].pikemen = k[atk].pikemen - 1;
                np = np + 1;
                cm = cm - 1.2;
                pass = true;
            }
            if k[atk].archers > 0 && cm - m0 >= 1.5 {
                k[atk].archers = k[atk].archers - 1;
                na = na + 1;
                cm = cm - 1.5;
                pass = true;
            }
            if k[atk].arbalests > 0 && cm - m0 >= 3.0 {
                k[atk].arbalests = k[atk].arbalests - 1;
                nar = nar + 1;
                cm = cm - 3.0;
                pass = true;
            }
            if k[atk].knights > 0 && cm - m0 >= 20.0 {
                k[atk].knights = k[atk].knights - 1;
                nk = nk + 1;
                //cm = cm - 20.0;
                pass = true;
            }
            if !pass {
                break;
            }
        }

        if 0 == (k[atk].barbarians
            + k[atk].pikemen
            + k[atk].archers
            + k[atk].arbalests
            + k[atk].knights)
        {
            kill0 = true;
            if 0 == atk {
                ret = format!("{}  All of your units were lost!\n", ret);
            } else if 0 == who {
                ret = format!("{}  All enemy units killed!\n", ret);
            }
        } else {
            if 0 == atk {
                if nb > 0 {
                    ret = format!("{}  {} Barbarians lost!\n", ret, nb);
                }
                if np > 0 {
                    ret = format!("{}  {} Pikemen lost!\n", ret, np);
                }
                if na > 0 {
                    ret = format!("{}  {} Archers lost!\n", ret, na);
                }
                if nar > 0 {
                    ret = format!("{}  {} Arbalesters lost!\n", ret, nar);
                }
                if nk > 0 {
                    ret = format!("{}  {} Knights lost!\n", ret, nk);
                }
            }
            if 0 == who {
                if nb > 0 {
                    ret = format!("{}  {} Enemy barbarians killed!\n", ret, nb);
                }
                if np > 0 {
                    ret = format!("{}  {} Enemy pikemen killed!\n", ret, np);
                }
                if na > 0 {
                    ret = format!("{}  {} Enemy archers killed!\n", ret, na);
                }
                if nar > 0 {
                    ret = format!("{}  {} Enemy arbalesters killed!\n", ret, nar);
                }
                if nk > 0 {
                    ret = format!("{}  {} Enemy knights killed!\n", ret, nk);
                }
            }
            if 0 == who {
                if m0 < m0s * 0.1 {
                    kill0 = true;

                    ret = format!("{}  Enemy has surrendered!\n", ret);
                }
            }
        }

        let mut nb = 0;
        let mut na = 0;
        let mut np = 0;
        let mut nar = 0;
        let mut nk = 0;

        loop {
            let mut cm = GameData::calc_might(&k[who]);
            let mut pass = false;
            if k[who].barbarians > 0 && cm - m1 >= 0.6 {
                k[who].barbarians = k[who].barbarians - 1;
                nb = nb + 1;
                cm = cm - 0.6;
                pass = true;
            }
            if k[who].pikemen > 0 && cm - m1 >= 1.2 {
                k[who].pikemen = k[who].pikemen - 1;
                np = np + 1;
                cm = cm - 1.2;
                pass = true;
            }
            if k[who].archers > 0 && cm - m1 >= 1.5 {
                k[who].archers = k[who].archers - 1;
                na = na + 1;
                cm = cm - 1.5;
                pass = true;
            }
            if k[who].arbalests > 0 && cm - m1 >= 3.0 {
                k[who].arbalests = k[who].arbalests - 1;
                nar = nar + 1;
                cm = cm - 3.0;
                pass = true;
            }
            if k[who].knights > 0 && cm - m1 >= 20.0 {
                k[who].knights = k[who].knights - 1;
                nk = nk + 1;
                //cm = cm - 20.0;
                pass = true;
            }
            if !pass {
                break;
            }
        }

        k[atk].bkill = k[atk].bkill + nb;
        k[atk].pkill = k[atk].pkill + np;
        k[atk].akill = k[atk].akill + na;
        k[atk].arkill = k[atk].arkill + nar;
        k[atk].kkill = k[atk].kkill + nk;

        if 0 == (k[who].barbarians
            + k[who].pikemen
            + k[who].archers
            + k[who].arbalests
            + k[who].knights)
        {
            kill1 = true;
            if 0 == atk {
                ret = format!("{}  All enemy units killed!\n", ret);
            } else if 0 == who {
                ret = format!("{}  All of your units were lost!\n", ret);
            }
        } else {
            if 0 == atk {
                if nb > 0 {
                    ret = format!("{}  {} Enemy barbarians killed!\n", ret, nb);
                }
                if np > 0 {
                    ret = format!("{}  {} Enemy pikemen killed!\n", ret, np);
                }
                if na > 0 {
                    ret = format!("{}  {} Enemy archers killed!\n", ret, na);
                }
                if nar > 0 {
                    ret = format!("{}  {} Enemy arbalesters killed!\n", ret, nar);
                }
                if nk > 0 {
                    ret = format!("{}  {} Enemy knights killed!\n", ret, nk);
                }
            }
            if 0 == who {
                if nb > 0 {
                    ret = format!("{}  {} Barbarians lost!\n", ret, nb);
                }
                if np > 0 {
                    ret = format!("{}  {} Pikemen lost!\n", ret, np);
                }
                if na > 0 {
                    ret = format!("{}  {} Archers lost!\n", ret, na);
                }
                if nar > 0 {
                    ret = format!("{}  {} Arbalesters lost!\n", ret, nar);
                }
                if nk > 0 {
                    ret = format!("{}  {} Knights lost!\n", ret, nk);
                }
            }
            if 0 == atk {
                if m1 < m1s * 0.1 {
                    kill1 = true;

                    ret = format!("{}  Enemy has surrendered!\n", ret);
                }
            }
        }

        let loss0: i32 = ((m0s - m0) / mpl0).floor() as i32;
        let loss1: i32 = ((m1s - m1) / mpl1).floor() as i32;
        let mut net = 0;
        if loss0 > loss1 {
            net = loss1 - loss0;
        }
        if loss1 > loss0 {
            net = loss1 - loss0;
        }

        if kill0 {
            net = -k[atk].land;
            k[who].civkill.push(atk);

            if 0 == atk {
                ret = format!("{}  All lands were lost!\n", ret);
                self.transfer_land(atk, who, -net);
                self.notifications.push_back(Notification {
                    from: "Military Advisor".to_string(),
                    message: ret,
                });

                self.end_of_turn();
                self.menu = MenuEnum::Notification;
                return;
            }
        }

        if kill1 {
            net = k[who].land;
            k[atk].civkill.push(who);

            if 0 == who {
                ret = format!("{}  All lands were lost!\n", ret);
                self.transfer_land(who, atk, net);
                self.notifications.push_back(Notification {
                    from: "Military Advisor".to_string(),
                    message: ret,
                });

                self.menu = MenuEnum::Notification;
                return;
            }
        }

        // if net is negative, attacker lost more land than defender
        if net <= -1 {
            net = -net;
            if net > k[atk].land {
                net = k[atk].land;
            }

            let mut plunder = (k[atk].gold as f64 * net as f64 / k[atk].land as f64) as i32;
            if plunder > k[atk].gold {
                plunder = k[atk].gold;
            }
            k[atk].gold = k[atk].gold - plunder;
            k[who].gold = k[who].gold + plunder;
            if 0 == atk {
                ret = format!("{}  {} gold lost\n", ret, plunder);
                ret = format!("{}  {} lands lost\n", ret, net);
            } else if 0 == who {
                ret = format!("{}  {} gold plundered\n", ret, plunder);
                ret = format!("{}  {} lands gained\n", ret, net);
            }
            self.transfer_land(atk, who, net);
        } else if net >= 1 {
            if net > k[who].land {
                net = k[who].land;
            }

            let mut plunder = (k[who].gold as f64 * net as f64 / k[who].land as f64) as i32;
            if plunder > k[who].gold {
                plunder = k[who].gold;
            }
            k[who].gold = k[who].gold - plunder;
            k[atk].gold = k[atk].gold + plunder;
            k[atk].gold_plundered = k[atk].gold_plundered + plunder;
            if 0 == atk {
                ret = format!("{}  {} gold plundered\n", ret, plunder);
                ret = format!("{}  {} lands gained\n", ret, net);
            } else if 0 == who {
                ret = format!("{}  {} gold lost\n", ret, plunder);
                ret = format!("{}  {} lands lost\n", ret, net);
            }
            self.transfer_land(who, atk, net);
        }

        if 0 == atk || 0 == who {
            self.notifications.push_back(Notification {
                from: "Military Advisor".to_string(),
                message: ret,
            });
            if 0 == atk {
                self.end_of_turn();
            }
            self.menu = MenuEnum::Notification;
        }
    }

    fn transfer_land(&mut self, from: usize, to: usize, land: i32) {
        println!("transfering {} lands from {} to {}", land, from, to);

        for _i in 0..land {
            let mut lands: Vec<usize> = Vec::new();

            // re-calculate borders
            for y in 0..5 {
                for x in 0..4 {
                    let idx0 = self.map[y * 5 + x + 0];
                    let idx1 = self.map[y * 5 + x + 1];

                    if idx0 == from {
                        if idx1 == to {
                            lands.push(y * 5 + x + 0);
                        }
                    } else if idx0 == to {
                        if idx1 == from {
                            lands.push(y * 5 + x + 1);
                        }
                    }
                }
            }
            for x in 0..5 {
                for y in 0..4 {
                    let idx0 = self.map[(y + 0) * 5 + x];
                    let idx1 = self.map[(y + 1) * 5 + x];

                    if idx0 == from {
                        if idx1 == to {
                            lands.push((y + 0) * 5 + x);
                        }
                    } else if idx0 == to {
                        if idx1 == from {
                            lands.push((y + 1) * 5 + x);
                        }
                    }
                }
            }

            if 0 == lands.len() {
                println!("failed to transfer lands from {} to {}", from, to);
                break;
            }

            let idx: usize = rand::thread_rng().gen_range(0..lands.len());
            self.map[lands[idx]] = to;
        }
    }

    fn train_barbarian(&mut self) -> bool {
        let mut k = &mut self.kingdoms[0];
        if k.gold >= 10 {
            k.gold = k.gold - 10;
            k.barbarians = k.barbarians + 10;
            return true;
        } else {
            println!("Not enough gold.");
        }
        false
    }

    fn train_pikeman(&mut self) -> bool {
        let mut k = &mut self.kingdoms[0];
        if k.gold >= 15 {
            k.gold = k.gold - 15;
            k.pikemen = k.pikemen + 10;
            return true;
        } else {
            println!("Not enough gold.");
        }
        false
    }

    fn train_archer(&mut self) -> bool {
        let mut k = &mut self.kingdoms[0];
        if k.gold >= 20 {
            k.gold = k.gold - 20;
            k.archers = k.archers + 10;
            return true;
        } else {
            println!("Not enough gold.");
        }
        false
    }

    fn train_arbalester(&mut self) -> bool {
        let mut k = &mut self.kingdoms[0];
        if k.gold >= 30 {
            k.gold = k.gold - 30;
            k.arbalests = k.arbalests + 10;
            return true;
        } else {
            println!("Not enough gold.");
        }
        false
    }

    fn train_knight(&mut self) -> bool {
        let mut k = &mut self.kingdoms[0];
        if k.gold >= 200 {
            k.gold = k.gold - 200;
            k.knights = k.knights + 10;
            return true;
        } else {
            println!("Not enough gold.");
        }
        false
    }

    fn get_player_input() -> u8 {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Expected input");

        input.as_bytes()[0]
    }

    fn initialize(&mut self) {
        let names = gen::gen_names();
        self.kingdoms.push(Kingdom::new(&"Player     ".to_string()));

        for name in names {
            self.kingdoms.push(Kingdom::new(&name));
        }

        self.calc_populations();
    }

    fn end_of_turn(&mut self) {
        self.menu = MenuEnum::Main;

        for idx in 1..25 {
            self.npc_turn(idx);
        }

        let mut vstv = 0;
        // comsume 1/2 resources
        for i in 0..25 {
            let pop: i32 = (self.pop[i] * 50.0) as i32;
            let mut gold: f64 = self.kingdoms[self.map[i]].gold as f64;

            let mut stv: i32 = 0;

            for _j in 0..pop {
                if gold >= 0.001 {
                    gold = gold - 0.001;
                } else {
                    stv = stv + 1;
                }
            }

            if (gold as i32) < self.kingdoms[self.map[i]].gold {
                self.kingdoms[self.map[i]].gold = gold.ceil() as i32;
            }
            if 0 < stv {
                self.pop[i] = self.pop[i] - (stv as f64 / 100.0);
                if self.pop[i] < 1.0 {
                    self.pop[i] = 1.0;
                }
                if 0 == self.map[i] {
                    vstv = vstv + stv;
                }
            }
        }

        let growth_rate = 0.30;

        for kingdom in &mut self.kingdoms {
            // resource growth
            kingdom.gold = kingdom.gold + (kingdom.pop * growth_rate).ceil() as i32;

            // recalc might
            kingdom.might = GameData::calc_might(kingdom);

            let mut gold: f64 = kingdom.gold as f64;

            kingdom.bstv = 0;
            kingdom.pstv = 0;
            kingdom.astv = 0;
            kingdom.arstv = 0;
            kingdom.kstv = 0;

            for _i in 0..kingdom.barbarians {
                if gold >= 0.1 {
                    gold = gold - 0.1;
                } else {
                    kingdom.bstv = kingdom.bstv + 1;
                }
            }
            for _i in 0..kingdom.pikemen {
                if gold >= 0.15 {
                    gold = gold - 0.15;
                } else {
                    kingdom.pstv = kingdom.pstv + 1;
                }
            }
            for _i in 0..kingdom.archers {
                if gold >= 0.2 {
                    gold = gold - 0.2;
                } else {
                    kingdom.astv = kingdom.astv + 1;
                }
            }
            for _i in 0..kingdom.arbalests {
                if gold >= 0.3 {
                    gold = gold - 0.3;
                } else {
                    kingdom.arstv = kingdom.arstv + 1;
                }
            }
            for _i in 0..kingdom.knights {
                if gold >= 2.0 {
                    gold = gold - 2.0;
                } else {
                    kingdom.kstv = kingdom.kstv + 1;
                }
            }

            kingdom.barbarians = kingdom.barbarians - kingdom.bstv;
            kingdom.pikemen = kingdom.pikemen - kingdom.pstv;
            kingdom.archers = kingdom.archers - kingdom.astv;
            kingdom.arbalests = kingdom.arbalests - kingdom.arstv;
            kingdom.knights = kingdom.knights - kingdom.kstv;

            if (gold as i32) < kingdom.gold {
                kingdom.gold = gold as i32;
            }
        }

        if 0 < (self.kingdoms[0].bstv
            + self.kingdoms[0].pstv
            + self.kingdoms[0].astv
            + self.kingdoms[0].arstv
            + self.kingdoms[0].kstv)
        {
            let mut msg = "The following units have disbanded due to starvation!".to_string();

            if self.kingdoms[0].bstv > 0 {
                msg = format!("{}\n  {} Barbarians", msg, self.kingdoms[0].bstv);
            }
            if self.kingdoms[0].pstv > 0 {
                msg = format!("{}\n  {} Pikemen", msg, self.kingdoms[0].pstv);
            }
            if self.kingdoms[0].astv > 0 {
                msg = format!("{}\n  {} Archers", msg, self.kingdoms[0].astv);
            }
            if self.kingdoms[0].arstv > 0 {
                msg = format!("{}\n  {} Arbalesters", msg, self.kingdoms[0].arstv);
            }
            if self.kingdoms[0].kstv > 0 {
                msg = format!("{}\n  {} Knights", msg, self.kingdoms[0].kstv);
            }

            self.notifications.push_back(Notification {
                from: "Domestic Advisor".to_string(),
                message: msg,
            });
            self.menu = MenuEnum::Notification;
        }

        // comsume 1/2 resources
        for i in 0..25 {
            let pop: i32 = (self.pop[i] * 50.0) as i32;
            let mut gold: f64 = self.kingdoms[self.map[i]].gold as f64;

            let mut stv: i32 = 0;

            for _j in 0..pop {
                if gold >= 0.001 {
                    gold = gold - 0.001;
                } else {
                    stv = stv + 1;
                }
            }

            if (gold as i32) < self.kingdoms[self.map[i]].gold {
                self.kingdoms[self.map[i]].gold = gold.ceil() as i32;
            }
            if 0 < stv {
                self.pop[i] = self.pop[i] - (stv as f64 / 100.0);
                if self.pop[i] < 1.0 {
                    self.pop[i] = 1.0;
                }
                if 0 == self.map[i] {
                    vstv = vstv + stv;
                }
            }
        }

        // update map cell populations
        for i in 0..25 {
            // population growth
            let t = self.pop[i];
            let t2 = (t - 250.0) / 250.0;
            let v = t2 * t2;
            let delta = 10.0 * (1.0 - v);
            self.pop[i] = self.pop[i] + delta;
        }

        if 0 < vstv {
            self.notifications.push_back(Notification {
                from: "Domestic Advisor".to_string(),
                message: format!("  {} villagers have starved to death!", vstv),
            });
            self.menu = MenuEnum::Notification;
        }

        let pre_pop = self.kingdoms[0].pop;
        self.calc_populations();
        let post_pop = self.kingdoms[0].pop;
        let eps = 1.0e-6;

        if pre_pop < 25.0 && post_pop >= 25.0 - eps {
            self.notifications.push_back(Notification {
                from: "Military Advisor".to_string(),
                message: "The training grounds can now support the Pikeman unit.".to_string(),
            });
            self.menu = MenuEnum::Notification;
        }
        if pre_pop < 50.0 && post_pop >= 50.0 - eps {
            self.notifications.push_back(Notification {
                from: "Military Advisor".to_string(),
                message: "The training grounds can now support the Archer unit.".to_string(),
            });
            self.menu = MenuEnum::Notification;
        }
        if pre_pop < 100.0 && post_pop >= 100.0 - eps {
            self.notifications.push_back(Notification {
                from: "Military Advisor".to_string(),
                message: "The training grounds can now support the Arbalester unit.".to_string(),
            });
            self.menu = MenuEnum::Notification;
        }
        if pre_pop < 200.0 && post_pop >= 200.0 - eps {
            self.notifications.push_back(Notification {
                from: "Military Advisor".to_string(),
                message: "The training grounds can now support the Knight unit.".to_string(),
            });
            self.menu = MenuEnum::Notification;
        }

        if 0 == self.kingdoms[0].land {
            self.notifications.push_back(Notification {
                from: "Domestic Advisor".to_string(),
                message: format!(
                    "Please forgive me, but your reign has ended, year {}.",
                    self.year
                ),
            });
        } else if 25 == self.kingdoms[0].land {
            self.notifications.push_back(Notification {
                from: "Domestic Advisor".to_string(),
                message: format!(
                    "Congratulations, your empire has conquered the known world, year {}!",
                    self.year
                ),
            });
        }

        for kingdom in &mut self.kingdoms {
            let mut net: f64 = kingdom.pop * 0.2;

            net = net - kingdom.barbarians as f64 * 0.1;
            net = net - kingdom.pikemen as f64 * 0.15;
            net = net - kingdom.archers as f64 * 0.2;
            net = net - kingdom.arbalests as f64 * 0.3;
            net = net - kingdom.knights as f64 * 2.0;

            kingdom.net_gold_per_turn = net.floor() as i32;
        }

        if 0 < self.kingdoms[0].demands.len() {
            self.menu = MenuEnum::TributeDemanded;
        }

        self.year = self.year + 5;
    }

    fn calc_populations(&mut self) {
        // reset populations
        for kingdom in &mut self.kingdoms {
            kingdom.land = 0;
            kingdom.pop = 0.0;
            for i in 0..25 {
                kingdom.borders[i] = false;
            }
        }

        // calculate new populations
        for i in 0..25 {
            let idx = self.map[i];
            self.kingdoms[idx].land = self.kingdoms[idx].land + 1;
            self.kingdoms[idx].pop = self.kingdoms[idx].pop + self.pop[i];
        }

        // re-calculate borders
        for y in 0..5 {
            for x in 0..4 {
                let idx0 = self.map[y * 5 + x + 0];
                let idx1 = self.map[y * 5 + x + 1];

                if idx0 != idx1 {
                    self.kingdoms[idx0].borders[idx1] = true;
                    self.kingdoms[idx1].borders[idx0] = true;
                }
            }
        }
        for x in 0..5 {
            for y in 0..4 {
                let idx0 = self.map[(y + 0) * 5 + x];
                let idx1 = self.map[(y + 1) * 5 + x];

                if idx0 != idx1 {
                    self.kingdoms[idx0].borders[idx1] = true;
                    self.kingdoms[idx1].borders[idx0] = true;
                }
            }
        }
    }

    fn calc_might(kingdom: &Kingdom) -> f64 {
        let mut ret: f64 = 0.0;
        ret += 0.3 * 2.0 * kingdom.barbarians as f64; // 0.6
        ret += 0.4 * 3.0 * kingdom.pikemen as f64; // 1.2
        ret += 0.5 * 3.0 * kingdom.archers as f64; // 1.5
        ret += 0.6 * 5.0 * kingdom.arbalests as f64; // 3.0
        ret += 1.0 * 20.0 * kingdom.knights as f64; // 20
        ret
    }

    fn npc_turn(&mut self, idx: usize) {
        let k: &Kingdom = &self.kingdoms[idx];

        let cmds = self.get_commands(idx);
        let wgts = self.get_weights(k.land, k.gold, k.might, &cmds);

        let sel = rand::thread_rng().gen::<f64>();
        let mut sum = 0.0;
        let mut option: usize = 0;
        let eps = 1.0e-6;

        for i in 0..cmds.len() {
            if eps > wgts[i] {
                continue;
            }

            if sel >= sum {
                sum = sum + wgts[i];
                if sel < sum {
                    option = i;
                    break;
                }
            }
        }

        //println!("{}: {}", idx, option);

        match option {
            0 => self.npc_perform(CommandEnum::Skip, idx),
            1 => self.npc_perform(CommandEnum::TrainBarbarians, idx),
            2 => self.npc_perform(CommandEnum::TrainPikemen, idx),
            3 => self.npc_perform(CommandEnum::TrainArchers, idx),
            4 => self.npc_perform(CommandEnum::TrainArbalesters, idx),
            5 => self.npc_perform(CommandEnum::TrainKnights, idx),
            6 => self.npc_perform(CommandEnum::InvadeLesser, idx),
            7 => self.npc_perform(CommandEnum::InvadeEqual, idx),
            8 => self.npc_perform(CommandEnum::InvadeHigher, idx),
            9 => self.npc_perform(CommandEnum::DemandLesser, idx),
            10 => self.npc_perform(CommandEnum::DemandEqual, idx),
            11 => self.npc_perform(CommandEnum::DemandHigher, idx),
            _ => (),
        }

        self.calc_populations();
    }

    fn npc_perform(&mut self, cmd: CommandEnum, idx: usize) {
        let mut dlesser: Vec<usize> = Vec::new();
        let mut dequal: Vec<usize> = Vec::new();
        let mut dhigher: Vec<usize> = Vec::new();
        let mut ilesser: Vec<usize> = Vec::new();
        let mut iequal: Vec<usize> = Vec::new();
        let mut ihigher: Vec<usize> = Vec::new();

        for i in 0..25 {
            if i == idx {
                continue;
            }

            if self.kingdoms[idx].borders[i] {
                let m0 = self.kingdoms[i].might;
                if m0 < self.kingdoms[idx].might * 0.8 {
                    dlesser.push(i);
                    if self.kingdoms[idx].status[i] != StatusEnum::Peace
                        && self.kingdoms[idx].gold <= self.kingdoms[i].land * 100
                    {
                        ilesser.push(i);
                    }
                } else if m0 > self.kingdoms[idx].might / 0.8 {
                    dhigher.push(i);
                    if self.kingdoms[idx].status[i] != StatusEnum::Peace
                        && self.kingdoms[idx].gold <= self.kingdoms[i].land * 100
                    {
                        ihigher.push(i);
                    }
                } else {
                    dequal.push(i);
                    if self.kingdoms[idx].status[i] != StatusEnum::Peace
                        && self.kingdoms[idx].gold <= self.kingdoms[i].land * 100
                    {
                        iequal.push(i);
                    }
                }
            }
        }

        let mut k: &mut Kingdom = &mut self.kingdoms[idx];

        match cmd {
            CommandEnum::Skip => (),
            CommandEnum::TrainBarbarians => {
                k.gold = k.gold - 10;
                k.barbarians = k.barbarians + 10;
            }
            CommandEnum::TrainPikemen => {
                k.gold = k.gold - 15;
                k.pikemen = k.pikemen + 10;
            }
            CommandEnum::TrainArchers => {
                k.gold = k.gold - 20;
                k.archers = k.archers + 10;
            }
            CommandEnum::TrainArbalesters => {
                k.gold = k.gold - 30;
                k.arbalests = k.arbalests + 10;
            }
            CommandEnum::TrainKnights => {
                k.gold = k.gold - 200;
                k.knights = k.knights + 10;
            }
            CommandEnum::InvadeLesser => {
                if !ilesser.is_empty() {
                    let who = ilesser[rand::thread_rng().gen_range(0..ilesser.len()) as usize];
                    self.invade(idx, who);
                }
            }
            CommandEnum::InvadeEqual => {
                if !iequal.is_empty() {
                    let who = iequal[rand::thread_rng().gen_range(0..iequal.len()) as usize];
                    self.invade(idx, who);
                }
            }
            CommandEnum::InvadeHigher => {
                if !ihigher.is_empty() {
                    let who = ihigher[rand::thread_rng().gen_range(0..ihigher.len()) as usize];
                    self.invade(idx, who);
                }
            }
            CommandEnum::DemandLesser => {
                if !dlesser.is_empty() {
                    let who = dlesser[rand::thread_rng().gen_range(0..dlesser.len()) as usize];
                    self.demand(idx, who);
                }
            }
            CommandEnum::DemandEqual => {
                if !dequal.is_empty() {
                    let who = dequal[rand::thread_rng().gen_range(0..dequal.len()) as usize];
                    self.demand(idx, who);
                }
            }
            CommandEnum::DemandHigher => {
                if !dhigher.is_empty() {
                    let who = dhigher[rand::thread_rng().gen_range(0..dhigher.len()) as usize];
                    self.demand(idx, who);
                }
            }
            _ => (),
        }
    }

    fn get_weights(&self, _land: i32, _gold: i32, _might: f64, cmds: &Vec<bool>) -> Vec<f64> {
        let mut ret: Vec<f64> = Vec::new();

        let mut n: i32 = 0;
        for b in cmds {
            if *b {
                n = n + 1;
            }
        }

        // uniform dist
        let weight = 1.0 / (n as f64);

        for b in cmds {
            if *b {
                ret.push(weight);
            } else {
                ret.push(0.0);
            }
        }

        ret
    }

    fn get_commands(&self, idx: usize) -> Vec<bool> {
        let mut ret: Vec<bool> = Vec::new();

        for _i in 0..(CommandEnum::NumCommands as usize) {
            ret.push(false);
        }

        // scenarios
        ret[CommandEnum::Skip as usize] = true;

        let k: &Kingdom = &self.kingdoms[idx];
        let eps = 1.0e-6;

        if k.gold >= 10 && k.net_gold_per_turn > 2 {
            ret[CommandEnum::TrainBarbarians as usize] = true;
        }
        if k.pop > 25.0 - eps && k.gold >= 15 && k.net_gold_per_turn > 3 {
            ret[CommandEnum::TrainPikemen as usize] = true;
        }
        if k.pop > 50.0 - eps && k.gold >= 20 && k.net_gold_per_turn > 4 {
            ret[CommandEnum::TrainArchers as usize] = true;
        }
        if k.pop > 100.0 - eps && k.gold >= 30 && k.net_gold_per_turn > 6 {
            ret[CommandEnum::TrainArbalesters as usize] = true;
        }
        if k.pop > 200.0 - eps && k.gold >= 200 && k.net_gold_per_turn > 40 {
            ret[CommandEnum::TrainKnights as usize] = true;
        }

        let mut lesser: Vec<usize> = Vec::new();
        let mut equal: Vec<usize> = Vec::new();
        let mut higher: Vec<usize> = Vec::new();

        for i in 0..25 {
            if i == idx {
                continue;
            }

            if k.borders[i] {
                let m0 = self.kingdoms[i].might;
                if m0 < k.might * 0.8 {
                    lesser.push(i);
                } else if m0 > k.might / 0.8 {
                    higher.push(i);
                } else {
                    equal.push(i);
                }
            }
        }

        if !lesser.is_empty() {
            ret[CommandEnum::DemandLesser as usize] = true;
        }
        if !equal.is_empty() {
            ret[CommandEnum::DemandEqual as usize] = true;
        }
        if !higher.is_empty() {
            ret[CommandEnum::DemandHigher as usize] = true;
        }

        let mut lesser: Vec<usize> = Vec::new();
        let mut equal: Vec<usize> = Vec::new();
        let mut higher: Vec<usize> = Vec::new();

        for i in 0..25 {
            if i == idx {
                continue;
            }

            if k.borders[i] {
                if k.status[i] == StatusEnum::Peace {
                    continue;
                }
                if k.gold <= self.kingdoms[i].land * 100 {
                    continue;
                }
                let m0 = self.kingdoms[i].might;
                if m0 < k.might * 0.8 {
                    lesser.push(i);
                } else if m0 > k.might / 0.8 {
                    higher.push(i);
                } else {
                    equal.push(i);
                }
            }
        }

        if !lesser.is_empty() {
            ret[CommandEnum::InvadeLesser as usize] = true;
        }
        if !equal.is_empty() {
            ret[CommandEnum::InvadeEqual as usize] = true;
        }
        if !higher.is_empty() {
            ret[CommandEnum::InvadeHigher as usize] = true;
        }

        ret
    }
}
