use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::StdRng;
use shared::{CatBreed, DogBreed, EntityType, FishBreed, HorseBreed, LizardBreed};

pub fn generate_pet_type_for_rank(rank: usize, rng: &mut StdRng) -> EntityType {
    // Luxury factor: higher rank = lower number = higher weight multiplier
    let luxury_multiplier = match rank {
        0 => 5, // CEO
        1 => 4,
        2 => 3,
        3 => 2,
        _ => 1, // Worker
    };

    // Define breed pools with base weights (1 = common, 2 = mid, 3 = luxurious)
    let cats = vec![
        (CatBreed::Tabby, 1),
        (CatBreed::Siamese, 2),
        (CatBreed::Persian, 2),
        (CatBreed::MaineCoon, 3),
        (CatBreed::Sphynx, 3),
        (CatBreed::ScottishFold, 2),
        (CatBreed::Bengal, 3),
        (CatBreed::Ragdoll, 3),
    ];

    let dogs = vec![
        (DogBreed::ShibaInu, 2),
        (DogBreed::LabradorRetriever, 1),
        (DogBreed::Poodle, 2),
        (DogBreed::Bulldog, 1),
        (DogBreed::GermanShepherd, 2),
        (DogBreed::Dachshund, 1),
        (DogBreed::GoldenRetriever, 2),
        (DogBreed::Chihuahua, 1),
    ];

    let horses = vec![
        (HorseBreed::Appaloosa, 2),
        (HorseBreed::Arabian, 3),
        (HorseBreed::Clydesdale, 3),
        (HorseBreed::Thoroughbred, 3),
        (HorseBreed::Mustang, 2),
        (HorseBreed::ShetlandPony, 1),
    ];

    let lizards = vec![
        (LizardBreed::BeardedDragon, 2),
        (LizardBreed::Gecko, 1),
        (LizardBreed::Iguana, 2),
        (LizardBreed::Chameleon, 3),
        (LizardBreed::Monitor, 3),
    ];

    let fish = vec![
        (FishBreed::GoldFish, 1),
        (FishBreed::Guppy, 1),
        (FishBreed::Betta, 2),
        (FishBreed::Angelfish, 2),
        (FishBreed::Tetra, 1),
        (FishBreed::Clownfish, 3),
    ];

    // Flatten all breeds into a single list of (EntityType, adjusted weight)
    let mut weighted_entities = vec![];

    for (breed, base_weight) in cats {
        weighted_entities.push((EntityType::Cat(breed), base_weight * luxury_multiplier));
    }
    for (breed, base_weight) in dogs {
        weighted_entities.push((EntityType::Dog(breed), base_weight * luxury_multiplier));
    }
    for (breed, base_weight) in horses {
        weighted_entities.push((EntityType::Horse(breed), base_weight * luxury_multiplier));
    }
    for (breed, base_weight) in lizards {
        weighted_entities.push((EntityType::Lizard(breed), base_weight * luxury_multiplier));
    }
    for (breed, base_weight) in fish {
        weighted_entities.push((EntityType::Fish(breed), base_weight * luxury_multiplier));
    }

    // Extract weights and sample
    let weights: Vec<_> = weighted_entities.iter().map(|(_, w)| *w).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    weighted_entities[dist.sample(rng)].0.clone()
}
