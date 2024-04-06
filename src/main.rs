// Import necessary modules from the reqwest and scraper crates.
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

// Define a custom error type for the program using thiserror crate.
#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error), // Error for handling reqwest-related issues.
    #[error("bad http response: {0}")]
    BadResponse(String), // Error for non-successful HTTP responses.
}

// A struct to represent a crawl command with a URL and a flag to extract links.
#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

// Function to visit a webpage and optionally extract all links from it.
fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    // Print the URL being visited.
    println!("Checking {:#}", command.url);
    // Perform an HTTP GET request on the URL.
    let response = client.get(command.url.clone()).send()?;
    // Check for HTTP success status, return error if not successful.
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    // Vector to store found URLs.
    let mut link_urls = Vec::new();
    // If not extracting links, return the empty vector.
    if !command.extract_links {
        return Ok(link_urls);
    }

    // Parse the body of the response to extract links.
    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    // Create a selector to find 'a' elements (hyperlinks).
    let selector = Selector::parse("a").unwrap();
    // Select all 'a' elements and extract the 'href' attributes.
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        // Attempt to resolve the href to a full URL.
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url); // Add valid URL to the list.
            }
            Err(err) => {
                // Print an error message for any href that can't be parsed into a URL.
                println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
            }
        }
    }
    // Return the list of found URLs.
    Ok(link_urls)
}

// The main function to run the crawler.
fn main() {
    // Create a new client instance for making HTTP requests.
    let client = Client::new();
    // Parse the starting URL.
    let start_url = Url::parse("https://www.google.org").unwrap();
    // Create a crawl command with the URL and flag to extract links.
    let crawl_command = CrawlCommand {
        url: start_url,
        extract_links: true,
    };

    // Visit the page and handle the result.
    match visit_page(&client, &crawl_command) {
        Ok(links) => println!("Links: {links:#?}"), // Print out all found links.
        Err(err) => println!("Could not extract links: {err:#}"), // Print out an error if occurred.
    }
}
