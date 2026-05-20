use mpris::PlayerFinder;

fn main() {
    let finder = PlayerFinder::new().unwrap();
    let players = finder.find_all().unwrap();
    for p in players {
        println!("{} - {} - {}", p.identity(), p.bus_name(), p.unique_name());
    }
}
