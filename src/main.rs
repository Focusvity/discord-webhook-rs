use json::object;

fn main() {
    println!("Nothing here yet!");
}

struct Webhook {
    url: String,
    content: String,
    username: String,
    avatar_url: String,
    tts: bool,
    embeds: Vec<EmbedObject>
}

impl Webhook {
    fn new(url: String) -> Webhook {
        Webhook {
            url,
            content: String::new(),
            username: String::new(),
            avatar_url: String::new(),
            tts: false,
            embeds: Vec::new()
        }
    }

    fn set_content(&mut self, content: String) {
        self.content = content;
    }

    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    fn set_avatar_url(&mut self, avatar_url: String) {
        self.avatar_url = avatar_url;
    }

    fn set_tts(&mut self, tts: bool) {
        self.tts = tts;
    }

    fn add_embed(&mut self, embed: EmbedObject) {
        self.embeds.push(embed);
    }

    fn execute(self) {
        assert!(!self.content.is_empty() && !self.embeds.is_empty());

        let mut embed_values: Vec<json::JsonValue> = Vec::new();
        if !self.embeds.is_empty() {
            for emb in self.embeds {
                let json_emb = object!{
                    title: emb.title,
                    description: emb.description,
                    url: emb.url
                };
            }
        }

        let json_obj = object!{
            content: self.content,
            username: self.username,
            avatar_url: self.avatar_url,
            tts: self.tts,
        };
    }
}

struct EmbedObject {
    title: String,
    description: String,
    url: String,
    color: Color,
    footer: Footer,
    thumbnail: Thumbnail,
    image: Image,
    author: Author,
    fields: Vec<Field>
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

struct Footer {
    text: String,
    icon_url: String
}

struct Thumbnail {
    url: String
}

struct Image {
    url: String
}

struct Author {
    name: String,
    url: String,
    icon_url: String
}

struct Field {
    name: String,
    value: String,
    inline: bool
}
