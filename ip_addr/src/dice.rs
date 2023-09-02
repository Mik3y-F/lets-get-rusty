pub fn dice_roll() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        6 => move_player(6),
        9 => reroll(),
        _ => println!("No effect!"),
    }

    let_no_match();
}

fn add_fancy_hat() {
    println!("Adding fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing fancy hat!");
}

fn move_player(spaces: i32) {
    println!("Moving player {} spaces!", spaces);
}

fn reroll() {
    println!("Rerolling!");
}

fn let_no_match() {
    let x: Option<i32> = None;
    let y = 10;

    if let Some(z) = x {
        println!("Matched, z = {:?}", z);
    } else {
        println!("Default case, x = {:?}", x);
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("config_max: {:?}", max);
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
