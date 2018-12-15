use crate::types::keyboards::InlineKeyboardMarkup;
use crate::types::location::Location;
use crate::types::primitive::{Float, Integer};
use crate::types::user::User;

/// This object represents an incoming inline query.
/// When the user sends an empty query, your bot could return some default or trending results.
#[derive(Debug)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 512 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}

/// Represents a link to an article or web page.
#[derive(Debug)]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be article
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    pub url: Option<String>,
    /// Pass True, if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    /// Short description of the result
    pub description: Option<String>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

/// Represents a link to an mp3 audio file.
/// By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the audio.
#[derive(Debug)]
pub struct InlineQueryResultAudio {
    /// Type of the result, must be audio
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Performer
    pub performer: Option<String>,
    /// Audio duration in seconds
    pub audio_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an mp3 audio file
/// stored on the Telegram servers.
/// By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the audio.
#[derive(Debug)]
pub struct InlineQueryResultCachedAudio {
    /// Type of the result, must be audio
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a file
/// stored on the Telegram servers.
/// By default, this file will be sent
/// by the user with an optional caption.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the file.
#[derive(Debug)]
pub struct InlineQueryResultCachedDocument {
    /// Type of the result, must be document
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to an animated GIF file
/// stored on the Telegram servers.
/// By default, this animated GIF file will be sent
/// by the user with an optional caption.
/// Alternatively, you can use input_message_content to send
/// a message with specified content instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultCachedGif {
    /// Type of the result, must be gif
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text
    /// or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video animation
/// (H.264/MPEG-4 AVC video without sound)
/// stored on the Telegram servers.
/// By default, this animated MPEG-4 file
/// will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content
/// instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MP4 file
    pub mpeg4_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a photo stored on the Telegram servers.
/// By default, this photo will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the photo.
#[derive(Debug)]
pub struct InlineQueryResultCachedPhoto {
    /// Type of the result, must be photo
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a sticker stored on the Telegram servers.
/// By default, this sticker will be sent by the user.
/// Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the sticker.
#[derive(Debug)]
pub struct InlineQueryResultCachedSticker {
    /// Type of the result, must be sticker
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a video file
/// stored on the Telegram servers.
/// By default, this video file
/// will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the video.
#[derive(Debug)]
pub struct InlineQueryResultCachedVideo {
    /// Type of the result, must be video
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a voice message
/// stored on the Telegram servers.
/// By default, this voice message
/// will be sent by the user.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content
/// instead of the voice message.
#[derive(Debug)]
pub struct InlineQueryResultCachedVoice {
    /// Type of the result, must be voice
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice message
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a contact with a phone number.
/// By default, this contact will be sent by the user.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the contact.
#[derive(Debug)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be contact
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

/// Represents a link to a file.
/// By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the file.
/// Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Debug)]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be document
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// A valid URL for the file
    pub document_url: String,
    /// Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail (jpeg only) for the file
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

/// Represents a Game.
#[derive(Debug)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be game
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Represents a link to an animated GIF file.
/// By default, this animated GIF file
/// will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be gif
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Width of the GIF
    pub gif_width: Option<Integer>,
    /// Height of the GIF
    pub gif_height: Option<Integer>,
    /// Duration of the GIF
    pub gif_duration: Option<Integer>,
    /// URL of the static thumbnail for the result (jpeg or gif)
    pub thumb_url: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a location on a map.
/// By default, the location will be sent by the user.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the location.
#[derive(Debug)]
pub struct InlineQueryResultLocation {
    /// Type of the result, must be location
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: Float,
    /// Location longitude in degrees
    pub longitude: Float,
    /// Location title
    pub title: String,
    /// Period in seconds for
    /// which the location can be updated,
    /// should be between 60 and 86400.
    pub live_period: Option<Integer>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

/// Represents a link to a video animation
/// (H.264/MPEG-4 AVC video without sound).
/// By default, this animated MPEG-4 file
/// will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content
/// instead of the animation.
#[derive(Debug)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MP4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Video width
    pub mpeg4_width: Option<Integer>,
    /// Video height
    pub mpeg4_height: Option<Integer>,
    /// Video duration
    pub mpeg4_duration: Option<Integer>,
    /// URL of the static thumbnail (jpeg or gif) for the result
    pub thumb_url: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a photo.
/// By default, this photo will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the photo.
#[derive(Debug)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo.
    /// Photo must be in jpeg format.
    /// Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    ///  Width of the photo
    pub photo_width: Option<Integer>,
    /// Height of the photo
    pub photo_height: Option<Integer>,
    /// Title for the result
    pub title: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a venue.
/// By default, the venue will be sent by the user.
/// Alternatively, you can use input_message_content
/// to send a message with the specified content
/// instead of the venue.
#[derive(Debug)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be venue
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: Float,
    /// Longitude of the venue location in degrees
    pub longitude: Float,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<Integer>,
    /// Thumbnail height
    pub thumb_height: Option<Integer>,
}

/// Represents a link to a page containing an embedded video player or a video file.
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with
/// the specified content instead of the video.
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube),
/// you must replace its content using input_message_content.
#[derive(Debug)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be video
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// Mime type of the content of video url, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    /// Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Video width
    pub video_width: Option<Integer>,
    /// Video height
    pub video_height: Option<Integer>,
    /// Video duration in seconds
    pub video_duration: Option<Integer>,
    /// Short description of the result
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video.
    /// This field is required if InlineQueryResultVideo
    /// is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}

/// Represents a link to a voice recording in an .ogg container encoded with OPUS.
/// By default, this voice recording will be sent by the user.
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the the voice message.
#[derive(Debug)]
pub struct InlineQueryResultVoice {
    /// Type of the result, must be voice
    pub kind: String, // TODO: rename to type
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    /// Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Recording duration in seconds
    pub voice_duration: Option<Integer>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice recording
    pub input_message_content: Option<InputMessageContent>,
}

/// This object represents the content of a message to be sent as a result of an inline query.
#[derive(Debug)]
pub enum InputMessageContent {
    /// Represents the content of a text message to be sent as the result of an inline query.
    Text(InputTextMessageContent),
    /// Represents the content of a location message to be sent as the result of an inline query.
    Location(InputLocationMessageContent),
    /// Represents the content of a venue message to be sent as the result of an inline query.
    Venue(InputVenueMessageContent),
    /// Represents the content of a contact message to be sent as the result of an inline query.
    Contact(InputContactMessageContent),
}

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in your bot's message.
    pub parse_mode: Option<String>,
    ///  Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: Float,
    /// Longitude of the location in degrees
    pub longitude: Float,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<Integer>,
}

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: Float,
    /// Longitude of the venue in degrees
    pub longitude: Float,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    ///  Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
}

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
}

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Debug)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message.
    /// Available only if there is an inline keyboard attached to the message.
    /// Will be also received in callback queries and can be used to edit the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}