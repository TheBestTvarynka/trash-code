pub struct PassCreds {
    pub username: String,
    pub password: String,
}

pub struct SmartCardCreds {
    pub pin: String,
    pub card_name: String,
}

pub struct OauthCreds {
    pub username: String,
    pub token: String,
}

pub enum LogonCreds {
    Password(PassCreds),
    SmartCard(SmartCardCreds),
    Oauth(OauthCreds),
}

fn process_auth(_log_on: LogonCreds) {
    unimplemented!("homework")
}

struct User {
    // todo!()
}

enum Room {
    Created {
        room_id: u32,
    },
    Started {
        room_id: u32,
        users: Vec<User>,
    },
    Finished {
        room_id: u32,
        users: Vec<User>,
        winner: User,
    }
}
