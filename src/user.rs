use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    country: String,
    location: String,
    bio: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
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
    rd: u32,
    prog: u32,
    prov: bool,
}

impl Perf {
    fn games(&self) -> &u32 {
        &self.games
    }

    fn rating(&self) -> &u32 {
        &self.rating
    }

    fn rd(&self) -> &u32 {
        &self.rd
    }

    fn prog(&self) -> &u32 {
        &self.prog
    }

    fn prov(&self) -> &bool {
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
