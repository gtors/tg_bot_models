#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


/// This object represents an incoming update.At most one of the optional parameters
/// can be present in any given update.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially. This ID becomes especially handy
    /// if you’re using Webhooks, since it allows you to ignore repeated updates or
    /// to restore the correct update sequence, should they get out of order.
    pub update_id: i64,
    /// Optional. New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,
    /// Optional. New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,
    /// Optional. New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// Optional. New version of a channel post that is known to the bot and was
    /// edited
    pub edited_channel_post: Option<Message>,
    /// Optional. New incoming inline query
    pub inline_query: Option<InlineQuery>,
    /// Optional. The result of an inline query that was chosen by a user and sent
    /// to their chat partner. Please see our documentation on the feedback
    /// collecting for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// Optional. New incoming callback query
    pub callback_query: Option<CallbackQuery>,
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<ShippingQuery>,
    /// Optional. New incoming pre-checkout query. Contains full information about
    /// checkout
    pub pre_checkout_query: Option<PreCheckoutQuery>,
}


/// Contains information about the current status of a webhook.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Optional. Unix time for the most recent error that happened when trying to
    /// deliver an update via webhook
    pub last_error_date: Option<i64>,
    /// Optional. Error message in human-readable format for the most recent error
    /// that happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Optional. Maximum allowed number of simultaneous HTTPS connections to the
    /// webhook for update delivery
    pub max_connections: Option<i64>,
    /// Optional. A list of update types the bot is subscribed to. Defaults to all
    /// update types
    pub allowed_updates: Option<Vec<String>>,
}


/// This object represents a Telegram user or bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: i64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// Optional. User‘s or bot’s last name
    pub last_name: Option<String>,
    /// Optional. User‘s or bot’s username
    pub username: Option<String>,
    /// Optional. IETF language tag of the user's language
    pub language_code: Option<String>,
}


/// This object represents a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat. This number may be greater than 32 bits and
    /// some programming languages may have difficulty/silent defects in
    /// interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer
    /// or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub ty: String,
    /// Optional. Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Optional. True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,
    /// Optional. Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Optional. Description, for supergroups and channel chats. Returned only in
    /// getChat.
    pub description: Option<String>,
    /// Optional. Chat invite link, for supergroups and channel chats. Returned only
    /// in getChat.
    pub invite_link: Option<String>,
    /// Optional. Pinned message, for supergroups. Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// Optional. For supergroups, name of group sticker set. Returned only in
    /// getChat.
    pub sticker_set_name: Option<String>,
    /// Optional. True, if the bot can change the group sticker set. Returned only
    /// in getChat.
    pub can_set_sticker_set: Option<bool>,
}


/// This object represents a message.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,
    /// Optional. Sender, empty for messages sent to channels
    pub from: Option<User>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Optional. For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// Optional. For messages forwarded from channels, information about the
    /// original channel
    pub forward_from_chat: Option<Chat>,
    /// Optional. For messages forwarded from channels, identifier of the original
    /// message in the channel
    pub forward_from_message_id: Option<i64>,
    /// Optional. For messages forwarded from channels, signature of the post author
    /// if present
    pub forward_signature: Option<String>,
    /// Optional. For forwarded messages, date the original message was sent in Unix
    /// time
    pub forward_date: Option<i64>,
    /// Optional. For replies, the original message. Note that the Message object in
    /// this field will not contain further reply_to_message fields even if it
    /// itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Optional. Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// Optional. The unique identifier of a media message group this message
    /// belongs to
    pub media_group_id: Option<String>,
    /// Optional. Signature of the post author for messages in channels
    pub author_signature: Option<String>,
    /// Optional. For text messages, the actual UTF-8 text of the message, 0-4096
    /// characters.
    pub text: Option<String>,
    /// Optional. For text messages, special entities like usernames, URLs, bot
    /// commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. For messages with a caption, special entities like usernames,
    /// URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Optional. Message is a general file, information about the file
    pub document: Option<Document>,
    /// Optional. Message is a game, information about the game. More about games »
    pub game: Option<Game>,
    /// Optional. Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Optional. Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Optional. Message is a video, information about the video
    pub video: Option<Video>,
    /// Optional. Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Optional. Message is a video note, information about the video message
    pub video_note: Option<VideoNote>,
    /// Optional. Caption for the audio, document, photo, video or voice, 0-200
    /// characters
    pub caption: Option<String>,
    /// Optional. Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Optional. Message is a shared location, information about the location
    pub location: Option<Location>,
    /// Optional. Message is a venue, information about the venue
    pub venue: Option<Venue>,
    /// Optional. New members that were added to the group or supergroup and
    /// information about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// Optional. A member was removed from the group, information about them (this
    /// member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// Optional. A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// Optional. A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Optional. Service message: the chat photo was deleted
    pub delete_chat_photo: Option<bool>,
    /// Optional. Service message: the group has been created
    pub group_chat_created: Option<bool>,
    /// Optional. Service message: the supergroup has been created. This field can‘t
    /// be received in a message coming through updates, because bot can’t be a
    /// member of a supergroup when it is created. It can only be found in
    /// reply_to_message if someone replies to a very first message in a directly
    /// created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// Optional. Service message: the channel has been created. This field can‘t be
    /// received in a message coming through updates, because bot can’t be a member
    /// of a channel when it is created. It can only be found in reply_to_message if
    /// someone replies to a very first message in a channel.
    pub channel_chat_created: Option<bool>,
    /// Optional. The group has been migrated to a supergroup with the specified
    /// identifier. This number may be greater than 32 bits and some programming
    /// languages may have difficulty/silent defects in interpreting it. But it is
    /// smaller than 52 bits, so a signed 64 bit integer or double-precision float
    /// type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// Optional. The supergroup has been migrated from a group with the specified
    /// identifier. This number may be greater than 32 bits and some programming
    /// languages may have difficulty/silent defects in interpreting it. But it is
    /// smaller than 52 bits, so a signed 64 bit integer or double-precision float
    /// type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// Optional. Specified message was pinned. Note that the Message object in this
    /// field will not contain further reply_to_message fields even if it is itself
    /// a reply.
    pub pinned_message: Option<Box<Message>>,
    /// Optional. Message is an invoice for a payment, information about the
    /// invoice. More about payments »
    pub invoice: Option<Invoice>,
    /// Optional. Message is a service message about a successful payment,
    /// information about the payment. More about payments »
    pub successful_payment: Option<SuccessfulPayment>,
}


/// This object represents one special entity in a text message. For example,
/// hashtags, usernames, URLs, etc.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity. Can be mention (@username), hashtag, bot_command, url,
    /// email, bold (bold text), italic (italic text), code (monowidth string), pre
    /// (monowidth block), text_link (for clickable text URLs), text_mention (for
    /// users without usernames)
    #[serde(rename = "type")]
    pub ty: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// Optional. For “text_link” only, url that will be opened after user taps on
    /// the text
    pub url: Option<String>,
    /// Optional. For “text_mention” only, the mentioned user
    pub user: Option<User>,
}


/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub file_id: String,
    /// Photo width
    pub width: i64,
    /// Photo height
    pub height: i64,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents an audio file to be treated as music by the Telegram
/// clients.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Audio {
    /// Unique identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// Optional. Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Optional. Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents a general file (as opposed to photos, voice messages and
/// audio files).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Document {
    /// Unique file identifier
    pub file_id: String,
    /// Optional. Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Optional. Original filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents a video file.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Video {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents a voice note.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Voice {
    /// Unique identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width and height as defined by sender
    pub length: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents a phone contact.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    pub last_name: Option<String>,
    /// Optional. Contact's user identifier in Telegram
    pub user_id: Option<i64>,
}


/// This object represents a point on the map.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
}


/// This object represents a venue.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Venue {
    /// Venue location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
}


/// This object represent a user's profile pictures.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}


/// This object represents a file ready to be downloaded. The file can be downloaded
/// via the link https://api.telegram.org/file/bot<token>/<file_path>. It is
/// guaranteed that the link will be valid for at least 1 hour. When the link
/// expires, a new one can be requested by calling getFile.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct File {
    /// Unique identifier for this file
    pub file_id: String,
    /// Optional. File size, if known
    pub file_size: Option<i64>,
    /// Optional. File path. Use
    /// https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    pub file_path: Option<String>,
}


/// This object represents a custom keyboard with reply options (see Introduction to
/// bots for details and examples).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Optional. Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard is always of the same
    /// height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,
    /// Optional. Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically display
    /// the usual letter-keyboard in the chat – the user can press a special button
    /// in the input field to see the custom keyboard again. Defaults to false.
    pub one_time_keyboard: Option<bool>,
    /// Optional. Use this parameter if you want to show the keyboard to specific
    /// users only. Targets: 1) users that are @mentioned in the text of the Message
    /// object; 2) if the bot's message is a reply (has reply_to_message_id), sender
    /// of the original message.Example: A user requests to change the bot‘s
    /// language, bot replies to the request with a keyboard to select the new
    /// language. Other users in the group don’t see the keyboard.
    pub selective: Option<bool>,
}


/// This object represents one button of the reply keyboard. For simple text buttons
/// String can be used instead of this object to specify text of the button.
/// Optional fields are mutually exclusive.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent
    /// as a message when the button is pressed
    pub text: String,
    /// Optional. If True, the user's phone number will be sent as a contact when
    /// the button is pressed. Available in private chats only
    pub request_contact: Option<bool>,
    /// Optional. If True, the user's current location will be sent when the button
    /// is pressed. Available in private chats only
    pub request_location: Option<bool>,
}


/// Upon receiving a message with this object, Telegram clients will remove the
/// current custom keyboard and display the default letter-keyboard. By default,
/// custom keyboards are displayed until a new keyboard is sent by a bot. An
/// exception is made for one-time keyboards that are hidden immediately after the
/// user presses a button (see ReplyKeyboardMarkup).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to
    /// summon this keyboard; if you want to hide the keyboard from sight but keep
    /// it accessible, use one_time_keyboard in ReplyKeyboardMarkup)
    pub remove_keyboard: bool,
    /// Optional. Use this parameter if you want to remove the keyboard for specific
    /// users only. Targets: 1) users that are @mentioned in the text of the Message
    /// object; 2) if the bot's message is a reply (has reply_to_message_id), sender
    /// of the original message.Example: A user votes in a poll, bot returns
    /// confirmation message in reply to the vote and removes the keyboard for that
    /// user, while still showing the keyboard with poll options to users who
    /// haven't voted yet.
    pub selective: Option<bool>,
}


/// This object represents an inline keyboard that appears right next to the message
/// it belongs to.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton
    /// objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}


/// This object represents one button of an inline keyboard. You must use exactly
/// one of the optional fields.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// Optional. HTTP url to be opened when button is pressed
    pub url: Option<String>,
    /// Optional. Data to be sent in a callback query to the bot when button is
    /// pressed, 1-64 bytes
    pub callback_data: Option<String>,
    /// Optional. If set, pressing the button will prompt the user to select one of
    /// their chats, open that chat and insert the bot‘s username and the specified
    /// inline query in the input field. Can be empty, in which case just the bot’s
    /// username will be inserted.Note: This offers an easy way for users to start
    /// using your bot in inline mode when they are currently in a private chat with
    /// it. Especially useful when combined with switch_pm… actions – in this case
    /// the user will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen.
    pub switch_inline_query: Option<String>,
    /// Optional. If set, pressing the button will insert the bot‘s username and the
    /// specified inline query in the current chat's input field. Can be empty, in
    /// which case only the bot’s username will be inserted.This offers a quick way
    /// for the user to open your bot in inline mode in the same chat – good for
    /// selecting something from multiple options.
    pub switch_inline_query_current_chat: Option<String>,
    /// Optional. Description of the game that will be launched when the user
    /// presses the button.NOTE: This type of button must always be the first button
    /// in the first row.
    pub callback_game: Option<String>,
    /// Optional. Specify True, to send a Pay button.NOTE: This type of button must
    /// always be the first button in the first row.
    pub pay: Option<bool>,
}


/// This object represents an incoming callback query from a callback button in an
/// inline keyboard. If the button that originated the query was attached to a
/// message sent by the bot, the field message will be present. If the button was
/// attached to a message sent via the bot (in inline mode), the field
/// inline_message_id will be present. Exactly one of the fields data or
/// game_short_name will be present.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Message with the callback button that originated the query. Note
    /// that message content and message date will not be available if the message
    /// is too old
    pub message: Option<Message>,
    /// Optional. Identifier of the message sent via the bot in inline mode, that
    /// originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message
    /// with the callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Optional. Data associated with the callback button. Be aware that a bad
    /// client can send arbitrary data in this field.
    pub data: Option<String>,
    /// Optional. Short name of a Game to be returned, serves as the unique
    /// identifier for the game
    pub game_short_name: Option<String>,
}


/// Upon receiving a message with this object, Telegram clients will display a reply
/// interface to the user (act as if the user has selected the bot‘s message and
/// tapped ’Reply'). This can be extremely useful if you want to create user-
/// friendly step-by-step interfaces without having to sacrifice privacy mode.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot‘s
    /// message and tapped ’Reply'
    pub force_reply: bool,
    /// Optional. Use this parameter if you want to force reply from specific users
    /// only. Targets: 1) users that are @mentioned in the text of the Message
    /// object; 2) if the bot's message is a reply (has reply_to_message_id), sender
    /// of the original message.
    pub selective: Option<bool>,
}


/// This object represents a chat photo.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatPhoto {
    /// Unique file identifier of small (160x160) chat photo. This file_id can be
    /// used only for photo download.
    pub small_file_id: String,
    /// Unique file identifier of big (640x640) chat photo. This file_id can be used
    /// only for photo download.
    pub big_file_id: String,
}


/// This object contains information about one member of a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatMember {
    /// Information about the user
    pub user: User,
    /// The member's status in the chat. Can be “creator”, “administrator”,
    /// “member”, “restricted”, “left” or “kicked”
    pub status: String,
    /// Optional. Restictred and kicked only. Date when restrictions will be lifted
    /// for this user, unix time
    pub until_date: Option<i64>,
    /// Optional. Administrators only. True, if the bot is allowed to edit
    /// administrator privileges of that user
    pub can_be_edited: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can change the
    /// chat title, photo and other settings
    pub can_change_info: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can post in the
    /// channel, channels only
    pub can_post_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can edit messages
    /// of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can delete
    /// messages of other users
    pub can_delete_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can invite new
    /// users to the chat
    pub can_invite_users: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can restrict, ban
    /// or unban chat members
    pub can_restrict_members: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can pin messages,
    /// supergroups only
    pub can_pin_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can add new
    /// administrators with a subset of his own privileges or demote administrators
    /// that he has promoted, directly or indirectly (promoted by administrators
    /// that were appointed by the user)
    pub can_promote_members: Option<bool>,
    /// Optional. Restricted only. True, if the user can send text messages,
    /// contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// Optional. Restricted only. True, if the user can send audios, documents,
    /// photos, videos, video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    /// Optional. Restricted only. True, if the user can send animations, games,
    /// stickers and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    /// Optional. Restricted only. True, if user may add web page previews to his
    /// messages, implies can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
}


/// Contains information about why a request was unsuccessful.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ResponseParameters {
    /// Optional. The group has been migrated to a supergroup with the specified
    /// identifier. This number may be greater than 32 bits and some programming
    /// languages may have difficulty/silent defects in interpreting it. But it is
    /// smaller than 52 bits, so a signed 64 bit integer or double-precision float
    /// type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// Optional. In case of exceeding flood control, the number of seconds left to
    /// wait before the request can be repeated
    pub retry_after: Option<i64>,
}


/// This object represents the content of a media message to be sent. It should be
/// one of
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum InputMedia {
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}


/// Represents a photo to be sent.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputMediaPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub ty: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from the
    /// Internet, or pass "attach://<file_attach_name>" to upload a new one using
    /// multipart/form-data under <file_attach_name> name. More info on Sending
    /// Files »
    pub media: String,
    /// Optional. Caption of the photo to be sent, 0-200 characters
    pub caption: Option<String>,
}


/// Represents a video to be sent.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub ty: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from the
    /// Internet, or pass "attach://<file_attach_name>" to upload a new one using
    /// multipart/form-data under <file_attach_name> name. More info on Sending
    /// Files »
    pub media: String,
    /// Optional. Caption of the video to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Video width
    pub width: Option<i64>,
    /// Optional. Video height
    pub height: Option<i64>,
    /// Optional. Video duration
    pub duration: Option<i64>,
}


/// This object represents a sticker.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Sticker {
    /// Unique identifier for this file
    pub file_id: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// Optional. Sticker thumbnail in the .webp or .jpg format
    pub thumb: Option<PhotoSize>,
    /// Optional. Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Optional. Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// Optional. For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents a sticker set.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// True, if the sticker set contains masks
    pub contains_masks: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
}


/// This object describes the position on faces where a mask should be placed by
/// default.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of
    /// “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from
    /// left to right. For example, choosing -1.0 will place mask just to the left
    /// of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size,
    /// from top to bottom. For example, 1.0 will place the mask just below the
    /// default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}


/// This object represents an incoming inline query. When the user sends an empty
/// query, your bot could return some default or trending results.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 512 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}


/// This object represents one result of an inline query. Telegram clients currently
/// support results of the following 20 types:
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum InlineQueryResult {
    InlineQueryResultCachedAudio(InlineQueryResultCachedAudio),
    InlineQueryResultCachedDocument(InlineQueryResultCachedDocument),
    InlineQueryResultCachedGif(InlineQueryResultCachedGif),
    InlineQueryResultCachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    InlineQueryResultCachedPhoto(InlineQueryResultCachedPhoto),
    InlineQueryResultCachedSticker(InlineQueryResultCachedSticker),
    InlineQueryResultCachedVideo(InlineQueryResultCachedVideo),
    InlineQueryResultCachedVoice(InlineQueryResultCachedVoice),
    InlineQueryResultArticle(InlineQueryResultArticle),
    InlineQueryResultAudio(InlineQueryResultAudio),
    InlineQueryResultContact(InlineQueryResultContact),
    InlineQueryResultGame(InlineQueryResultGame),
    InlineQueryResultDocument(InlineQueryResultDocument),
    InlineQueryResultGif(InlineQueryResultGif),
    InlineQueryResultLocation(InlineQueryResultLocation),
    InlineQueryResultMpeg4Gif(InlineQueryResultMpeg4Gif),
    InlineQueryResultPhoto(InlineQueryResultPhoto),
    InlineQueryResultVenue(InlineQueryResultVenue),
    InlineQueryResultVideo(InlineQueryResultVideo),
    InlineQueryResultVoice(InlineQueryResultVoice),
}


/// Represents a link to an article or web page.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be article
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. URL of the result
    pub url: Option<String>,
    /// Optional. Pass True, if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i64>,
}


/// Represents a link to a photo. By default, this photo will be sent by the user
/// with optional caption. Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the photo.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not
    /// exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    /// Optional. Width of the photo
    pub photo_width: Option<i64>,
    /// Optional. Height of the photo
    pub photo_height: Option<i64>,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to an animated GIF file. By default, this animated GIF file
/// will be sent by the user with optional caption. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of
/// the animation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Optional. Width of the GIF
    pub gif_width: Option<i64>,
    /// Optional. Height of the GIF
    pub gif_height: Option<i64>,
    /// Optional. Duration of the GIF
    pub gif_duration: Option<i64>,
    /// URL of the static thumbnail for the result (jpeg or gif)
    pub thumb_url: String,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
/// By default, this animated MPEG-4 file will be sent by the user with optional
/// caption. Alternatively, you can use input_message_content to send a message with
/// the specified content instead of the animation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MP4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Optional. Video width
    pub mpeg4_width: Option<i64>,
    /// Optional. Video height
    pub mpeg4_height: Option<i64>,
    /// Optional. Video duration
    pub mpeg4_duration: Option<i64>,
    /// URL of the static thumbnail (jpeg or gif) for the result
    pub thumb_url: String,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a page containing an embedded video player or a video file.
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the
/// specified content instead of the video.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub ty: String,
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
    /// Optional. Caption of the video to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Video width
    pub video_width: Option<i64>,
    /// Optional. Video height
    pub video_height: Option<i64>,
    /// Optional. Video duration in seconds
    pub video_duration: Option<i64>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video. This field
    /// is required if InlineQueryResultVideo is used to send an HTML-page as a
    /// result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to an mp3 audio file. By default, this audio file will be sent
/// by the user. Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the audio.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Optional. Caption, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Performer
    pub performer: Option<String>,
    /// Optional. Audio duration in seconds
    pub audio_duration: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a voice recording in an .ogg container encoded with OPUS.
/// By default, this voice recording will be sent by the user. Alternatively, you
/// can use input_message_content to send a message with the specified content
/// instead of the the voice message.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    /// Optional. Caption, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Recording duration in seconds
    pub voice_duration: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the voice recording
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a file. By default, this file will be sent by the user with
/// an optional caption. Alternatively, you can use input_message_content to send a
/// message with the specified content instead of the file. Currently, only .PDF and
/// .ZIP files can be sent using this method.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the document to be sent, 0-200 characters
    pub caption: Option<String>,
    /// A valid URL for the file
    pub document_url: String,
    /// Mime type of the content of the file, either “application/pdf” or
    /// “application/zip”
    pub mime_type: String,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. URL of the thumbnail (jpeg only) for the file
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i64>,
}


/// Represents a location on a map. By default, the location will be sent by the
/// user. Alternatively, you can use input_message_content to send a message with
/// the specified content instead of the location.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    /// Type of the result, must be location
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// Optional. Period in seconds for which the location can be updated, should be
    /// between 60 and 86400.
    pub live_period: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i64>,
}


/// Represents a venue. By default, the venue will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the
/// specified content instead of the venue.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be venue
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i64>,
}


/// Represents a contact with a phone number. By default, this contact will be sent
/// by the user. Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the contact.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be contact
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    pub last_name: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i64>,
}


/// Represents a Game.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be game
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// Represents a link to a photo stored on the Telegram servers. By default, this
/// photo will be sent by the user with an optional caption. Alternatively, you can
/// use input_message_content to send a message with the specified content instead
/// of the photo.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to an animated GIF file stored on the Telegram servers. By
/// default, this animated GIF file will be sent by the user with an optional
/// caption. Alternatively, you can use input_message_content to send a message with
/// specified content instead of the animation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound)
/// stored on the Telegram servers. By default, this animated MPEG-4 file will be
/// sent by the user with an optional caption. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of
/// the animation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MP4 file
    pub mpeg4_file_id: String,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a sticker stored on the Telegram servers. By default, this
/// sticker will be sent by the user. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of
/// the sticker.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    /// Type of the result, must be sticker
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a file stored on the Telegram servers. By default, this
/// file will be sent by the user with an optional caption. Alternatively, you can
/// use input_message_content to send a message with the specified content instead
/// of the file.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Caption of the document to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a video file stored on the Telegram servers. By default,
/// this video file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the
/// specified content instead of the video.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Caption of the video to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to a voice message stored on the Telegram servers. By default,
/// this voice message will be sent by the user. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of
/// the voice message.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVoice {
    /// Type of the result, must be voice
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Optional. Caption, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the voice message
    pub input_message_content: Option<InputMessageContent>,
}


/// Represents a link to an mp3 audio file stored on the Telegram servers. By
/// default, this audio file will be sent by the user. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of
/// the audio.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InlineQueryResultCachedAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Optional. Caption, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}


/// This object represents the content of a message to be sent as a result of an
/// inline query. Telegram clients currently support the following 4 types:
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
}


/// Represents the content of a text message to be sent as the result of an inline
/// query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in your bot's message.
    pub parse_mode: Option<String>,
    /// Optional. Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}


/// Represents the content of a location message to be sent as the result of an
/// inline query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// Optional. Period in seconds for which the location can be updated, should be
    /// between 60 and 86400.
    pub live_period: Option<i64>,
}


/// Represents the content of a venue message to be sent as the result of an inline
/// query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
}


/// Represents the content of a contact message to be sent as the result of an
/// inline query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    pub last_name: Option<String>,
}


/// Represents a result of an inline query that was chosen by the user and sent to
/// their chat partner.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Optional. Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Optional. Identifier of the sent inline message. Available only if there is
    /// an inline keyboard attached to the message. Will be also received in
    /// callback queries and can be used to edit the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}


/// This object represents a portion of the price for goods or services.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the smallest units of the currency (integer, not
    /// float/double). For example, for a price of US$ 1.45 pass amount = 145. See
    /// the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies).
    pub amount: i64,
}


/// This object contains basic information about an invoice.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not
    /// float/double). For example, for a price of US$ 1.45 pass amount = 145. See
    /// the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
}


/// This object represents a shipping address.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}


/// This object represents information about an order.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OrderInfo {
    /// Optional. User name
    pub name: Option<String>,
    /// Optional. User's phone number
    pub phone_number: Option<String>,
    /// Optional. User email
    pub email: Option<String>,
    /// Optional. User shipping address
    pub shipping_address: Option<ShippingAddress>,
}


/// This object represents one shipping option.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}


/// This object contains basic information about a successful payment.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not
    /// float/double). For example, for a price of US$ 1.45 pass amount = 145. See
    /// the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Optional. Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Optional. Order info provided by the user
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}


/// This object contains information about an incoming shipping query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}


/// This object contains information about an incoming pre-checkout query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not
    /// float/double). For example, for a price of US$ 1.45 pass amount = 145. See
    /// the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Optional. Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Optional. Order info provided by the user
    pub order_info: Option<OrderInfo>,
}


/// This object represents a game. Use BotFather to create and edit games, their
/// short names will act as unique identifiers.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Optional. Brief description of the game or high scores included in the game
    /// message. Can be automatically edited to include current high scores for the
    /// game when the bot calls setGameScore, or manually edited using
    /// editMessageText. 0-4096 characters.
    pub text: Option<String>,
    /// Optional. Special entities that appear in text, such as usernames, URLs, bot
    /// commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Optional. Animation that will be displayed in the game message in chats.
    /// Upload via BotFather
    pub animation: Option<Animation>,
}


/// You can provide an animation for your game so that it looks stylish in chats
/// (check out Lumberjack for an example). This object represents an animation file
/// to be displayed in the message containing a game.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Animation {
    /// Unique file identifier
    pub file_id: String,
    /// Optional. Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Optional. Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i64>,
}


/// This object represents one row of the high scores table for a game.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: User,
    /// Score
    pub score: i64,
}
