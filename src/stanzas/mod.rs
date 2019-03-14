use xml::writer::XmlEvent;

fn parse(event: &XmlEvent) -> XMPPData {
    XMPPData::StreamOpen(StreamOpen {
        from: Option::None,
        id: Option::None,
        to: Option::None,
        version: Option::None,
    })
}

pub struct Entity {
    node: String,
    domain: String,
    resource: String,
}

pub enum XMPPData {
    StreamOpen(StreamOpen),
}

pub struct StreamOpen {
    from: Option<Entity>,
    id: Option<String>,
    to: Option<Entity>,
    version: Option<f32>,
}

pub struct Features {}
