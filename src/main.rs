use scraper::selectable::Selectable;
use std::error::Error;
use std::fs::File;

// Define a struct to hold country data
#[derive(Debug)]
struct Country {
    name: String,
    capital: String,
    population: String,
    area: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let response =
    reqwest::blocking::get("https://www.scrapethissite.com/pages/simple/")?;

    let html = response.text()?;

    // Parsing
    let document = scraper::Html::parse_document(&html);

    let html_country_info_box_selector = scraper::Selector::parse(".country")?;
    
    // Create selectors outside the loop for better performance
    let country_name_selector = scraper::Selector::parse(".country-name")?;
    let country_capital_selector = scraper::Selector::parse(".country-capital")?;
    let country_population_selector = scraper::Selector::parse(".country-population")?;
    let country_area_selector = scraper::Selector::parse(".country-area")?;

    // Create a vector to store all country data
    let mut countries = Vec::new();

    // Iterate through all country elements
    for country_element in document.select(&html_country_info_box_selector) {
        let name = country_element
            .select(&country_name_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country name not found!")?;

        let capital = country_element
            .select(&country_capital_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country capital not found!")?;

        let population = country_element
            .select(&country_population_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country population not found")?;

        let area = country_element
            .select(&country_area_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country area not found")?;

        // Store the country data
        countries.push(Country {
            name,
            capital,
            population,
            area,
        });

        // Still print to console for immediate feedback
        println!("Processed: {}", countries.last().unwrap().name);
    }

    // Create a new CSV file
    let file = File::create("countries.csv")?;
    let mut writer = csv::Writer::from_writer(file);

    // Write the header
    writer.write_record(&["Country", "Capital", "Population", "Area"])?;

    // Write all country data
    for country in countries {
        writer.write_record(&[
            &country.name,
            &country.capital,
            &country.population,
            &country.area,
        ])?;
    }

    // Flush the writer to ensure everything is written to the file
    writer.flush()?;

    println!("\nData has been exported to 'countries.csv' successfully!");

    Ok(())
}
