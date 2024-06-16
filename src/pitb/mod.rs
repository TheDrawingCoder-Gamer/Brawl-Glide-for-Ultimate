mod game;
mod effect;
mod sound;
mod frame;

pub fn install() {
    let mut agent = smashline::Agent::new("pitb");
    game::install(&mut agent);
    effect::install(&mut agent);
    sound::install(&mut agent);
    frame::install(&mut agent);
}
