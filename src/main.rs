use std::process::{Command, Stdio};

fn cmd_stdout(cmd: &str, args: Vec<&str>) -> String {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let output_str = String::from_utf8(output.stdout).unwrap();
    return output_str;
}

fn main() {
    let card_string = cmd_stdout("pactl", vec!["list", "cards", "short"]);
    let card_list: Vec<&str> = card_string.split("\n").collect();
    for card in card_list {
        if card.contains("bluez_card") {
            let card_id = &card[0..2];
            Command::new("pactl")
                .args(vec!["set-card-profile", card_id, "a2dp-sink-sbc"])
                .output()
                .expect("Error");
        }
    }
}
