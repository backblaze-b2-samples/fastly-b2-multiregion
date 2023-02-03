use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Default edge server code - used when running on the local test server
pub(crate) const DEFAULT_POP: &str = "SJC";

pub struct Origin {
    /// This should match the name of a storage backend. See the the `Hosts`
    /// section of the Fastly WASM service UI for more information.
    pub backend_name: &'static str,
    /// The name of the bucket to serve content from.
    pub bucket_name: &'static str,
    /// The host that the bucket is served on. This is used to make requests to the backend.
    pub bucket_host: &'static str,
}

/// Details of the origins. You must edit the bucket_names and bucket_hosts. Do not change
/// the backend_name.
pub(crate) const EU_CENTRAL: Origin = Origin {
    backend_name: "eu_central",
    bucket_name: "YOUR-EU-CENTRAL-BUCKET",
    bucket_host: "YOUR-EU-CENTRAL-ENDPOINT",
};

pub(crate) const US_EAST: Origin = Origin {
    backend_name: "us_east",
    bucket_name: "YOUR-US-EAST-BUCKET",
    bucket_host: "YOUR-US-EAST-ENDPOINT",
};

pub(crate) const US_WEST: Origin = Origin {
    backend_name: "us_west",
    bucket_name: "YOUR-US-WEST-BUCKET",
    bucket_host: "YOUR-US-WEST-ENDPOINT",
};

lazy_static! {
    /// Regex for extracting region from endpoint
    pub(crate) static ref REGION_REGEX: Regex = Regex::new(r"^s3\.([[:alnum:]\-]+)\.backblazeb2\.com$").unwrap();
}

// If auth is required, configure your access keys in an edge dictionary named "bucket_auth":
// <backend_name>_access_key_id
// <backend_name>_secret_access_key

lazy_static! {
    /// Mapping from POP to ordered list of origins
    pub(crate) static ref POP_ORIGIN: HashMap<&'static str, [Origin; 3]> = HashMap::from([
        ("ACC", [EU_CENTRAL, US_EAST, US_WEST]),
        ("ADL", [US_WEST, US_EAST, EU_CENTRAL]),
        ("AKL", [US_WEST, US_EAST, EU_CENTRAL]),
        ("AMS", [EU_CENTRAL, US_EAST, US_WEST]),
        ("ATL", [US_EAST, US_WEST, EU_CENTRAL]),
        ("BFI", [US_WEST, US_EAST, EU_CENTRAL]),
        ("BKK", [US_WEST, US_EAST, EU_CENTRAL]),
        ("BMA", [EU_CENTRAL, US_EAST, US_WEST]),
        ("BNE", [US_WEST, US_EAST, EU_CENTRAL]),
        ("BOG", [US_WEST, US_EAST, EU_CENTRAL]),
        ("BOM", [EU_CENTRAL, US_EAST, US_WEST]),
        ("BOS", [US_EAST, US_WEST, EU_CENTRAL]),
        ("BRU", [EU_CENTRAL, US_EAST, US_WEST]),
        ("BUR", [US_WEST, US_EAST, EU_CENTRAL]),
        ("CCU", [EU_CENTRAL, US_WEST, US_EAST]),
        ("CDG", [EU_CENTRAL, US_EAST, US_WEST]),
        ("CGH", [US_EAST, US_WEST, EU_CENTRAL]),
        ("CHI", [US_WEST, US_EAST, EU_CENTRAL]),
        ("CMH", [US_EAST, US_WEST, EU_CENTRAL]),
        ("CPH", [EU_CENTRAL, US_EAST, US_WEST]),
        ("CPT", [EU_CENTRAL, US_EAST, US_WEST]),
        ("CWB", [US_EAST, US_WEST, EU_CENTRAL]),
        ("DAL", [US_WEST, US_EAST, EU_CENTRAL]),
        ("DEL", [EU_CENTRAL, US_WEST, US_EAST]),
        ("DEN", [US_WEST, US_EAST, EU_CENTRAL]),
        ("DFW", [US_WEST, US_EAST, EU_CENTRAL]),
        ("DTW", [US_EAST, US_WEST, EU_CENTRAL]),
        ("DUB", [EU_CENTRAL, US_EAST, US_WEST]),
        ("DXB", [EU_CENTRAL, US_EAST, US_WEST]),
        ("EWR", [US_EAST, US_WEST, EU_CENTRAL]),
        ("EZE", [US_WEST, US_EAST, EU_CENTRAL]),
        ("FCO", [EU_CENTRAL, US_EAST, US_WEST]),
        ("FJR", [EU_CENTRAL, US_EAST, US_WEST]),
        ("FOR", [US_WEST, US_EAST, EU_CENTRAL]),
        ("FRA", [EU_CENTRAL, US_EAST, US_WEST]),
        ("FTY", [US_WEST, US_EAST, EU_CENTRAL]),
        ("GIG", [US_EAST, US_WEST, EU_CENTRAL]),
        ("GNV", [US_WEST, US_EAST, EU_CENTRAL]),
        ("HEL", [EU_CENTRAL, US_EAST, US_WEST]),
        ("HHN", [EU_CENTRAL, US_EAST, US_WEST]),
        ("HKG", [US_WEST, EU_CENTRAL, US_EAST]),
        ("HND", [US_WEST, EU_CENTRAL, US_EAST]),
        ("HNL", [US_WEST, US_EAST, EU_CENTRAL]),
        ("HYD", [EU_CENTRAL, US_WEST, US_EAST]),
        ("IAD", [US_EAST, US_WEST, EU_CENTRAL]),
        ("IAH", [US_WEST, US_EAST, EU_CENTRAL]),
        ("ICN", [US_WEST, US_EAST, EU_CENTRAL]),
        ("ITM", [US_WEST, US_EAST, EU_CENTRAL]),
        ("JNB", [EU_CENTRAL, US_EAST, US_WEST]),
        ("KUL", [EU_CENTRAL, US_WEST, US_EAST]),
        ("LCY", [EU_CENTRAL, US_EAST, US_WEST]),
        ("LGA", [US_EAST, US_WEST, EU_CENTRAL]),
        ("LGB", [US_WEST, US_EAST, EU_CENTRAL]),
        ("LHR", [EU_CENTRAL, US_EAST, US_WEST]),
        ("LIM", [US_EAST, US_WEST, EU_CENTRAL]),
        ("LIN", [EU_CENTRAL, US_EAST, US_WEST]),
        ("LIS", [EU_CENTRAL, US_EAST, US_WEST]),
        ("LON", [EU_CENTRAL, US_EAST, US_WEST]),
        ("MAA", [EU_CENTRAL, US_WEST, US_EAST]),
        ("MAD", [EU_CENTRAL, US_EAST, US_WEST]),
        ("MAN", [EU_CENTRAL, US_EAST, US_WEST]),
        ("MCI", [US_WEST, US_EAST, EU_CENTRAL]),
        ("MEL", [US_WEST, US_EAST, EU_CENTRAL]),
        ("MIA", [US_WEST, US_EAST, EU_CENTRAL]),
        ("MNL", [US_WEST, EU_CENTRAL, US_EAST]),
        ("MRS", [EU_CENTRAL, US_EAST, US_WEST]),
        ("MSP", [US_EAST, US_WEST, EU_CENTRAL]),
        ("MUC", [EU_CENTRAL, US_EAST, US_WEST]),
        ("MXP", [EU_CENTRAL, US_EAST, US_WEST]),
        ("NRT", [US_WEST, US_EAST, EU_CENTRAL]),
        ("OSL", [EU_CENTRAL, US_EAST, US_WEST]),
        ("PAO", [US_WEST, US_EAST, EU_CENTRAL]),
        ("PDK", [US_EAST, US_WEST, EU_CENTRAL]),
        ("PDX", [US_WEST, US_EAST, EU_CENTRAL]),
        ("PER", [US_WEST, US_EAST, EU_CENTRAL]),
        ("PHX", [US_WEST, US_EAST, EU_CENTRAL]),
        ("PMO", [EU_CENTRAL, US_EAST, US_WEST]),
        ("QPG", [EU_CENTRAL, US_WEST, US_EAST]),
        ("SCL", [US_EAST, US_WEST, EU_CENTRAL]),
        ("SJC", [US_WEST, US_EAST, EU_CENTRAL]),
        ("SOF", [EU_CENTRAL, US_EAST, US_WEST]),
        ("STL", [US_WEST, US_EAST, EU_CENTRAL]),
        ("SYD", [US_WEST, US_EAST, EU_CENTRAL]),
        ("TYO", [US_WEST, US_EAST, EU_CENTRAL]),
        ("VIE", [EU_CENTRAL, US_EAST, US_WEST]),
        ("WLG", [US_WEST, US_EAST, EU_CENTRAL]),
        ("YUL", [US_EAST, US_WEST, EU_CENTRAL]),
        ("YVR", [US_WEST, US_EAST, EU_CENTRAL]),
        ("YYC", [US_WEST, US_EAST, EU_CENTRAL]),
        ("YYZ", [US_EAST, US_WEST, EU_CENTRAL]),
    ]);
}
