// LORD - Legend of the Rusty Dragon - A text adventure by Syn9
// Inspired by Seth Able's BBS classic, Legend of the Red Dragon
//
extern crate console;

use console::style;
use console::Term;
use rand::Rng;

#[derive(PartialEq, Eq)]
enum RoomEnum {
    RoomTown,
    RoomTavern,
    RoomTavernRest,
    RoomTavernLore,
    RoomArena,
    RoomArenaChallenge,
    RoomBank,
    RoomBankDeposit,
    RoomBankWithdraw,
    RoomWeapons,
    RoomWeaponsBuy,
    RoomWeaponsSell,
    RoomArmour,
    RoomArmourBuy,
    RoomArmourSell,
    RoomStats,
    RoomForest,
    RoomForestCombat,
    RoomQuit,
    RoomWin,
}

#[derive(PartialEq, Eq)]
enum WeaponEnum {
    WeaponNothing,
    WeaponStick,
    WeaponDagger,
    WeaponShortSword,
    WeaponLongSword,
}

#[derive(PartialEq, Eq)]
enum ArmourEnum {
    ArmourNothing,
    ArmourCloak,
    ArmourLeatherVest,
    ArmourBreastPlate,
    ArmourPlateMail,
}

struct Player {
    health: i32,
    max_health: i32,
    gold: u32,
    gold_bank: u32,
    level: u32,
    exp: u32,
    weapon: WeaponEnum,
    armour: ArmourEnum,
    encounter: Enemy,
    blackout: bool,
}

impl Player {
    fn new() -> Player {
        let start_health = player_max_health(&1);

        Player {
            health: start_health,
            max_health: start_health,
            level: 1,
            gold: 40,
            gold_bank: 0,
            exp: 0,
            weapon: WeaponEnum::WeaponNothing,
            armour: ArmourEnum::ArmourNothing,
            encounter: Enemy::new("", &1, 1, ""),
            blackout: false,
        }
    }
}

struct Enemy {
    name: String,
    health: i32,
    max_health: i32,
    weapon: String,
    gold: u32,
    exp: u32,
    eidx: u32,
}

impl Enemy {
    fn new(name: &str, level: &u32, eidx: u32, weapon: &str) -> Enemy {
        let start_health = enemy_max_health(level, &eidx);
        Enemy {
            name: String::from(name),
            health: start_health,
            max_health: start_health,
            weapon: String::from(weapon),
            gold: enemy_gold(level, &eidx),
            exp: enemy_exp(level, &eidx),
            eidx,
        }
    }
}

fn reset_view() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

fn srand() -> f32 {
    0.8 + rand::thread_rng().gen::<f32>() * 0.4
}

fn enemy_gold(level: &u32, eidx: &u32) -> u32 {
    let ret = (enemy_max_health(level, eidx) as u32) * level;
    ((ret as f32) * srand()).ceil() as u32
}

fn enemy_exp(level: &u32, eidx: &u32) -> u32 {
    let ret = (enemy_max_health(level, eidx) as u32) * level;
    ((ret as f32) * 1.5 * srand()).ceil() as u32
}

fn enemy_max_health(level: &u32, eidx: &u32) -> i32 {
    let mut ret: f32 = 20.0 * 2.0_f32.powf((level - 1) as f32);
    ret = ret * (0.4 + (*eidx as f32) * 0.1);
    if 5 == *eidx {
        ret = ret * 1.5;
    }
    (ret * srand()).ceil() as i32
}

fn enemy_atk(level: &u32, eidx: &u32) -> i32 {
    let base_atk = 3.0 + (*eidx as f32) * 0.5;
    (base_atk * (level * level) as f32) as i32
}

fn enemy_def(level: &u32, eidx: &u32) -> i32 {
    let base_def = 1.5 + (*eidx as f32) * 0.5;
    (base_def * (level * level) as f32) as i32
}

fn player_max_health(level: &u32) -> i32 {
    let ret = 20.0 * 2.0_f32.powf((level - 1) as f32);
    ret.ceil() as i32
}

fn player_atk(stats: &Player) -> i32 {
    let base_atk: f32 = ((4 + stats.level) * stats.level) as f32;
    (base_atk
        * ((match stats.weapon {
            WeaponEnum::WeaponNothing => 0,
            WeaponEnum::WeaponStick => 1,
            WeaponEnum::WeaponDagger => 2,
            WeaponEnum::WeaponShortSword => 3,
            WeaponEnum::WeaponLongSword => 4,
        } as f32)
            * 0.5
            + 1.0))
        .ceil() as i32
}

fn player_def(stats: &Player) -> i32 {
    let base_atk: f32 = ((1 + stats.level) * stats.level) as f32;
    (base_atk
        * ((match stats.armour {
            ArmourEnum::ArmourNothing => 0,
            ArmourEnum::ArmourCloak => 1,
            ArmourEnum::ArmourLeatherVest => 2,
            ArmourEnum::ArmourBreastPlate => 3,
            ArmourEnum::ArmourPlateMail => 4,
        } as f32)
            * 0.5
            + 1.0))
        .ceil() as i32
}

fn tavern_price(level: &u32) -> u32 {
    match *level {
        1 => 5,
        2 => 20,
        3 => 60,
        4 => 150,
        _ => panic!(),
    }
}

fn level_up_exp(level: &u32) -> u32 {
    match *level {
        1 => 300,
        2 => 2000,
        3 => 7000,
        4 => 20000,
        _ => panic!(),
    }
}

fn main() {
    reset_view();
    show_welcome();
    input_continue();
    reset_view();

    let mut stats = Player::new();
    let mut room: RoomEnum = RoomEnum::RoomTown;

    loop {
        println!("\n{}:", room_name(&room));
        println!(
            "{}",
            style("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
                .color256(8)
        );
        println!("{}", room_description(&room, &mut stats));

        if room == RoomEnum::RoomQuit || room == RoomEnum::RoomWin {
            break;
        }

        get_valid_input(&mut room, &mut stats);
    }
}

fn input_continue() {
    println!("\nPress return to continue...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Expected input");
}

fn get_valid_input(mut room: &mut RoomEnum, mut stats: &mut Player) {
    println!(
        "\n{}\n{}",
        desc_color(room, String::from("Options:")),
        room_options(&room, &mut stats)
    );

    loop {
        if *room == RoomEnum::RoomBankDeposit || *room == RoomEnum::RoomBankWithdraw {
            let money = get_money_input();
            if perform_money_action(money, &mut room, &mut stats) {
                return;
            }
        } else {
            let option = get_player_input();
            if perform_action(&option, &mut room, &mut stats) {
                return;
            }
        }
        println!("Try Again:");
    }
}

fn get_player_input() -> u8 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Expected input");

    input.as_bytes()[0]
}

fn get_money_input() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Expected input");

    let money: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    money
}

fn show_welcome() {
    println!(
        "\n{}{}",
        style("LORD - Legend of the Rusty Dragon").color256(9),
        style(" - A text adventure by Syn9").color256(7)
    );
    println!(
        "{}",
        style("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-").color256(8)
    );
    println!(
        "{}",
        style("Inspired by Seth Able's classic BBS game Legend of the Red Dragon.").color256(7)
    );
}

fn perform_action(option: &u8, room: &mut RoomEnum, stats: &mut Player) -> bool {
    match room {
        RoomEnum::RoomTown => {
            match *option {
                b't' => *room = RoomEnum::RoomTavern,
                b'b' => *room = RoomEnum::RoomBank,
                b'n' => *room = RoomEnum::RoomArena,
                b'a' => *room = RoomEnum::RoomArmour,
                b'w' => *room = RoomEnum::RoomWeapons,
                b'f' => *room = RoomEnum::RoomForest,
                b's' => *room = RoomEnum::RoomStats,
                b'q' => *room = RoomEnum::RoomQuit,
                _ => return false,
            };
            reset_view();
        }
        RoomEnum::RoomTavern => match *option {
            b'g' if charge_for_room(stats) => *room = RoomEnum::RoomTavernRest,
            b'l' => *room = RoomEnum::RoomTavernLore,
            b'r' => {
                *room = RoomEnum::RoomTown;
                reset_view();
            }
            _ => return false,
        },
        RoomEnum::RoomTavernRest => match *option {
            b'r' => *room = RoomEnum::RoomTavern,
            _ => return false,
        },
        RoomEnum::RoomTavernLore => match *option {
            b'r' => *room = RoomEnum::RoomTavern,
            _ => return false,
        },
        RoomEnum::RoomArena => match *option {
            b'c' if stats.level < 4 => {
                let req = level_up_exp(&stats.level);
                if stats.exp < req {
                    println!(
                        "{}",
                        style("Come back when you have more experience.").color256(11)
                    );
                } else {
                    *room = RoomEnum::RoomArenaChallenge;
                    stats.encounter = generate_boss(&stats.level);
                }
            }
            b'r' => {
                *room = RoomEnum::RoomTown;
                reset_view();
            }
            _ => return false,
        },
        RoomEnum::RoomArenaChallenge => match *option {
            b'a' => {
                encounter_step(stats);
                if stats.health <= 0 {
                    println!("{}", style("\nBLACK OUT!\n").red());
                    stats.health = 1;
                    stats.blackout = true;
                    *room = RoomEnum::RoomTavern;
                } else if stats.encounter.health <= 0 {
                    println!("{}", style("\nYOU WIN! -- LEVEL UP!").color256(10));
                    stats.level = stats.level + 1;
                    stats.max_health = player_max_health(&stats.level);
                    stats.health = stats.max_health;
                    if stats.level == 4 {
                        println!(
                            "{}",
                            style("You are the master Gladiator now.").color256(14)
                        );
                    }
                    *room = RoomEnum::RoomArena;
                }
            }
            _ => return false,
        },
        RoomEnum::RoomBank => match *option {
            b'd' => *room = RoomEnum::RoomBankDeposit,
            b'w' => *room = RoomEnum::RoomBankWithdraw,
            b'r' => {
                *room = RoomEnum::RoomTown;
                reset_view();
            }
            _ => return false,
        },
        RoomEnum::RoomForest => match *option {
            b'l' => {
                *room = RoomEnum::RoomForestCombat;
                stats.encounter = generate_enemy(&stats);
            }
            b'r' => {
                *room = RoomEnum::RoomTown;
                reset_view();
            }
            _ => return false,
        },
        RoomEnum::RoomForestCombat => match *option {
            b'a' => {
                encounter_step(stats);
                if stats.health <= 0 {
                    println!("{}", style("\nBLACK OUT!\n").red());
                    stats.gold = 0;
                    stats.health = 1;
                    stats.blackout = true;
                    *room = RoomEnum::RoomTavern;
                } else if stats.encounter.health <= 0 {
                    println!("{}", style("\nYOU WIN!\n").yellow());
                    let gold = stats.encounter.gold;
                    let exp = stats.encounter.exp;
                    stats.gold = stats.gold + gold;
                    stats.exp = stats.exp + exp;
                    println!(
                        "{}",
                        style(format!("You find {} gold and gain {} exp!", gold, exp)).yellow()
                    );
                    *room = RoomEnum::RoomForest;
                    if 5 == stats.encounter.eidx {
                        *room = RoomEnum::RoomWin;
                    }
                }
            }
            b'r' => {
                run_encounter_step(stats);
                *room = RoomEnum::RoomForest;
                if stats.health <= 0 {
                    println!("{}", style("\nBLACK OUT!\n").red());
                    stats.gold = 0;
                    stats.health = 1;
                    stats.blackout = true;
                    *room = RoomEnum::RoomTavern;
                }
            }
            _ => return false,
        },
        RoomEnum::RoomArmour => match *option {
            b'b' => *room = RoomEnum::RoomArmourBuy,
            b's' => *room = RoomEnum::RoomArmourSell,
            b'r' => {
                *room = RoomEnum::RoomTown;
                reset_view();
            }
            _ => return false,
        },
        RoomEnum::RoomArmourBuy => match *option {
            b'1' => buy_armour(room, stats, ArmourEnum::ArmourCloak),
            b'2' => buy_armour(room, stats, ArmourEnum::ArmourLeatherVest),
            b'3' => buy_armour(room, stats, ArmourEnum::ArmourBreastPlate),
            b'4' => buy_armour(room, stats, ArmourEnum::ArmourPlateMail),
            b'r' => *room = RoomEnum::RoomArmour,
            _ => return false,
        },
        RoomEnum::RoomArmourSell => match *option {
            b's' if stats.armour != ArmourEnum::ArmourNothing => sell_armour(room, stats),
            b'r' => *room = RoomEnum::RoomArmour,
            _ => return false,
        },
        RoomEnum::RoomWeapons => match *option {
            b'b' => *room = RoomEnum::RoomWeaponsBuy,
            b's' => *room = RoomEnum::RoomWeaponsSell,
            b'r' => {
                *room = RoomEnum::RoomTown;
                reset_view();
            }
            _ => return false,
        },
        RoomEnum::RoomWeaponsBuy => match *option {
            b'1' => buy_weapon(room, stats, WeaponEnum::WeaponStick),
            b'2' => buy_weapon(room, stats, WeaponEnum::WeaponDagger),
            b'3' => buy_weapon(room, stats, WeaponEnum::WeaponShortSword),
            b'4' => buy_weapon(room, stats, WeaponEnum::WeaponLongSword),
            b'r' => *room = RoomEnum::RoomWeapons,
            _ => return false,
        },
        RoomEnum::RoomWeaponsSell => match *option {
            b's' if stats.weapon != WeaponEnum::WeaponNothing => sell_weapon(room, stats),
            b'r' => *room = RoomEnum::RoomWeapons,
            _ => return false,
        },
        RoomEnum::RoomStats => match *option {
            b'r' => {
                *room = RoomEnum::RoomTown;
                reset_view();
            }
            _ => return false,
        },
        _ => return false,
    }
    true
}

fn perform_money_action(money: u32, room: &mut RoomEnum, stats: &mut Player) -> bool {
    match room {
        RoomEnum::RoomBankDeposit => {
            if money == 1 {
                stats.gold_bank = stats.gold_bank + stats.gold;
                println!(
                    "{}",
                    style(format!("You deposited {} gold", stats.gold)).color256(10)
                );
                stats.gold = 0;
            } else if money <= stats.gold {
                stats.gold = stats.gold - money;
                stats.gold_bank = stats.gold_bank + money;
                println!(
                    "{}",
                    style(format!("You deposited {} gold", money)).color256(10)
                );
            } else if money > stats.gold {
                println!(
                    "{}",
                    style(format!("You don't have that much gold on hand!")).red()
                );
                return false;
            }
        }
        RoomEnum::RoomBankWithdraw => {
            if money == 1 {
                stats.gold = stats.gold + stats.gold_bank;
                println!(
                    "{}",
                    style(format!("You withdrew {} gold", stats.gold_bank)).color256(10)
                );
                stats.gold_bank = 0;
            } else if money <= stats.gold_bank {
                stats.gold = stats.gold + money;
                stats.gold_bank = stats.gold_bank - money;
                println!(
                    "{}",
                    style(format!("You withdrew {} gold", money)).color256(10)
                );
            } else if money > stats.gold_bank {
                println!(
                    "{}",
                    style("You don't have that much gold in the bank!").red()
                );
                return false;
            }
        }
        _ => return false,
    }
    *room = RoomEnum::RoomBank;
    true
}

fn room_options(room: &RoomEnum, stats: &Player) -> String {
    match room {
        RoomEnum::RoomTown => {
            let mut ret = String::new();
            ret.push_str(" [T] Tavern\n");
            ret.push_str(" [B] Bank\n");
            ret.push_str(" [N] Arena\n");
            ret.push_str(" [A] Armour Store\n");
            ret.push_str(" [W] Weapon Store\n");
            ret.push_str(" [F] Black Forest\n");
            ret.push_str(" [S] Your Stats\n");
            ret.push_str(" [Q] Quit Game\n");
            ret
        }
        RoomEnum::RoomTavern => {
            let mut ret = String::new();
            ret = format!(
                "{} [G] Get a room to rest for {} gold\n",
                ret,
                tavern_price(&stats.level)
            );
            ret.push_str(" [L] Lore\n");
            ret.push_str(" [R] Return to town\n");
            ret
        }
        RoomEnum::RoomTavernRest => String::from(" [R] Return\n"),
        RoomEnum::RoomTavernLore => String::from(" [R] Return\n"),
        RoomEnum::RoomArena => {
            let mut ret = String::new();
            if stats.level == 4 {
                ret = format!(
                    "{}{}",
                    ret,
                    style(" You are the master Gladiator now.\n").color256(14)
                );
            } else {
                ret.push_str(" [C] Challenge Gladiator\n");
            }
            ret.push_str(" [R] Return to town\n");
            ret
        }
        RoomEnum::RoomArenaChallenge => {
            let mut ret = String::new();
            ret.push_str(" [A] Attack\n");
            ret
        }
        RoomEnum::RoomBank => {
            let mut ret = String::new();
            ret.push_str(" [D] Deposit gold\n");
            ret.push_str(" [W] Withdraw gold\n");
            ret.push_str(" [R] Return to town\n");
            ret
        }
        RoomEnum::RoomBankDeposit => {
            String::from(" Enter how much gold you want to deposit (1 for all).\n")
        }
        RoomEnum::RoomBankWithdraw => {
            String::from(" Enter how much gold you want to withdraw (1 for all).\n")
        }
        RoomEnum::RoomForest => {
            let mut ret = String::new();
            ret.push_str(" [L] Look for something to kill\n");
            ret.push_str(" [R] Return to town\n");
            ret
        }
        RoomEnum::RoomForestCombat => {
            let mut ret = String::new();
            ret.push_str(" [A] Attack\n");
            ret.push_str(" [R] Run\n");
            ret
        }
        RoomEnum::RoomArmour => {
            let mut ret = String::new();
            ret.push_str(" [B] Buy armour\n");
            ret.push_str(" [S] Sell armour\n");
            ret.push_str(" [R] Return to town\n");
            ret
        }
        RoomEnum::RoomArmourBuy => {
            let mut ret = String::new();
            ret = format!(
                "{} [1] Cloak             {}g\n",
                ret,
                armour_buy_price(&ArmourEnum::ArmourCloak)
            );
            ret = format!(
                "{} [2] Leather Vest      {}g\n",
                ret,
                armour_buy_price(&ArmourEnum::ArmourLeatherVest)
            );
            ret = format!(
                "{} [3] Breast Plate     {}g\n",
                ret,
                armour_buy_price(&ArmourEnum::ArmourBreastPlate)
            );
            ret = format!(
                "{} [4] Plate Mail       {}g\n",
                ret,
                armour_buy_price(&ArmourEnum::ArmourPlateMail)
            );
            ret.push_str(" [R] Nevermind\n");
            ret = format!("{}\nYou have {} gold on hand\n", ret, stats.gold);
            ret
        }
        RoomEnum::RoomArmourSell => {
            let mut ret = String::new();
            if stats.armour != ArmourEnum::ArmourNothing {
                ret = format!(
                    "{} [S] Sell your {} for {} gold\n",
                    ret,
                    armour_name(&stats.armour),
                    armour_sell_price(&stats.armour)
                );
            } else {
                ret = format!(
                    "{}{}",
                    ret,
                    style(" You have nothing of value to sell...\n").red()
                );
            }
            ret.push_str(" [R] Nevermind\n");
            ret
        }
        RoomEnum::RoomWeapons => {
            let mut ret = String::new();
            ret.push_str(" [B] Buy weapon\n");
            ret.push_str(" [S] Sell weapon\n");
            ret.push_str(" [R] Return to town\n");
            ret
        }
        RoomEnum::RoomWeaponsBuy => {
            let mut ret = String::new();
            ret = format!(
                "{} [1] Stick            {}g\n",
                ret,
                weapon_buy_price(&WeaponEnum::WeaponStick)
            );
            ret = format!(
                "{} [2] Dagger           {}g\n",
                ret,
                weapon_buy_price(&WeaponEnum::WeaponDagger)
            );
            ret = format!(
                "{} [3] Short Sword     {}g\n",
                ret,
                weapon_buy_price(&WeaponEnum::WeaponShortSword)
            );
            ret = format!(
                "{} [4] Long Sword      {}g\n",
                ret,
                weapon_buy_price(&WeaponEnum::WeaponLongSword)
            );
            ret.push_str(" [R] Nevermind\n");
            ret = format!("{}\nYou have {} gold on hand\n", ret, stats.gold);
            ret
        }
        RoomEnum::RoomWeaponsSell => {
            let mut ret = String::new();
            if stats.weapon != WeaponEnum::WeaponNothing {
                ret = format!(
                    "{} [S] Sell your {} for {} gold\n",
                    ret,
                    weapon_name(&stats.weapon),
                    weapon_sell_price(&stats.weapon)
                );
            } else {
                ret = format!(
                    "{}{}",
                    ret,
                    style(" You have nothing of value to sell...\n").red()
                );
            }
            ret.push_str(" [R] Nevermind\n");
            ret
        }
        RoomEnum::RoomStats => {
            let mut ret = String::new();
            ret.push_str(" [R] Return to Town\n");
            ret
        }
        _ => String::new(),
    }
}

fn room_name(room: &RoomEnum) -> String {
    format!(
        "{}",
        match room {
            RoomEnum::RoomTown => style("South Ableton").color256(15),
            RoomEnum::RoomArena => style("Arena").color256(14),
            RoomEnum::RoomArenaChallenge => style("Arena Challenge!").color256(14),
            RoomEnum::RoomBank => style("Bank").color256(11),
            RoomEnum::RoomBankDeposit => style("Deposit Gold").color256(11),
            RoomEnum::RoomBankWithdraw => style("Withdraw Gold").color256(11),
            RoomEnum::RoomForest => style("Black Forest").color256(10),
            RoomEnum::RoomForestCombat => style("Encounter!").color256(9),
            RoomEnum::RoomArmour => style("Armour Store").color256(12),
            RoomEnum::RoomArmourBuy => style("Buy Armour").color256(12),
            RoomEnum::RoomArmourSell => style("Sell Armour").color256(12),
            RoomEnum::RoomWeapons => style("Weapon Store").color256(12),
            RoomEnum::RoomWeaponsBuy => style("Buy Weapon").color256(12),
            RoomEnum::RoomWeaponsSell => style("Sell Weapon").color256(12),
            RoomEnum::RoomStats => style("Stats").color256(15),
            RoomEnum::RoomTavern => style("Tavern").color256(5),
            RoomEnum::RoomTavernRest => style("Rest").color256(5),
            RoomEnum::RoomTavernLore => style("Lore").color256(5),
            RoomEnum::RoomQuit => style("Now leaving South Ableton").color256(2),
            RoomEnum::RoomWin => style("YOU WIN!").color256(14),
        }
    )
}

fn desc_color(room: &RoomEnum, ret: String) -> String {
    format!(
        "{}",
        match room {
            RoomEnum::RoomTown => style(ret).color256(2),
            RoomEnum::RoomArena => style(ret).color256(6),
            RoomEnum::RoomArenaChallenge => style(ret).color256(6),
            RoomEnum::RoomBank => style(ret).color256(3),
            RoomEnum::RoomBankDeposit => style(ret).color256(3),
            RoomEnum::RoomBankWithdraw => style(ret).color256(3),
            RoomEnum::RoomForest => style(ret).color256(2),
            RoomEnum::RoomForestCombat => style(ret).color256(1),
            RoomEnum::RoomArmour => style(ret).color256(12),
            RoomEnum::RoomArmourBuy => style(ret).color256(12),
            RoomEnum::RoomArmourSell => style(ret).color256(12),
            RoomEnum::RoomWeapons => style(ret).color256(12),
            RoomEnum::RoomWeaponsBuy => style(ret).color256(12),
            RoomEnum::RoomWeaponsSell => style(ret).color256(12),
            RoomEnum::RoomStats => style(ret).color256(2),
            RoomEnum::RoomTavern => style(ret).color256(5),
            RoomEnum::RoomTavernRest => style(ret).color256(5),
            RoomEnum::RoomTavernLore => style(ret).color256(5),
            RoomEnum::RoomQuit => style(ret).color256(7),
            RoomEnum::RoomWin => style(ret).color256(6),
        }
    )
}

fn room_description(room: &RoomEnum, stats: &mut Player) -> String {
    match room {
        RoomEnum::RoomTown => desc_color(&room, String::from("Another gloomy day in the old town of South Ableton.")),
        RoomEnum::RoomArena => desc_color(&room, String::from("Blood stained ground greets you as you prepare to be tested.")),
        RoomEnum::RoomArenaChallenge =>
        {
            let mut ret = format!("You challenge... {}\n\n", style(stats.encounter.name.to_string()).red());
            ret = format!("{} Your Health: {}/{}\n", ret, stats.health, stats.max_health);
            ret = format!("{} {} Health: {}/{}", ret, stats.encounter.name, stats.encounter.health, stats.encounter.max_health);
            ret
        }
        RoomEnum::RoomBank => desc_color(&room, format!("The ragged old teller looks at you sternly.\nYou have {} gold on hand.\nYou have {} gold in the bank.", stats.gold, stats.gold_bank)),
        RoomEnum::RoomBankDeposit => desc_color(&room, format!("You have {} gold on hand.\nYou have {} gold in the bank.", stats.gold, stats.gold_bank)),
        RoomEnum::RoomBankWithdraw => desc_color(&room, format!("You have {} gold on hand.\nYou have {} gold in the bank.", stats.gold, stats.gold_bank)),
        RoomEnum::RoomForest => desc_color(&room, String::from("Leaving the safety of the town, you creep down a dirt path into\nthe dark woodlands, greeted by creeking, croaking, and cackling\nechoes through the shadows of the Black Forest.")),
        RoomEnum::RoomForestCombat =>
        {
            let mut ret = format!("You have encountered... {}\n\n", style(stats.encounter.name.to_string()).red());
            ret = format!("{} Your Health: {}/{}\n", ret, stats.health, stats.max_health);
            ret = format!("{} {} Health: {}/{}", ret, stats.encounter.name, stats.encounter.health, stats.encounter.max_health);
            ret
        }
        RoomEnum::RoomArmour => desc_color(&room, String::from("The smell of leather, oil, and smoke billow out of the workshop.")),
        RoomEnum::RoomArmourBuy => desc_color(&room, String::from("See anything you like?")),
        RoomEnum::RoomArmourSell => desc_color(&room, String::from("Let's see what you have...")),
        RoomEnum::RoomWeapons => desc_color(&room, String::from("All around you see the shimmering steal of glorious weaponry.")),
        RoomEnum::RoomWeaponsBuy => desc_color(&room, String::from("See anything you like?")),
        RoomEnum::RoomWeaponsSell => desc_color(&room, String::from("Let's see what you have...")),
        RoomEnum::RoomTavern =>
        {
            if stats.blackout
            {
                stats.blackout = false;
                desc_color(&room, String::from("You wake up on the Tavern floor, bruised and bloody, missing\nall your gold on hand."))
            }
            else
            {
                desc_color(&room, String::from("You enter the dark, musty Tavern, welcomed by the incoherent\nramblings of drunkards and thieves."))
            }
        }
        RoomEnum::RoomTavernRest => desc_color(&room, String::from("You feel well rested after a long night's sleep.")),
        RoomEnum::RoomTavernLore => desc_color(&room, lore()),
        RoomEnum::RoomQuit => desc_color(&room, String::from("Thank you for playing.")),
        RoomEnum::RoomWin => desc_color(&room, String::from("You have slain the great Rusty Dragon and become the savior of the\nrealm. Your name shall ever be synonymous with valor.")),
        RoomEnum::RoomStats =>
        {
            let mut ret = String::new();
            ret = format!("{} Health: {}/{}\n", ret, stats.health, stats.max_health);
            ret = format!("{} Lvl: {}\n", ret, stats.level);
            ret = format!("{} Atk: {}\n", ret, player_atk(stats));
            ret = format!("{} Def: {}\n", ret, player_def(stats));
            ret = format!("{} Exp: {}\n", ret, stats.exp);
            ret = format!("{} Gold: {}\n", ret, stats.gold);
            ret = format!("{} Bank: {}\n", ret, stats.gold_bank);
            ret = format!("{} Weapon: {}\n", ret, weapon_name(&stats.weapon));
            ret = format!("{} Armour: {}", ret, armour_name(&stats.armour));
            desc_color(&room, ret)
        }
    }
}

fn armour_name(armour: &ArmourEnum) -> String {
    String::from(match armour {
        ArmourEnum::ArmourNothing => "Rags",
        ArmourEnum::ArmourCloak => "Cloak",
        ArmourEnum::ArmourLeatherVest => "Leather Vest",
        ArmourEnum::ArmourBreastPlate => "Breast Plate",
        ArmourEnum::ArmourPlateMail => "Plate Mail",
    })
}

fn weapon_name(weapon: &WeaponEnum) -> String {
    String::from(match weapon {
        WeaponEnum::WeaponNothing => "Fists",
        WeaponEnum::WeaponStick => "Stick",
        WeaponEnum::WeaponDagger => "Dagger",
        WeaponEnum::WeaponShortSword => "Short Sword",
        WeaponEnum::WeaponLongSword => "Long Sword",
    })
}

fn weapon_sell_price(weapon: &WeaponEnum) -> u32 {
    weapon_buy_price(&weapon) / 2
}

fn weapon_buy_price(weapon: &WeaponEnum) -> u32 {
    match weapon {
        WeaponEnum::WeaponStick => 100,
        WeaponEnum::WeaponDagger => 500,
        WeaponEnum::WeaponShortSword => 1600,
        WeaponEnum::WeaponLongSword => 4000,
        _ => 0,
    }
}

fn armour_sell_price(armour: &ArmourEnum) -> u32 {
    armour_buy_price(&armour) / 2
}

fn armour_buy_price(armour: &ArmourEnum) -> u32 {
    match armour {
        ArmourEnum::ArmourCloak => 100,
        ArmourEnum::ArmourLeatherVest => 500,
        ArmourEnum::ArmourBreastPlate => 1600,
        ArmourEnum::ArmourPlateMail => 4000,
        _ => 0,
    }
}

fn buy_armour(room: &mut RoomEnum, stats: &mut Player, armour: ArmourEnum) {
    let price = armour_buy_price(&armour);
    if stats.gold >= price {
        stats.gold = stats.gold - price;
        stats.armour = armour;
        println!(
            "{}",
            style(format!("You purchased {}.", armour_name(&stats.armour))).color256(10)
        );
        *room = RoomEnum::RoomArmour;
    } else {
        println!("{}", style("You don't have enough gold!").red());
    }
}

fn buy_weapon(room: &mut RoomEnum, stats: &mut Player, weapon: WeaponEnum) {
    let price = weapon_buy_price(&weapon);
    if stats.gold >= price {
        stats.gold = stats.gold - price;
        stats.weapon = weapon;
        println!(
            "{}",
            style(format!("You purchased {}.", weapon_name(&stats.weapon))).color256(10)
        );
        *room = RoomEnum::RoomWeapons;
    } else {
        println!("{}", style("You don't have enough gold!").red());
    }
}

fn sell_armour(room: &mut RoomEnum, stats: &mut Player) {
    let value = armour_sell_price(&stats.armour);
    println!(
        "{}",
        style(format!(
            "You sold your {} for {} gold.",
            armour_name(&stats.armour),
            value
        ))
        .color256(10)
    );
    stats.gold = stats.gold + value;
    stats.armour = ArmourEnum::ArmourNothing;
    *room = RoomEnum::RoomArmour;
}

fn sell_weapon(room: &mut RoomEnum, stats: &mut Player) {
    let value = weapon_sell_price(&stats.weapon);
    println!(
        "{}",
        style(format!(
            "You sold your {} for {} gold.",
            weapon_name(&stats.weapon),
            value
        ))
        .color256(10)
    );
    stats.gold = stats.gold + value;
    stats.weapon = WeaponEnum::WeaponNothing;
    *room = RoomEnum::RoomWeapons;
}

fn charge_for_room(stats: &mut Player) -> bool {
    let price = tavern_price(&stats.level);
    if stats.gold >= price {
        stats.gold = stats.gold - price;
        stats.health = stats.max_health;
        return true;
    }
    println!("{}", style("You don't have enough gold!").red());
    false
}

fn lore() -> String {
    let mut ret = String::from("Barkeep:");
    ret = format!("{} Angry Karen says that the Rusty Dragon killed her children.\nWe all know that's a lie...", ret);
    ret
}

fn generate_enemy(stats: &Player) -> Enemy {
    let mut idx = rand::thread_rng().gen_range(0..5);
    let level = stats.level;
    if 4 == level && stats.exp > level_up_exp(&stats.level) {
        idx = 5;
    }
    match level {
        1 => match idx {
            0 => Enemy::new("Slime", &level, 0, "Floppy Claw"),
            1 => Enemy::new("Woodland Fairy", &level, 1, "Wonderlust"),
            2 => Enemy::new("Flesh Eating Ladybug", &level, 2, "Sticky Spit"),
            3 => Enemy::new("Trash Panda", &level, 3, "Dumpster Fire"),
            4 => Enemy::new("Chainsmoking Cranky Canary", &level, 4, "Cigarette Butts"),
            _ => panic!(),
        },
        2 => match idx {
            0 => Enemy::new("Smooth Talking Pitcher Plant", &level, 0, "Sweet Talk"),
            1 => Enemy::new("Stink Horn", &level, 1, "Stink Trap"),
            2 => Enemy::new("Karma Chimera", &level, 2, "Earworm"),
            3 => Enemy::new("Wombat", &level, 3, "Dirty Trick"),
            4 => Enemy::new("Leering Larry", &level, 4, "Evil Eye"),
            _ => panic!(),
        },
        3 => match idx {
            0 => Enemy::new("Magic Marsupial", &level, 0, "Pixie Dust"),
            1 => Enemy::new("Black Swan", &level, 1, "Mope"),
            2 => Enemy::new("Unemployed Accountant", &level, 2, "One Thousand Papercuts"),
            3 => Enemy::new("Ostracised Ostritch", &level, 3, "Awkward Dance"),
            4 => Enemy::new("Mountain Man", &level, 4, "Slingblade"),
            _ => panic!(),
        },
        4 => match idx {
            0 => Enemy::new("English Teacher", &level, 0, "Student Debt"),
            1 => Enemy::new("Dare Bear", &level, 1, "Double Dog Dare"),
            2 => Enemy::new("Rowdy Redneck", &level, 2, "Truck Nuts"),
            3 => Enemy::new("Stinkerbell", &level, 3, "Swamp Gas"),
            4 => Enemy::new("Chupacabra", &level, 4, "Hot Breath"),
            //
            5 => Enemy::new("]=== RUSTY DRAGON ===[", &level, 5, "RUSTY RAZOR CLAWS"),
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn generate_boss(level: &u32) -> Enemy {
    match level {
        1 => Enemy::new("Gladiator", level, 4, "Gladius"),
        2 => Enemy::new("Gladiator Leutenant", level, 4, "Gladius Omega"),
        3 => Enemy::new("Gladiator Alpha", level, 4, "Gladius Prime"),
        _ => panic!(),
    }
}

fn encounter_step(stats: &mut Player) {
    let hit = calculate_hit(stats);
    if hit <= 0 {
        println!("{}", style("You miss!").red());
    } else {
        println!(
            "{}",
            style(format!(
                "You attack with {} for {} damage!",
                weapon_name(&stats.weapon),
                hit
            ))
            .color256(10)
        );
        stats.encounter.health = stats.encounter.health - hit;
        if stats.encounter.health <= 0 {
            return;
        }
    }

    run_encounter_step(stats);
}

fn run_encounter_step(stats: &mut Player) {
    let dmg = calculate_dmg(stats);
    if dmg <= 0 {
        println!(
            "{}",
            style(format!("{} misses!", stats.encounter.name)).red()
        );
    } else {
        println!(
            "{}",
            style(format!(
                "{} attacks with {} for {} damage!",
                stats.encounter.name, stats.encounter.weapon, dmg
            ))
            .red()
        );
        stats.health = stats.health - dmg;
    }
}

fn calculate_hit(stats: &Player) -> i32 {
    ((player_atk(stats) as f32 * srand())
        - (enemy_def(&stats.level, &stats.encounter.eidx) as f32 * srand()))
    .floor() as i32
}

fn calculate_dmg(stats: &Player) -> i32 {
    ((enemy_atk(&stats.level, &stats.encounter.eidx) as f32 * srand())
        - (player_def(stats) as f32 * srand()))
    .floor() as i32
}
