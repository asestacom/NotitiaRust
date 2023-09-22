use crate::imdb::{Imdb};
use std::fs;
use serde::{Deserialize, Serialize};

static JUSTWATCH_EDGES: &[&str] = &[
    "files/justwatch/all.netflix.json",
    "files/justwatch/all.disney+.json",
    ];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TvPackage {
    pub packageId: i32, // 337
    pub clearName: String, // "Disney Plus"
    pub __typename: String // "Package"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalIds {
    pub imdbId: Option<String>, // "tt10234724"
    pub __typename: String // "ExternalIds"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scoring {
    pub imdbScore: Option<f32>, // 7.5
    pub __typename: String // "Scoring"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Genre {
    pub shortName: String, // "act"
    pub __typename: String // "Genre"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credit {
    pub role: String, // "ACTOR"
    pub name: String, // "Oscar Isaac"
    pub characterName: String, // "Marc Spector / Steven Grant / Moon Knight / Mr. Knight"
    pub personId: i32, // 3018
    pub __typename: String // "Credit"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NowOffer {
    pub id: String, // "b2Z8dHMyMTkwMDM6R0I6MzM3OmZsYXRyYXRlOjRr"
    pub standardWebURL: String, // "https://disneyplus.bn5x.net/c/1206980/705874/9358?u=https%3A%2F%2Fwww.disneyplus.com%2Fseries%2Fmoon-knight%2F4S3oOF1knocS&subId3=justappsvod"
    pub package: TvPackage,
    pub presentationType: String, // "_4K"
    pub monetizationType: String, // "FLATRATE"
    pub __typename: String // "Offer"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clip {
    pub externalId: String,
    pub __typename: String // "Clip"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub title: String,
    pub originalReleaseYear: i32,
    pub fullPath: String, // "/uk/tv-series/moon-knight"
    pub ageCertification: String, 
    pub productionCountries: Vec<String>,
    pub clips: Vec<Clip>,
    pub credits: Vec<Credit>,
    pub genres: Vec<Genre>,
    pub scoring: Scoring,
    pub externalIds: ExternalIds,
    pub shortDescription: String,
    pub runtime: i32, 
    pub posterUrl: Option<String>, // "/poster/267595463/{profile}/moon-knight.{format}"
    pub __typename: String // "ShowContent"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: String,
    pub objectId: i32,
    pub objectType: String, // "SHOW"
    pub content: Content,
    pub watchNowOffer: NowOffer,
    pub __typename: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub cursor: String,
    pub node: Node,
    pub __typename: String // "PopularTitlesEdge"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageInfo {
    pub startCursor: String,
    pub endCursor: String,
    pub hasPreviousPage: bool,
    pub hasNextPage: bool,
    pub __typename: String // "PageInfo"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Titles {
    pub totalCount: i32,
    pub pageInfo: PageInfo,
    pub edges: Vec<Edge>,
    pub __typename: String // "PopularTitlesConnection"
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JustWatchData {
    pub popularTitles: Titles
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JustWatchList {
    pub data: JustWatchData
}

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
pub struct Justwatch {
    pub id: String, // "id": "tm372685"

    pub edge: Option<Edge>,
}
impl Default for Justwatch {
    fn default() -> Justwatch {
        Justwatch {
            id: String::new(),

            edge: None,
        }
    }
}

pub fn load_justwatch_file(justwatch_fileName: &str) -> Result<Vec<Edge>, Box<dyn std::error::Error>> {
    let justwatch_data = fs::read_to_string(justwatch_fileName.to_string()).expect("Something went wrong reading the file");
    let justwatch_edges: Vec<Edge> = serde_json::from_str(&format!("{}", justwatch_data))?;
    
    Ok(justwatch_edges)
}

pub fn load_justwatch_files(justwatch_data_files: &[&str]) -> Result<Vec<Edge>, Box<dyn std::error::Error>> {
    let mut justwatch_edges: Vec<Edge> = vec!();
    for justwatch_fileName in justwatch_data_files {
        let justwatch_edges_temp = load_justwatch_file(justwatch_fileName)?;
        for idx in justwatch_edges_temp.into_iter() {
            justwatch_edges.push(idx);
        }
    }

    Ok(justwatch_edges)
}

pub fn load_justwatchs() -> Result<Vec<Edge>, Box<dyn std::error::Error>> {
    println!("Loading data for just watch...");
    load_justwatch_files(&JUSTWATCH_EDGES)
}

pub fn get_justwatchs(justwatch_edges: &Vec<Edge>, imdbs: &Vec<Imdb>, title: &String) -> Vec<Justwatch> {
    // TODO: to check this maybe we should use English version of the title, so if akas...
    println!("justwatch_edges...");
    let mut justwatchs: Vec<Justwatch> = Vec::<Justwatch>::new();
    for edge in justwatch_edges {
        if (edge.node.content.externalIds.imdbId.as_ref() != None && 
            imdbs.iter().find(|&item| &item.tconst == edge.node.content.externalIds.imdbId.as_ref().unwrap()) != None)
            || edge.node.content.title == *title
        {
            println!("{:?}", edge.node.id);
            
            let mut edge_copy = edge.clone();
            edge_copy.node.content.credits = Vec::new();
            justwatchs.push(Justwatch { 
                id: String::from(&edge.node.id),

                edge: Some(edge_copy),
            });
        }
    }
    justwatchs
}
