use rand::Rng;
use rand::prelude::{SliceRandom, StdRng};
use shared::EntityType;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum PetKind {
    Cat,
    Dog,
    Horse,
    Lizard,
    Fish,
    Other,
}

pub fn pet_kind_from_entity(entity_type: &EntityType) -> PetKind {
    match entity_type {
        EntityType::Cat(_) => PetKind::Cat,
        EntityType::Dog(_) => PetKind::Dog,
        EntityType::Horse(_) => PetKind::Horse,
        EntityType::Lizard(_) => PetKind::Lizard,
        EntityType::Fish(_) => PetKind::Fish,
        _ => PetKind::Other,
    }
}

fn pet_name_prefixes(kind: PetKind) -> Vec<&'static str> {
    match kind {
        PetKind::Dog => vec!["Mr.", "Sir", "Captain", "Dr.", "Chief", "Baron"],
        PetKind::Cat => vec!["Miss", "Lady", "Duchess", "Queen", "Princess", "Madam"],
        PetKind::Horse => vec!["Lord", "Sir", "Count", "Baron", "Duke", "Majesty"],
        PetKind::Fish => vec!["Captain", "Admiral", "Sir", "Lord"],
        PetKind::Lizard => vec!["Sir", "Master", "Count", "Duke"],
        PetKind::Other => vec!["Mr.", "Ms.", "Mx.", "Dr."],
    }
}

fn pet_name_suffixes(kind: PetKind) -> Vec<&'static str> {
    match kind {
        PetKind::Dog => vec!["the Brave", "the Barky", "the Bold", "of the Yard"],
        PetKind::Cat => vec!["the Fluffy", "the Sneaky", "the Regal", "of the Night"],
        PetKind::Horse => vec!["the Swift", "the Strong", "the Sturdy", "of the Plains"],
        PetKind::Fish => vec!["the Swift", "the Silent", "the Glimmering"],
        PetKind::Lizard => vec!["the Scaly", "the Quick", "the Ancient"],
        PetKind::Other => vec!["the Curious", "the Mysterious"],
    }
}

fn all_pet_names(kind: PetKind) -> Vec<String> {
    match kind {
        PetKind::Dog => vec![
            "Rover".to_string(),
            "Barkley".to_string(),
            "Fido".to_string(),
            "Maximus".to_string(),
            "Waffles".to_string(),
            "Snickers".to_string(),
            "Scout".to_string(),
            "Captain".to_string(),
            "Pickles".to_string(),
            "Gizmo".to_string(),
            "Boomer".to_string(),
            "Ziggy".to_string(),
            "Otis".to_string(),
            "Mochi".to_string(),
            "Turbo".to_string(),
            "Nugget".to_string(),
            "Banjo".to_string(),
            "Tater".to_string(),
            "Churro".to_string(),
            "Diesel".to_string(),
            "Yapper".to_string(),
            "Biscuit".to_string(),
            "Freckles".to_string(),
            "Jasper".to_string(),
            "Cosmo".to_string(),
            "Sprinkles".to_string(),
            "Beefy".to_string(),
            "Marbles".to_string(),
            "Bongo".to_string(),
            "Tugboat".to_string(),
            "Muzzle".to_string(),
            "Dogtor".to_string(),
            "Toby".to_string(),
            "Dingo".to_string(),
            "Sparky".to_string(),
            "Buttons".to_string(),
            "Chomp".to_string(),
            "Bark Twain".to_string(),
            "Goober".to_string(),
            "Cheddar".to_string(),
        ],
        PetKind::Cat => vec![
            "Whiskers".to_string(),
            "Mittens".to_string(),
            "Luna".to_string(),
            "Purrcy".to_string(),
            "Sassy".to_string(),
            "Cleo".to_string(),
            "Binx".to_string(),
            "Snugglepaws".to_string(),
            "Velvet".to_string(),
            "Nimbus".to_string(),
            "Jinx".to_string(),
            "Marble".to_string(),
            "Tinker".to_string(),
            "Salem".to_string(),
            "Pebbles".to_string(),
            "Sprout".to_string(),
            "Zuzu".to_string(),
            "Socks".to_string(),
            "Cricket".to_string(),
            "Tofu".to_string(),
            "Shadow".to_string(),
            "Clawdia".to_string(),
            "Chairman Meow".to_string(),
            "Miso".to_string(),
            "Velcro".to_string(),
            "Flufferton".to_string(),
            "Onyx".to_string(),
            "Stormy".to_string(),
            "Jazzpaws".to_string(),
            "Yowza".to_string(),
            "Napkin".to_string(),
            "Gato".to_string(),
            "Espresso".to_string(),
            "Static".to_string(),
            "Whimsy".to_string(),
            "Tabasco".to_string(),
            "Nebula".to_string(),
            "Fuzz".to_string(),
            "Tinsel".to_string(),
            "Catastrophe".to_string(),
        ],
        PetKind::Horse => vec![
            "Comet".to_string(),
            "Starlight".to_string(),
            "Thunder".to_string(),
            "Maple".to_string(),
            "Zephyr".to_string(),
            "Blaze".to_string(),
            "Dakota".to_string(),
            "Aurora".to_string(),
            "Nimbus".to_string(),
            "Echo".to_string(),
            "Whinny".to_string(),
            "Sable".to_string(),
            "Apollo".to_string(),
            "Chestnut".to_string(),
            "Clover".to_string(),
            "Flicka".to_string(),
            "Indigo".to_string(),
            "Rustler".to_string(),
            "Galaxy".to_string(),
            "Storm".to_string(),
            "Dustmane".to_string(),
            "Majesty".to_string(),
            "Pinecone".to_string(),
            "Willow".to_string(),
            "Copper".to_string(),
            "Cricket".to_string(),
            "Lightning".to_string(),
            "River".to_string(),
            "Orion".to_string(),
            "Velvet Hoof".to_string(),
            "Sunburst".to_string(),
            "Wander".to_string(),
            "Sprinter".to_string(),
            "Bard".to_string(),
            "Chime".to_string(),
            "Mango".to_string(),
            "Solstice".to_string(),
            "Meadow".to_string(),
            "Myst".to_string(),
            "Comanche".to_string(),
        ],
        PetKind::Fish => vec![
            "Bubbles".to_string(),
            "Finley".to_string(),
            "Coral".to_string(),
            "Splash".to_string(),
            "Neptune".to_string(),
            "Nemo".to_string(),
            "Gilligan".to_string(),
            "Drift".to_string(),
            "Reef".to_string(),
            "Zappy".to_string(),
            "Scuba".to_string(),
            "Squirt".to_string(),
            "Swishy".to_string(),
            "Tide".to_string(),
            "Jellybean".to_string(),
            "Marlin".to_string(),
            "Kelp".to_string(),
            "Flipper".to_string(),
            "Ripple".to_string(),
            "Sonar".to_string(),
            "Guppy".to_string(),
            "Goldie".to_string(),
            "Salty".to_string(),
            "Floaty".to_string(),
            "Speckle".to_string(),
            "Wiggle".to_string(),
            "Barnacle".to_string(),
            "Inky".to_string(),
            "Seaweed".to_string(),
            "Toona".to_string(),
            "Eelvis".to_string(),
            "Blinky".to_string(),
            "Floater".to_string(),
            "Bloop".to_string(),
            "Turbofin".to_string(),
            "Sushimi".to_string(),
            "Gillbert".to_string(),
            "Watson".to_string(),
            "Marina".to_string(),
            "Orko".to_string(),
        ],
        PetKind::Lizard => vec![
            "Scales".to_string(),
            "Zilla".to_string(),
            "Slink".to_string(),
            "Pebble".to_string(),
            "Rango".to_string(),
            "Igor".to_string(),
            "Cactus".to_string(),
            "Toothless".to_string(),
            "Dusty".to_string(),
            "Slinky".to_string(),
            "Spike".to_string(),
            "Echo".to_string(),
            "Molty".to_string(),
            "Claws".to_string(),
            "Rex".to_string(),
            "Tango".to_string(),
            "Napoleon".to_string(),
            "Leafy".to_string(),
            "Gecko".to_string(),
            "Crispy".to_string(),
            "Charbroil".to_string(),
            "Scorch".to_string(),
            "Flicker".to_string(),
            "Crunch".to_string(),
            "Salamando".to_string(),
            "Wartson".to_string(),
            "Dart".to_string(),
            "Hiss".to_string(),
            "Dino".to_string(),
            "Ember".to_string(),
            "Grimey".to_string(),
            "Grub".to_string(),
            "Lash".to_string(),
            "Newton".to_string(),
            "Pebblor".to_string(),
            "Kaa".to_string(),
            "Wrangle".to_string(),
            "Sunsoak".to_string(),
            "Rusty".to_string(),
            "Zip".to_string(),
        ],
        PetKind::Other => vec![
            "Thingy".to_string(),
            "Blob".to_string(),
            "Mooch".to_string(),
            "Fizz".to_string(),
            "Noodle".to_string(),
            "Wiggles".to_string(),
            "Blorbo".to_string(),
            "Sprank".to_string(),
            "Chonk".to_string(),
            "Sploot".to_string(),
            "Orbit".to_string(),
            "Zorp".to_string(),
            "Oob".to_string(),
            "Mib".to_string(),
            "Glob".to_string(),
            "Crumb".to_string(),
            "Zazu".to_string(),
            "Taco".to_string(),
            "Churro".to_string(),
            "Blip".to_string(),
            "Snargle".to_string(),
            "Pib".to_string(),
            "Yomp".to_string(),
            "Fizzgig".to_string(),
            "Momo".to_string(),
            "Snickerdoodle".to_string(),
            "Brumble".to_string(),
            "Tiblet".to_string(),
            "Glim".to_string(),
            "Jorb".to_string(),
            "Flarn".to_string(),
            "Zot".to_string(),
            "Bibble".to_string(),
            "Gronk".to_string(),
            "Bloopie".to_string(),
            "Flibber".to_string(),
            "Queek".to_string(),
            "Smidge".to_string(),
            "Twerp".to_string(),
            "Zumble".to_string(),
        ],
    }
}

pub fn build_all_pet_name_pools(rng: &mut StdRng) -> HashMap<PetKind, VecDeque<String>> {
    let mut pools = HashMap::new();

    for kind in [
        PetKind::Dog,
        PetKind::Cat,
        PetKind::Horse,
        PetKind::Fish,
        PetKind::Lizard,
        PetKind::Other,
    ] {
        let mut names = all_pet_names(kind);
        names.shuffle(rng);
        pools.insert(kind, VecDeque::from(names));
    }

    pools
}

pub fn generate_unique_pet_name(
    entity_type: &EntityType,
    pools: &mut HashMap<PetKind, VecDeque<String>>,
    rng: &mut StdRng,
) -> Option<String> {
    let kind = pet_kind_from_entity(entity_type);
    let base_name = pools.get_mut(&kind)?.pop_front()?;

    // 30% chance to add prefix or suffix
    let add_decoration = rng.gen_bool(0.3);

    if add_decoration {
        let add_prefix = rng.gen_bool(0.5);
        if add_prefix {
            let prefixes = pet_name_prefixes(kind);
            if let Some(prefix) = prefixes.choose(rng) {
                return Some(format!("{} {}", prefix, base_name));
            }
        } else {
            let suffixes = pet_name_suffixes(kind);
            if let Some(suffix) = suffixes.choose(rng) {
                return Some(format!("{} {}", base_name, suffix));
            }
        }
    }

    Some(base_name)
}
