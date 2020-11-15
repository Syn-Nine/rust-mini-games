// Asylum - A text adventure by Syn9
//
enum RoomEnum
{
    RoomEntrance,
    RoomFoyer,
    RoomHallway,
    RoomHobo,
    RoomOffice,
    RoomCloset,
    RoomStairwell,
    RoomRoof,
    RoomWin,
    RoomDiedHobo,
    RoomDiedDrugs,
    RoomDiedFall,
}

fn main() {

    println!("Asylum");
    println!("------------------------");
    println!("A text adventure by Syn9");

    let mut room: RoomEnum = RoomEnum::RoomEntrance;
    let mut flags: [bool;10] = [false;10];

    loop
    {
        println!("\n{}:", room_name(&room));
        println!("------------------------");
        println!("{}", room_description(&room, &flags));

        match room
        {
            RoomEnum::RoomDiedDrugs => break,
            RoomEnum::RoomDiedHobo => break,
            RoomEnum::RoomDiedFall => break,
            RoomEnum::RoomWin => break,
            _ => (),
        }

        get_valid_input(&mut room, &mut flags);

    }
}

fn perform_action(option: &u8, room: &mut RoomEnum, flags: &mut[bool;10]) -> bool
{
    match room
    {
        RoomEnum::RoomEntrance =>
        {
            if b'g' == *option
            {
                println!("\nYou climb your way through a small opening in the broken glass door.");
                *room = RoomEnum::RoomFoyer;
                return true;
            }
        },
        RoomEnum::RoomFoyer =>
        {
            if b'c' == *option
            {
                println!("\nYou slowly make your way down the dark hallway.");
                *room = RoomEnum::RoomHallway;
                return true;
            }
        },
        RoomEnum::RoomHallway =>
        {
            if b'b' == *option
            {
                println!("\nYou jimmy open the old wooden door.");
                *room = RoomEnum::RoomOffice;
                return true;
            }
            else if b'o' == *option
            {
                println!("\nThe door creeks loudly as you open it.");
                *room = RoomEnum::RoomCloset;
                return true;
            }
            else if b'i' == *option
            {
                println!("\nYou slowly walk toward a faint groaning sound in the dark corner.");
                *room = RoomEnum::RoomHobo;
                return true;
            }
            else if b's' == *option
            {
                if !flags[4]
                {
                    println!("\nYou try to open the stairwell door, but it won't budge!");
                    return true;
                }
                else
                {
                    println!("\nThe door is locked. You use the keys and with a some force it cracks open just enough to slide through.");
                    *room = RoomEnum::RoomStairwell;
                    return true;
                }
            }
            else if b'g' == *option
            {
                println!("Maybe it's better to head back to the Foyer.");
                *room = RoomEnum::RoomFoyer;
                return true;
            }
        },
        RoomEnum::RoomOffice =>
        {
            if !flags[0]
            {
                if b'l' == *option
                {
                    println!("\nThe door is stuck, but you pull with all your weight to bust it open.\nIt's full of office supplies and, strangely, a metal bat on the bottom shelf.");
                    flags[0] = true;
                    return true;
                }
            }
            else if flags[0] && !flags[1]
            {
                if b't' == *option
                {
                    println!("\nYou grab the old dirty bat.");
                    flags[1] = true;
                    return true;
                }
            }
            
            if b'g' == *option
            {
                println!("\nYou head back to the Hallway.");
                *room = RoomEnum::RoomHallway;
                return true;
            }
        },
        RoomEnum::RoomCloset =>
        {
            if b'l' == *option
            {
                if !flags[5]
                {
                    println!("\nThe shelves are full of cleaning chemicals, however you do find a long length of rope.");
                    flags[2] = true;
                }
                else
                {
                    println!("\nThere is nothing else here that you can use.");
                }
                return true;
            }
            else if b't' == *option
            {
                if flags[2] && !flags[5]
                {
                    println!("\nYou grab the rope.");
                    flags[5] = true;
                    return true;
                }
            }
            else if b'p' == *option
            {
                println!("\nThe cat lets out a shreek and bites you! That's gonna leave a mark.");
                return true;
            }
            else if b'g' == *option
            {
                println!("\nYou head back to the Hallway.");
                *room = RoomEnum::RoomHallway;
                return true;
            }
        },
        RoomEnum::RoomHobo =>
        {
            if !flags[3]
            {
                if b'a' == *option
                {
                    if flags[1]
                    {
                        println!("\nIn sheer terror, you swing the bat wildly, knocking the hobo to the ground.");
                        flags[3] = true;
                        return true;
                    }
                }
                else if b'r' == *option
                {
                    println!("\nYou turn and attempt to run away!");
                    *room = RoomEnum::RoomDiedHobo;
                    return true;
                }
            }
            else
            {
                if !flags[4]
                {
                    if b't' == *option
                    {
                        println!("\nYou find some keys.");
                        flags[4] = true;
                        return true;
                    }
                }
                if b'g' == *option
                {
                    println!("\nYou head back to the Hallway");
                    *room = RoomEnum::RoomHallway;
                    return true;
                }
            }
        },
        RoomEnum::RoomStairwell =>
        {
            if b'u' == *option
            {
                println!("\nYou start climbing the stairs, let's find the roof!");
                *room = RoomEnum::RoomRoof;
                return true;
            }
            else if b'r' == *option
            {
                println!("\nYou find some old porno mags and what looks like it might be drugs.");
                flags[6] = true;
                return true;
            }
            else if flags[6] && b't' == *option
            {
                println!("\nYou grab the drugs and pop them in your mouth. What the hell, you only live once...");
                *room = RoomEnum::RoomDiedDrugs;
                return true;
            }
            else if b'g' == *option
            {
                println!("\nMaybe it's better to head back");
                *room = RoomEnum::RoomHallway;
                return true;
            }
        },
        RoomEnum::RoomRoof =>
        {
            if b'c' == *option
            {
                if !flags[5]
                {
                    println!("\nYou lean over the edge and try to make your way down.");
                    *room = RoomEnum::RoomDiedFall;
                    return true;
                }
                else
                {
                    println!("\nYou lean over the edge and try to make your way down. Thankfully, you have this rope!");
                    *room = RoomEnum::RoomWin;
                    return true;
                }
            }
            else if b'g' == *option
            {
                println!("\nMaybe you'd rather look around some more and head back");
                *room = RoomEnum::RoomStairwell;
                return true;
            }
        }
        _ => (),
    }
    false
}

fn get_valid_input(mut room: &mut RoomEnum, mut flags: &mut[bool;10])
{
    println!("\nWhat do you do?");
    println!("\nOptions:\n{}", room_options(&room, &mut flags));

    loop
    {
        let option = get_player_input();
        if perform_action(&option, &mut room, &mut flags)
        {
            return;
        }
        println!("Try Again:");
    }
}

fn get_player_input() -> u8
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Expected input");
    
    input.as_bytes()[0]
}

fn room_options(room: &RoomEnum, flags: &[bool;10]) -> String
{
    match room
    {
        RoomEnum::RoomEntrance => String::from(" [g] go inside"),
        RoomEnum::RoomFoyer => String::from(" [c] check out hallway"),
        RoomEnum::RoomHallway => String::from(" [i] investigate sound\n [b] break into office\n [o] open unmarked door\n [s] stairwell door\n [g] go back"),
        RoomEnum::RoomOffice =>
        {
            let mut ret = String::new();
            if !flags[0]
            {
                ret = String::from(" [l] look in cabinet\n");
            }
            else if flags[0] && !flags[1]
            {
                ret = String::from(" [t] take bat\n");
            }
            ret.push_str(" [g] go back");
            return ret;
        }
        RoomEnum::RoomCloset =>
        {
            let mut ret = String::from(" [l] look through shelves\n");
            if flags[2] && !flags[5]
            {
                ret.push_str(" [t] take rope\n");
            }
            ret.push_str(" [p] pet the cat\n [g] go back");
            return ret;
        },
        RoomEnum::RoomHobo =>
        {
            let mut ret = String::new();
            if !flags[3]
            {
                if flags[1]
                {
                    ret = String::from(" [a] attack with bat\n");
                }
                ret.push_str(" [r] run away");
            }
            else
            {
                if !flags[4]
                {
                    ret = String::from(" [t] take the shiny object\n");
                }
                ret.push_str(" [g] go back");
            }
            return ret;
        },
        RoomEnum::RoomStairwell =>
        {
            let mut ret = String::from(" [u] up to the roof\n [r] rummage through trash\n");
            if flags[6]
            {
                ret.push_str(" [t] take drugs\n");
            }
            ret.push_str(" [g] go back");
            return ret;
        },
        RoomEnum::RoomRoof =>
        {
            String::from(" [c] climb down to friends\n [g] go back")
        },
        _ => String::new(),
    }
}

fn room_name(room: &RoomEnum) -> String
{
    match room
    {
        RoomEnum::RoomEntrance => String::from("Dirt Path"),
        RoomEnum::RoomFoyer => String::from("Foyer"),
        RoomEnum::RoomHallway => String::from("Hallway"),
        RoomEnum::RoomHobo => String::from("Dark Corner"),
        RoomEnum::RoomOffice => String::from("Office"),
        RoomEnum::RoomCloset => String::from("Closet"),
        RoomEnum::RoomStairwell => String::from("Stairwell"),
        RoomEnum::RoomRoof => String::from("Roof"),
        RoomEnum::RoomWin => String::from("YOU ESCAPE!"),
        RoomEnum::RoomDiedHobo => String::from("Game Over"),
        RoomEnum::RoomDiedDrugs => String::from("Game Over"),
        RoomEnum::RoomDiedFall => String::from("Game Over"),
    }
}

fn room_description(room: &RoomEnum, flags: &[bool;10]) -> String
{
    match room
    {
        RoomEnum::RoomEntrance => String::from("Wandering the night with your friends along an old dirt path, you come upon an abandoned building.\nOne of your friends says, \"I bet you won't go inside and check it out...\""),
        RoomEnum::RoomFoyer => String::from("It's dark, through the moonlight you can see a hallway at the end of the room."),
        RoomEnum::RoomHallway => String::from("Grungy, old broken tiles, moon light shines in, you see a door that says office and another that is unmarked.\nYou hear a faint groaning sound in the corner by the stairwell door."),
        RoomEnum::RoomHobo =>
        {
            if !flags[3]
            {
                String::from("Suddenly a deranged hobo jumps to his feet, thrusting a knife at you in an incoherent rage.")
            }
            else
            {
                let mut ret = String::from("The hobo lies on the ground unconscious.");
                if !flags[4]
                {
                    ret.push_str(" You see something glinting on the floor.");
                }
                ret
            }
        },
        RoomEnum::RoomOffice => String::from("You see papers everywhere, a desk, waiting bench, and an old rusty storage cabinet."),
        RoomEnum::RoomCloset => String::from("You see floor to ceiling shelves. The room smells of death, a cat howls at you from the top of the shelves."),
        RoomEnum::RoomStairwell => String::from("Blegh, you find trash everywhere with the smell of vomit and shit."),
        RoomEnum::RoomRoof => String::from("Thankful to see the moonlight and smell the fresh air. See your friends below."),
        RoomEnum::RoomWin => String::from("Happy to see your friends, you swear you'll never take a bet again."),
        RoomEnum::RoomDiedHobo => String::from("The hobo lunges at you with a knife, burrying it into your neck. As you lay on the ground, bleeding to death,\nyou wonder how this could have happened to someone like you."),
        RoomEnum::RoomDiedDrugs => String::from("Colors and sounds take hold of your body, the drugs take hold of your mind,\nyour body convulses as you accidentally overdose in incredible pain and die."),
        RoomEnum::RoomDiedFall => String::from("You attempt do jump down, you didn't realize you were so high up, trying to grab hold of something as you fall,\nyou smash your head on the ground and die."),
    }
}