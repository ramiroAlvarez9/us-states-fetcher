use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
struct StateApiResponse {
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_fips")]
    fips: String,
    #[serde(rename = "_abbrev")]
    abbrev: String,
    #[serde(flatten)]
    counties: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct CountyEntry {
    county: String,
    fips: String,
}

#[derive(Debug, Serialize)]
struct StateCounties {
    #[serde(rename = "uspsAbbreviation")]
    usps_abbreviation: String,
    #[serde(rename = "fips")]
    fips: String,
    #[serde(rename = "counties")]
    counties: Vec<CountyEntry>,
}

fn fetch_state_counties(client: &Client) -> Result<Vec<StateCounties>, Box<dyn Error>> {
    let response = client
        .get("https://api.fips.codes/index")
        .send()?
        .error_for_status()?;
    let states_response: HashMap<String, StateApiResponse> = response.json()?;

    let mut states: Vec<StateCounties> = states_response
        .values()
        .filter(|state_data| state_data.abbrev != "PR")
        .map(|state_data| {
            let mut counties: Vec<CountyEntry> = state_data
                .counties
                .iter()
                .map(|(county, fips)| CountyEntry {
                    county: county.clone(),
                    fips: fips.clone(),
                })
                .collect();
            counties.sort_by(|a, b| a.county.cmp(&b.county));

            StateCounties {
                usps_abbreviation: state_data.abbrev.clone(),
                fips: state_data.fips.clone(),
                counties,
            }
        })
        .collect();

    states.sort_by(|a, b| a.usps_abbreviation.cmp(&b.usps_abbreviation));

    Ok(states)
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().user_agent("rust-script/1.0").build()?;

    let results = fetch_state_counties(&client)?;

    let json_output = serde_json::to_string_pretty(&results)?;
    println!("{}", json_output);
    fs::write("counties.json", &json_output)?;

    Ok(())
}
