use crate::types::{chat::ChannelChat, message::*, user::User};

use serde_json::json;

#[test]
fn test_deserialize_message_channel() {
    let input = json!({
        "message_id": 1,
        "date": 0,
        "chat": {
            "id": 1,
            "type": "channel",
            "title": "channeltitle"
        },
        "text": "test"
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    assert_eq!(msg.id, 1);
    assert_eq!(msg.date, 0);
    assert_eq!(msg.get_chat_id(), 1);
    assert!(msg.get_user().is_none());
    assert!(!msg.is_edited());
    if let MessageKind::Channel {
        chat: ChannelChat { id, title, .. },
        author_signature,
    } = msg.kind
    {
        assert_eq!(id, 1);
        assert_eq!(title, "channeltitle");
        assert_eq!(author_signature, None);
    } else {
        panic!("Unexpected message kind: {:?}", msg.kind);
    }
    if let MessageData::Text(Text { data, entities }) = msg.data {
        assert_eq!(data, "test");
        assert!(entities.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn test_deserialize_message_group() {
    let input = json!({
        "message_id": 1,
        "date": 0,
        "from": {
            "id": 1,
            "first_name": "firstname",
            "is_bot": false
        },
        "chat": {
            "id": 1,
            "type": "group",
            "title": "grouptitle",
            "all_members_are_administrators": true
        },
        "text": "test",
        "edit_date": 1
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    assert_eq!(msg.id, 1);
    assert_eq!(msg.date, 0);
    assert_eq!(msg.get_chat_id(), 1);
    assert_eq!(msg.get_user().map(|u| u.id), Some(1));
    assert_eq!(msg.get_text().map(|t| t.data.as_str()), Some("test"));
    assert!(msg.is_edited());
    if let MessageKind::Group { chat, from } = msg.kind {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "grouptitle");
        assert_eq!(chat.all_members_are_administrators, true);
        assert_eq!(from.id, 1);
        assert_eq!(from.first_name, "firstname");
        assert_eq!(from.is_bot, false);
    } else {
        panic!("Unexpected message kind: {:?}", msg.kind);
    }
    if let MessageData::Text(Text { data, entities }) = msg.data {
        assert_eq!(data, "test");
        assert!(entities.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }

    let input = json!({
        "message_id": 1, "date": 0, "text": "test",
        "chat": {"id": 1, "type": "group", "title": "grouptitle", "all_members_are_administrators": true}
    });
    let err = serde_json::from_value::<Message>(input).unwrap_err();
    assert_eq!(err.to_string(), String::from("\"from\" field is missing"));
}

#[test]
fn test_deserialize_message_private() {
    let input = json!({
        "message_id": 1,
        "date": 0,
        "from": {
            "id": 1,
            "first_name": "firstname",
            "is_bot": false
        },
        "chat": {
            "id": 1,
            "type": "private",
            "first_name": "firstname"
        },
        "text": "test"
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    assert_eq!(msg.id, 1);
    assert_eq!(msg.date, 0);
    assert_eq!(msg.get_chat_id(), 1);
    assert_eq!(msg.get_user().map(|u| u.id), Some(1));
    assert_eq!(msg.get_text().map(|t| t.data.as_str()), Some("test"));
    if let MessageKind::Private { chat, from } = msg.kind {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.first_name, "firstname");
        assert_eq!(from.id, 1);
        assert_eq!(from.first_name, "firstname");
        assert_eq!(from.is_bot, false);
    } else {
        panic!("Unexpected message kind: {:?}", msg.kind);
    }
    if let MessageData::Text(Text { data, entities }) = msg.data {
        assert_eq!(data, "test");
        assert!(entities.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }

    let input = json!({
        "message_id": 1, "date": 0, "text": "test",
        "chat": {"id": 1, "type": "private", "first_name": "firstname"}
    });
    let err = serde_json::from_value::<Message>(input).unwrap_err();
    assert_eq!(err.to_string(), String::from("\"from\" field is missing"));
}

#[test]
fn test_deserialize_message_supergroup() {
    let input = json!({
        "message_id": 1,
        "date": 0,
        "from": {
            "id": 1,
            "first_name": "firstname",
            "is_bot": false
        },
        "chat": {
            "id": 1,
            "type": "supergroup",
            "title": "supergrouptitle",
            "username": "supergroupusername"
        },
        "text": "test"
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    assert_eq!(msg.id, 1);
    assert_eq!(msg.date, 0);
    assert_eq!(msg.get_chat_id(), 1);
    assert_eq!(msg.get_chat_username().unwrap(), "supergroupusername");
    assert_eq!(msg.get_user().map(|u| u.id), Some(1));
    assert_eq!(msg.get_text().map(|t| t.data.as_str()), Some("test"));
    if let MessageKind::Supergroup { chat, from } = msg.kind {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, "supergrouptitle");
        assert_eq!(from.id, 1);
        assert_eq!(from.first_name, "firstname");
        assert_eq!(from.is_bot, false);
    } else {
        panic!("Unexpected message kind: {:?}", msg.kind);
    }
    if let MessageData::Text(Text { data, entities }) = msg.data {
        assert_eq!(data, "test");
        assert!(entities.is_none());
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }

    let input = json!({
        "message_id": 1, "date": 0,
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test"
    });
    let err = serde_json::from_value::<Message>(input).unwrap_err();
    assert_eq!(err.to_string(), String::from("\"from\" field is missing"));
}

#[test]
fn test_deserialize_message_forward() {
    let input = json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "forward_from": {"id": 2, "first_name": "firstname", "is_bot": false},
        "forward_date": 0
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let Some(Forward {
        date,
        from: ForwardFrom::User(user),
    }) = msg.forward
    {
        assert_eq!(date, 0);
        assert_eq!(user.id, 2);
        assert_eq!(user.first_name, String::from("firstname"));
        assert_eq!(user.is_bot, false);
    } else {
        panic!("Unexpected forward data: {:?}", msg.forward);
    }

    let input = json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "forward_from_chat": {"id": 1, "type": "channel", "title": "test"},
        "forward_from_message_id": 1,
        "forward_signature": "test",
        "forward_date": 0
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let Some(Forward {
        date,
        from: ForwardFrom::Channel {
            chat,
            message_id,
            signature,
        },
    }) = msg.forward
    {
        assert_eq!(date, 0);
        assert_eq!(message_id, 1);
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("test"));
        assert_eq!(signature, Some(String::from("test")));
    } else {
        panic!("Unexpected forward data: {:?}", msg.forward);
    }

    let input = json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "forward_from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "forward_from_chat": {"id": 1, "type": "channel", "title": "test"},
        "forward_from_message_id": 1,
        "forward_signature": "test",
        "forward_date": 0
    });
    let err = serde_json::from_value::<Message>(input).unwrap_err();
    assert_eq!(err.to_string(), String::from("Unexpected forward_* fields combination"));
}

#[test]
fn test_deserialize_message_reply() {
    let input = json!({
        "message_id": 2, "date": 1,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "test",
        "reply_to_message": {
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test"
        }
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let Some(msg) = msg.reply_to {
        assert_eq!(msg.id, 1);
    } else {
        panic!("Unexpected reply_to data: {:?}", msg.reply_to);
    }
}

#[test]
fn test_deserialize_message_data() {
    let input = json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "animation": {
            "file_id": "fileid",
            "width": 200,
            "height": 200,
            "duration": 10
        },
        "document": {
            "file_id": "fileid"
        }
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    if let MessageData::Animation(animation) = msg.data {
        assert_eq!(animation.file_id, String::from("fileid"));
        assert_eq!(animation.width, 200);
        assert_eq!(animation.height, 200);
        assert_eq!(animation.duration, 10);
    } else {
        panic!("Unexpected message data: {:?}", msg.data)
    }
}

#[test]
fn test_deserialize_message_entities() {
    let input = json!({
        "message_id": 1, "date": 0,
        "from": {"id": 1, "first_name": "firstname", "is_bot": false},
        "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
        "text": "bold /botcommand $cashtag code u@h.z #hashtag italic @mention phone pre textlink textmention url",
        "entities": [
            {"type": "bold", "offset": 0, "length": 4},
            {"type": "bot_command", "offset": 5, "length": 11},
            {"type": "cashtag", "offset": 17, "length": 8},
            {"type": "code", "offset": 26, "length": 4},
            {"type": "email", "offset": 31, "length": 5},
            {"type": "hashtag", "offset": 37, "length": 8},
            {"type": "italic", "offset": 46, "length": 6},
            {"type": "mention", "offset": 53, "length": 8},
            {"type": "phone_number", "offset": 62, "length": 5},
            {"type": "pre", "offset": 68, "length": 3},
            {"type": "text_link", "offset": 72, "length": 8, "url": "https://example.com"},
            {
                "type": "text_mention",
                "offset": 81,
                "length": 11,
                "user": {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                }
            },
            {"type": "url", "offset": 93, "length": 3}
        ]
    });
    let msg: Message = serde_json::from_value(input).unwrap();
    assert_eq!(msg.commands.unwrap().len(), 1);
    if let MessageData::Text(text) = msg.data {
        let entities = text.entities.unwrap();
        assert_eq!(
            vec![
                TextEntity::Bold(TextEntityData {
                    data: String::from("bold"),
                    offset: 0,
                    length: 4
                }),
                TextEntity::BotCommand(BotCommand {
                    command: String::from("/botcommand"),
                    bot_name: None,
                    data: TextEntityData {
                        data: String::from("/botcommand"),
                        offset: 5,
                        length: 11
                    }
                }),
                TextEntity::Cashtag(TextEntityData {
                    data: String::from("$cashtag"),
                    offset: 17,
                    length: 8
                }),
                TextEntity::Code(TextEntityData {
                    data: String::from("code"),
                    offset: 26,
                    length: 4
                }),
                TextEntity::Email(TextEntityData {
                    data: String::from("u@h.z"),
                    offset: 31,
                    length: 5
                }),
                TextEntity::Hashtag(TextEntityData {
                    data: String::from("#hashtag"),
                    offset: 37,
                    length: 8
                }),
                TextEntity::Italic(TextEntityData {
                    data: String::from("italic"),
                    offset: 46,
                    length: 6
                }),
                TextEntity::Mention(TextEntityData {
                    data: String::from("@mention"),
                    offset: 53,
                    length: 8
                }),
                TextEntity::PhoneNumber(TextEntityData {
                    data: String::from("phone"),
                    offset: 62,
                    length: 5
                }),
                TextEntity::Pre(TextEntityData {
                    data: String::from("pre"),
                    offset: 68,
                    length: 3
                }),
                TextEntity::TextLink(TextLink {
                    data: TextEntityData {
                        data: String::from("textlink"),
                        offset: 72,
                        length: 8
                    },
                    url: String::from("https://example.com")
                }),
                TextEntity::TextMention(TextMention {
                    data: TextEntityData {
                        data: String::from("textmention"),
                        offset: 81,
                        length: 11
                    },
                    user: User {
                        id: 1,
                        is_bot: false,
                        first_name: String::from("test"),
                        last_name: None,
                        username: None,
                        language_code: None
                    }
                }),
                TextEntity::Url(TextEntityData {
                    data: String::from("url"),
                    offset: 93,
                    length: 3
                })
            ],
            entities
        );
    } else {
        panic!("Unexpected message data: {:?}", msg.data);
    }
}

#[test]
fn test_deserialize_message_bad_entities() {
    for (input, error) in vec![
        (
            json!({
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "bad offset",
                "entities": [
                    {
                        "type": "bold",
                        "offset": -1,
                        "length": 1
                    }
                ]
            }),
            "Failed to parse text: Offset \"-1\" is out of text bounds",
        ),
        (
            json!({
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "bad offset",
                "entities": [
                    {
                        "type": "bold",
                        "offset": 11,
                        "length": 1
                    }
                ]
            }),
            "Failed to parse text: Offset \"11\" is out of text bounds",
        ),
        (
            json!({
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "bad offset",
                "entities": [
                    {
                        "type": "bold",
                        "offset": 0,
                        "length": -1
                    }
                ]
            }),
            "Failed to parse text: Length \"-1\" is out of text bounds",
        ),
        (
            json!({
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "bad offset",
                "entities": [
                    {
                        "type": "bold",
                        "offset": 0,
                        "length": 11
                    }
                ]
            }),
            "Failed to parse text: Length \"11\" is out of text bounds",
        ),
        (
            json!({
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "bad offset",
                "entities": [
                    {
                        "type": "text_link",
                        "offset": 0,
                        "length": 2
                    }
                ]
            }),
            "Failed to parse text: URL is required for text_link entity",
        ),
        (
            json!({
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "bad offset",
                "entities": [
                    {
                        "type": "text_mention",
                        "offset": 0,
                        "length": 2
                    }
                ]
            }),
            "Failed to parse text: User is required for text_mention entity",
        ),
    ] {
        let err = serde_json::from_value::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), error.to_string());
    }
}
