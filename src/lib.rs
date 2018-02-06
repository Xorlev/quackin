//! # Quackin - API documentation
//!
//! Hi, welcome to the Quackin API documentation!
//!
//! Quackin is a recommender systems framework written in Rust focused on:
//!
//! - Facilitate data handling.
//! - Providing collaborative filtering algorithms.
//! - Being an environment to build, test and evaluate new algorithms.
//!
//! Until we get a website for the crate, this docs will be the primary
//! reference for both technical and user-side aspects of Quackin.
//!
//! Quackin was called Oozie as a reference to Apache Mahout, but then I
//! discovered that Apache Oozie already exists. Now is called Quackin because
//! I wanted a funny name and my debugger rubber duck was on my desk at that
//! moment.
//!
//! ## Usage
//! To start using Quackin just add it as a dependency to your `Cargo.toml` file:
//!
//! ```ignore
//! [dependencies]
//! quackin = "0.1.2"
//! ```
//!
//! ## Getting started
//! We will write a simple movie recommender engine using Quackin over one of
//! the movielens dataset. Download `ml-latest-small.zip` from
//! [here](https://grouplens.org/datasets/movielens/), and then extract the
//! `ratings.csv` file.
//!
//! ### Loading data into Quackin
//! First we need to load the movielens dataset into Rust. This dataset has the
//! following columns: `userId`, `movieId`, `rating` and `timestamp`. We must
//! tell quackin that this file has this columns in this specific order. Also,
//! we need to specify if this file has headers or not and which delimiter is
//! separating the values`.
//!
//! ```ignore
//! use quackin::data::{Record, ReadOptions, read_custom_records};
//! quackin::data::Field::*;
//!
//! let options = ReadOptions::custom(vec![UserID, ItemID, Rating, Other], true, ',').unwrap();
//! //                                                             ^^^^^   ^^^^  ^^^
//! //                                                             |       |     |
//! //                             we don't care about the timestamp.      |     |
//! //                                                 this file has headers.    |
//! //                                                    use comma as a delimiter.
//! ```
//! Whew, thats a lot of boilerplate for just loading the dataset. But this was
//! the hardest part of the process. Now lets load the dataset:
//!
//! ```ignore
//! let records = read_custom_records("/path/to/movielens", options);
//! ```
//! Thats it! now let's build a recommender
//!
//! ### Creating a recommender
//! We will use a traditional K-nearest neighbors algorithm to build a basic
//! recommender. For each user we will take the 50 nearest users to him,
//! using a cosine similarity
//!
//! ```ignore
//! use quackin::recommender::{KnnUserRecommender, Recommender};
//! use quackin::metrics::similarity::cosine;
//!
//! let recommender = KnnUserRecommender::from_records(&records, cosine, 50);
//! ```
//! Now we can ask predictions from the recommender. For example, the user with
//! user ID `1` gave a rating of 4 to the classic science fiction movie "Tron".
//! What would our recommender predict about this?
//!
//! ```ignore
//! println!("{:?}", recommender.predict("1", "2105"));
//! // Ok(3.504942020280084)
//! ```
//! Not bad! we aren't that far from the real rating. Now you can start recommending
//! movies to everyone!
extern crate csv;
extern crate sprs;

pub mod data;
pub mod recommender;
pub mod metrics;

pub fn build_quackin(
    pbf_path: String,
    with_geom: bool,
    libpostal_file_path: PathBuf,
    country_code: Option<String>,
) -> Result<Cosmogony, Error> {
    let path = Path::new(&pbf_path);
    let file = File::open(&path).context("no pbf file")?;

    let mut parsed_pbf = OsmPbfReader::new(file);

    let (mut zones, mut stats) = if with_geom {
        get_zones_and_stats(&mut parsed_pbf)?
    } else {
        get_zones_and_stats_without_geom(&mut parsed_pbf)?
    };

    create_ontology(&mut zones, &mut stats, libpostal_file_path, country_code)?;

    let cosmogony = Cosmogony {
        zones: zones,
        meta: CosmogonyMetadata {
            osm_filename: path.file_name()
                .and_then(|f| f.to_str())
                .map(|f| f.to_string())
                .unwrap_or("invalid file name".into()),
            stats: stats,
        },
    };
    Ok(cosmogony)
}