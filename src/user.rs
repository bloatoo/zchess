use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    country: Option<String>,
    location: Option<String>,
    bio: Option<String>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
}

impl Profile {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Perfs {
    blitz: Perf,
    bullet: Perf,
    rapid: Perf,
    correspondence: Perf,
    classical: Perf,
}

impl Perfs {
    fn blitz(&self) -> &Perf {
        &self.blitz
    }

    fn bullet(&self) -> &Perf {
        &self.bullet
    }

    fn rapid(&self) -> &Perf {
        &self.rapid
    }

    fn correspondence(&self) -> &Perf {
        &self.correspondence
    }

    fn classical(&self) -> &Perf {
        &self.classical
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Perf {
    games: u32,
    rating: u32,
    rd: i32,
    prog: i32,
    prov: Option<bool>,
}

impl Perf {
    fn games(&self) -> &u32 {
        &self.games
    }

    fn rating(&self) -> &u32 {
        &self.rating
    }

    fn rd(&self) -> &i32 {
        &self.rd
    }

    fn prog(&self) -> &i32 {
        &self.prog
    }

    fn prov(&self) -> &Option<bool> {
        &self.prov
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: String,
    username: String,
    online: bool,
    profile: Option<Profile>,
    perfs: Perfs,
}

impl User {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn online(&self) -> &bool {
        &self.online
    }
}
