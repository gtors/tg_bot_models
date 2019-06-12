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
    /// to restore the correct update sequence, should they get out of order. If
    /// there are no new updates for at least a week, then identifier of the next
    /// update will be chosen randomly instead of sequentially.
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
    /// Optional. New poll state. Bots receive only updates about stopped polls and
    /// polls, which are sent by the bot
    pub poll: Option<Poll>,
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
    /// Optional. Chat invite link, for supergroups and channel chats. Each
    /// administrator in a chat generates their own invite links, so the bot must
    /// first generate the link using exportChatInviteLink. Returned only in
    /// getChat.
    pub invite_link: Option<String>,
    /// Optional. Pinned message, for groups, supergroups and channels. Returned
    /// only in getChat.
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
    /// Optional. Sender's name for messages forwarded from users who disallow
    /// adding a link to their account in forwarded messages
    pub forward_sender_name: Option<String>,
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
    /// Optional. Message is an animation, information about the animation. For
    /// backward compatibility, when this field is set, the document field will also
    /// be set
    pub animation: Option<Animation>,
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
    /// Optional. Caption for the animation, audio, document, photo, video or voice,
    /// 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Optional. Message is a shared location, information about the location
    pub location: Option<Location>,
    /// Optional. Message is a venue, information about the venue
    pub venue: Option<Venue>,
    /// Optional. Message is a native poll, information about the poll
    pub poll: Option<Poll>,
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
    /// Optional. The domain name of the website on which the user has logged in.
    /// More about Telegram Login »
    pub connected_website: Option<String>,
    /// Optional. Telegram Passport data
    pub passport_data: Option<PassportData>,
    /// Optional. Inline keyboard attached to the message. login_url buttons are
    /// represented as ordinary url buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// This object represents one special entity in a text message. For example,
/// hashtags, usernames, URLs, etc.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity. Can be mention (@username), hashtag, cashtag,
    /// bot_command, url, email, phone_number, bold (bold text), italic (italic
    /// text), code (monowidth string), pre (monowidth block), text_link (for
    /// clickable text URLs), text_mention (for users without usernames)
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
    /// Optional. Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
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


/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without
/// sound).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Animation {
    /// Unique file identifier
    pub file_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Optional. Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
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
    /// Video width and height (diameter of the video message) as defined by sender
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
    /// Optional. Additional data about the contact in the form of a vCard
    pub vcard: Option<String>,
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
    /// Optional. Foursquare type of the venue. (For example,
    /// “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    pub foursquare_type: Option<String>,
}


/// This object contains information about one answer option in a poll.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: i64,
}


/// This object contains information about a poll.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// True, if the poll is closed
    pub is_closed: bool,
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
    /// Optional. HTTP or tg:// url to be opened when button is pressed
    pub url: Option<String>,
    /// Optional. An HTTP URL used to automatically authorize the user. Can be used
    /// as a replacement for the Telegram Login Widget.
    pub login_url: Option<LoginUrl>,
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


/// This object represents a parameter of the inline keyboard button used to
/// automatically authorize a user. Serves as a great replacement for the Telegram
/// Login Widget when the user is coming from Telegram. All the user needs to do is
/// tap/click a button and confirm that they want to log in:
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LoginUrl {
    /// An HTTP URL to be opened with user authorization data added to the query
    /// string when the button is pressed. If the user refuses to provide
    /// authorization data, the original URL without information about the user will
    /// be opened. The data added is the same as described in Receiving
    /// authorization data.NOTE: You must always check the hash of the received data
    /// to verify the authentication and the integrity of the data as described in
    /// Checking authorization.
    pub url: String,
    /// Optional. New text of the button in forwarded messages.
    pub forward_text: Option<String>,
    /// Optional. Username of a bot, which will be used for user authorization. See
    /// Setting up a bot for more details. If not specified, the current bot's
    /// username will be assumed. The url's domain must be the same as the domain
    /// linked with the bot. See Linking your domain to the bot for more details.
    pub bot_username: Option<String>,
    /// Optional. Pass True to request the permission for your bot to send messages
    /// to the user.
    pub request_write_access: Option<bool>,
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
    /// Optional. Restricted and kicked only. Date when restrictions will be lifted
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
    /// groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can add new
    /// administrators with a subset of his own privileges or demote administrators
    /// that he has promoted, directly or indirectly (promoted by administrators
    /// that were appointed by the user)
    pub can_promote_members: Option<bool>,
    /// Optional. Restricted only. True, if the user is a member of the chat at the
    /// moment of the request
    pub is_member: Option<bool>,
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
    InputMediaAnimation(InputMediaAnimation),
    InputMediaDocument(InputMediaDocument),
    InputMediaAudio(InputMediaAudio),
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
    /// Internet, or pass “attach://<file_attach_name>” to upload a new one using
    /// multipart/form-data under <file_attach_name> name. More info on Sending
    /// Files »
    pub media: String,
    /// Optional. Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
}


/// Represents a video to be sent.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub ty: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from the
    /// Internet, or pass “attach://<file_attach_name>” to upload a new one using
    /// multipart/form-data under <file_attach_name> name. More info on Sending
    /// Files »
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height should
    /// not exceed 320. Ignored if the file is not uploaded using multipart/form-
    /// data. Thumbnails can’t be reused and can be only uploaded as a new file, so
    /// you can pass “attach://<file_attach_name>” if the thumbnail was uploaded
    /// using multipart/form-data under <file_attach_name>. More info on Sending
    /// Files »
    pub thumb: Option<String>,
    /// Optional. Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Optional. Video width
    pub width: Option<i64>,
    /// Optional. Video height
    pub height: Option<i64>,
    /// Optional. Video duration
    pub duration: Option<i64>,
    /// Optional. Pass True, if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
}


/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be
/// sent.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputMediaAnimation {
    /// Type of the result, must be animation
    #[serde(rename = "type")]
    pub ty: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from the
    /// Internet, or pass “attach://<file_attach_name>” to upload a new one using
    /// multipart/form-data under <file_attach_name> name. More info on Sending
    /// Files »
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height should
    /// not exceed 320. Ignored if the file is not uploaded using multipart/form-
    /// data. Thumbnails can’t be reused and can be only uploaded as a new file, so
    /// you can pass “attach://<file_attach_name>” if the thumbnail was uploaded
    /// using multipart/form-data under <file_attach_name>. More info on Sending
    /// Files »
    pub thumb: Option<String>,
    /// Optional. Caption of the animation to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Optional. Animation width
    pub width: Option<i64>,
    /// Optional. Animation height
    pub height: Option<i64>,
    /// Optional. Animation duration
    pub duration: Option<i64>,
}


/// Represents an audio file to be treated as music to be sent.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputMediaAudio {
    /// Type of the result, must be audio
    #[serde(rename = "type")]
    pub ty: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from the
    /// Internet, or pass “attach://<file_attach_name>” to upload a new one using
    /// multipart/form-data under <file_attach_name> name. More info on Sending
    /// Files »
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height should
    /// not exceed 320. Ignored if the file is not uploaded using multipart/form-
    /// data. Thumbnails can’t be reused and can be only uploaded as a new file, so
    /// you can pass “attach://<file_attach_name>” if the thumbnail was uploaded
    /// using multipart/form-data under <file_attach_name>. More info on Sending
    /// Files »
    pub thumb: Option<String>,
    /// Optional. Caption of the audio to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Optional. Duration of the audio in seconds
    pub duration: Option<i64>,
    /// Optional. Performer of the audio
    pub performer: Option<String>,
    /// Optional. Title of the audio
    pub title: Option<String>,
}


/// Represents a general file to be sent.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InputMediaDocument {
    /// Type of the result, must be document
    #[serde(rename = "type")]
    pub ty: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from the
    /// Internet, or pass “attach://<file_attach_name>” to upload a new one using
    /// multipart/form-data under <file_attach_name> name. More info on Sending
    /// Files »
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height should
    /// not exceed 320. Ignored if the file is not uploaded using multipart/form-
    /// data. Thumbnails can’t be reused and can be only uploaded as a new file, so
    /// you can pass “attach://<file_attach_name>” if the thumbnail was uploaded
    /// using multipart/form-data under <file_attach_name>. More info on Sending
    /// Files »
    pub thumb: Option<String>,
    /// Optional. Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Foursquare type of the venue, if known. (For example,
    /// “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    pub foursquare_type: Option<String>,
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
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048
    /// bytes
    pub vcard: Option<String>,
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
    /// Optional. Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Caption, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold,
    /// italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
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
    /// Optional. Foursquare type of the venue, if known. (For example,
    /// “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    pub foursquare_type: Option<String>,
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
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048
    /// bytes
    pub vcard: Option<String>,
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


/// Contains information about Telegram Passport data shared with the bot by the
/// user.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements
    /// that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}


/// This object represents a file uploaded to Telegram Passport. Currently all
/// Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportFile {
    /// Unique identifier for this file
    pub file_id: String,
    /// File size
    pub file_size: i64,
    /// Unix time when the file was uploaded
    pub file_date: i64,
}


/// Contains information about documents or other Telegram Passport elements shared
/// with the bot by the user.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    /// Element type. One of “personal_details”, “passport”, “driver_license”,
    /// “identity_card”, “internal_passport”, “address”, “utility_bill”,
    /// “bank_statement”, “rental_agreement”, “passport_registration”,
    /// “temporary_registration”, “phone_number”, “email”.
    #[serde(rename = "type")]
    pub ty: String,
    /// Optional. Base64-encoded encrypted Telegram Passport element data provided
    /// by the user, available for “personal_details”, “passport”, “driver_license”,
    /// “identity_card”, “internal_passport” and “address” types. Can be decrypted
    /// and verified using the accompanying EncryptedCredentials.
    pub data: Option<String>,
    /// Optional. User's verified phone number, available only for “phone_number”
    /// type
    pub phone_number: Option<String>,
    /// Optional. User's verified email address, available only for “email” type
    pub email: Option<String>,
    /// Optional. Array of encrypted files with documents provided by the user,
    /// available for “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration” and “temporary_registration” types. Files can be
    /// decrypted and verified using the accompanying EncryptedCredentials.
    pub files: Option<Vec<PassportFile>>,
    /// Optional. Encrypted file with the front side of the document, provided by
    /// the user. Available for “passport”, “driver_license”, “identity_card” and
    /// “internal_passport”. The file can be decrypted and verified using the
    /// accompanying EncryptedCredentials.
    pub front_side: Option<PassportFile>,
    /// Optional. Encrypted file with the reverse side of the document, provided by
    /// the user. Available for “driver_license” and “identity_card”. The file can
    /// be decrypted and verified using the accompanying EncryptedCredentials.
    pub reverse_side: Option<PassportFile>,
    /// Optional. Encrypted file with the selfie of the user holding a document,
    /// provided by the user; available for “passport”, “driver_license”,
    /// “identity_card” and “internal_passport”. The file can be decrypted and
    /// verified using the accompanying EncryptedCredentials.
    pub selfie: Option<PassportFile>,
    /// Optional. Array of encrypted files with translated versions of documents
    /// provided by the user. Available if requested for “passport”,
    /// “driver_license”, “identity_card”, “internal_passport”, “utility_bill”,
    /// “bank_statement”, “rental_agreement”, “passport_registration” and
    /// “temporary_registration” types. Files can be decrypted and verified using
    /// the accompanying EncryptedCredentials.
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for using in PassportElementErrorUnspecified
    pub hash: String,
}


/// Contains data required for decrypting and authenticating
/// EncryptedPassportElement. See the Telegram Passport Documentation for a complete
/// description of the data decryption and authentication processes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload,
    /// data hashes and secrets required for EncryptedPassportElement decryption and
    /// authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for
    /// data decryption
    pub secret: String,
}


/// This object represents an error in the Telegram Passport element which was
/// submitted that should be resolved by the user. It should be one of:
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum PassportElementError {
    PassportElementErrorDataField(PassportElementErrorDataField),
    PassportElementErrorFrontSide(PassportElementErrorFrontSide),
    PassportElementErrorReverseSide(PassportElementErrorReverseSide),
    PassportElementErrorSelfie(PassportElementErrorSelfie),
    PassportElementErrorFile(PassportElementErrorFile),
    PassportElementErrorFiles(PassportElementErrorFiles),
    PassportElementErrorTranslationFile(PassportElementErrorTranslationFile),
    PassportElementErrorTranslationFiles(PassportElementErrorTranslationFiles),
    PassportElementErrorUnspecified(PassportElementErrorUnspecified),
}


/// Represents an issue in one of the data fields that was provided by the user. The
/// error is considered resolved when the field's value changes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorDataField {
    /// Error source, must be data
    pub source: String,
    /// The section of the user's Telegram Passport which has the error, one of
    /// “personal_details”, “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”, “address”
    #[serde(rename = "type")]
    pub ty: String,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}


/// Represents an issue with the front side of a document. The error is considered
/// resolved when the file with the front side of the document changes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be front_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of
    /// “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    pub ty: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}


/// Represents an issue with the reverse side of a document. The error is considered
/// resolved when the file with reverse side of the document changes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of
    /// “driver_license”, “identity_card”
    #[serde(rename = "type")]
    pub ty: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}


/// Represents an issue with the selfie with a document. The error is considered
/// resolved when the file with the selfie changes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be selfie
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of
    /// “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    pub ty: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}


/// Represents an issue with a document scan. The error is considered resolved when
/// the file with the document scan changes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    /// Error source, must be file
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of
    /// “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub ty: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}


/// Represents an issue with a list of scans. The error is considered resolved when
/// the list of files containing the scans changes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    /// Error source, must be files
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of
    /// “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub ty: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}


/// Represents an issue with one of the files that constitute the translation of a
/// document. The error is considered resolved when the file changes.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be translation_file
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of
    /// “passport”, “driver_license”, “identity_card”, “internal_passport”,
    /// “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub ty: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}


/// Represents an issue with the translated version of a document. The error is
/// considered resolved when a file with the document translation change.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be translation_files
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of
    /// “passport”, “driver_license”, “identity_card”, “internal_passport”,
    /// “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub ty: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}


/// Represents an issue in an unspecified place. The error is considered resolved
/// when new data is added.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be unspecified
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub ty: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
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



#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum PolymorphChatId {
    Integer(i64),
    String(String),
}



#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum PolymorphReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}



#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum PolymorphFromChatId {
    Integer(i64),
    String(String),
}/// Use this method to receive incoming updates using long polling (wiki). An Array
/// of Update objects is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than
    /// the highest among the identifiers of previously received updates. By
    /// default, updates starting with the earliest unconfirmed update are returned.
    /// An update is considered confirmed as soon as getUpdates is called with an
    /// offset higher than its update_id. The negative offset can be specified to
    /// retrieve updates starting from -offset update from the end of the updates
    /// queue. All previous updates will forgotten.
    pub offset: Option<i64>,
    /// Limits the number of updates to be retrieved. Values between 1—100 are
    /// accepted. Defaults to 100.
    pub limit: Option<i64>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short
    /// polling. Should be positive, short polling should be used for testing
    /// purposes only.
    pub timeout: Option<i64>,
    /// List the types of updates you want your bot to receive. For example, specify
    /// [“message”, “edited_channel_post”, “callback_query”] to only receive updates
    /// of these types. See Update for a complete list of available update types.
    /// Specify an empty list to receive all updates regardless of type (default).
    /// If not specified, the previous setting will be used.Please note that this
    /// parameter doesn't affect updates created before the call to the getUpdates,
    /// so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to specify a url and receive incoming updates via an outgoing
/// webhook. Whenever there is an update for the bot, we will send an HTTPS POST
/// request to the specified url, containing a JSON-serialized Update. In case of an
/// unsuccessful request, we will give up after a reasonable amount of attempts.
/// Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook
    /// integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can
    /// be checked. See our self-signed guide for details.
    pub certificate: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for
    /// update delivery, 1-100. Defaults to 40. Use lower values to limit the load
    /// on your bot‘s server, and higher values to increase your bot’s throughput.
    pub max_connections: Option<i64>,
    /// List the types of updates you want your bot to receive. For example, specify
    /// [“message”, “edited_channel_post”, “callback_query”] to only receive updates
    /// of these types. See Update for a complete list of available update types.
    /// Specify an empty list to receive all updates regardless of type (default).
    /// If not specified, the previous setting will be used.Please note that this
    /// parameter doesn't affect updates created before the call to the setWebhook,
    /// so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to remove webhook integration if you decide to switch back to
/// getUpdates. Returns True on success. Requires no parameters.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeleteWebhook {
}

/// Use this method to get current webhook status. Requires no parameters. On
/// success, returns a WebhookInfo object. If the bot is using getUpdates, will
/// return an object with the url field empty.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetWebhookInfo {
}

/// A simple method for testing your bot's auth token. Requires no parameters.
/// Returns basic information about the bot in form of a User object.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetMe {
}

/// Use this method to send text messages. On success, the sent Message is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Text of the message to be sent
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    pub parse_mode: Option<String>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to forward messages of any kind. On success, the sent Message is
/// returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Unique identifier for the chat where the original message was sent (or
    /// channel username in the format @channelusername)
    pub from_chat_id: PolymorphFromChatId,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
}

/// Use this method to send photos. On success, the sent Message is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to
    /// get a photo from the Internet, or upload a new photo using multipart/form-
    /// data. More info on Sending Files »
    pub photo: String,
    /// Photo caption (may also be used when resending photos by file_id), 0-1024
    /// characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send audio files, if you want Telegram clients to display
/// them in the music player. Your audio must be in the .mp3 format. On success, the
/// sent Message is returned. Bots can currently send audio files of up to 50 MB in
/// size, this limit may be changed in the future.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendAudio {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Audio file to send. Pass a file_id as String to send an audio file that
    /// exists on the Telegram servers (recommended), pass an HTTP URL as a String
    /// for Telegram to get an audio file from the Internet, or upload a new one
    /// using multipart/form-data. More info on Sending Files »
    pub audio: String,
    /// Audio caption, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Duration of the audio in seconds
    pub duration: Option<i64>,
    /// Performer
    pub performer: Option<String>,
    /// Track name
    pub title: Option<String>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the
    /// file is supported server-side. The thumbnail should be in JPEG format and
    /// less than 200 kB in size. A thumbnail‘s width and height should not exceed
    /// 320. Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More info on Sending Files »
    pub thumb: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send general files. On success, the sent Message is returned.
/// Bots can currently send files of any type of up to 50 MB in size, this limit may
/// be changed in the future.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendDocument {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// File to send. Pass a file_id as String to send a file that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to
    /// get a file from the Internet, or upload a new one using multipart/form-data.
    /// More info on Sending Files »
    pub document: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the
    /// file is supported server-side. The thumbnail should be in JPEG format and
    /// less than 200 kB in size. A thumbnail‘s width and height should not exceed
    /// 320. Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More info on Sending Files »
    pub thumb: Option<String>,
    /// Document caption (may also be used when resending documents by file_id),
    /// 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send video files, Telegram clients support mp4 videos (other
/// formats may be sent as Document). On success, the sent Message is returned. Bots
/// can currently send video files of up to 50 MB in size, this limit may be changed
/// in the future.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Video to send. Pass a file_id as String to send a video that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to
    /// get a video from the Internet, or upload a new video using multipart/form-
    /// data. More info on Sending Files »
    pub video: String,
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    /// Video width
    pub width: Option<i64>,
    /// Video height
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the
    /// file is supported server-side. The thumbnail should be in JPEG format and
    /// less than 200 kB in size. A thumbnail‘s width and height should not exceed
    /// 320. Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More info on Sending Files »
    pub thumb: Option<String>,
    /// Video caption (may also be used when resending videos by file_id), 0-1024
    /// characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Pass True, if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without
/// sound). On success, the sent Message is returned. Bots can currently send
/// animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendAnimation {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Animation to send. Pass a file_id as String to send an animation that exists
    /// on the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get an animation from the Internet, or upload a new animation
    /// using multipart/form-data. More info on Sending Files »
    pub animation: String,
    /// Duration of sent animation in seconds
    pub duration: Option<i64>,
    /// Animation width
    pub width: Option<i64>,
    /// Animation height
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the
    /// file is supported server-side. The thumbnail should be in JPEG format and
    /// less than 200 kB in size. A thumbnail‘s width and height should not exceed
    /// 320. Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More info on Sending Files »
    pub thumb: Option<String>,
    /// Animation caption (may also be used when resending animation by file_id),
    /// 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send audio files, if you want Telegram clients to display the
/// file as a playable voice message. For this to work, your audio must be in an
/// .ogg file encoded with OPUS (other formats may be sent as Audio or Document). On
/// success, the sent Message is returned. Bots can currently send voice messages of
/// up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendVoice {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Audio file to send. Pass a file_id as String to send a file that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data. More info on Sending Files »
    pub voice: String,
    /// Voice message caption, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Duration of the voice message in seconds
    pub duration: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1
/// minute long. Use this method to send video messages. On success, the sent
/// Message is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendVideoNote {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Video note to send. Pass a file_id as String to send a video note that
    /// exists on the Telegram servers (recommended) or upload a new video using
    /// multipart/form-data. More info on Sending Files ». Sending video notes by a
    /// URL is currently unsupported
    pub video_note: String,
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    /// Video width and height, i.e. diameter of the video message
    pub length: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the
    /// file is supported server-side. The thumbnail should be in JPEG format and
    /// less than 200 kB in size. A thumbnail‘s width and height should not exceed
    /// 320. Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file, so you
    /// can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More info on Sending Files »
    pub thumb: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send a group of photos or videos as an album. On success, an
/// array of the sent Messages is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// A JSON-serialized array describing photos and videos to be sent, must
    /// include 2–10 items
    pub media: Vec<InputMedia>,
    /// Sends the messages silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: Option<bool>,
    /// If the messages are a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
}

/// Use this method to send point on the map. On success, the sent Message is
/// returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// Period in seconds for which the location will be updated (see Live
    /// Locations, should be between 60 and 86400.
    pub live_period: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to edit live location messages. A location can be edited until
/// its live_period expires or editing is explicitly disabled by a call to
/// stopMessageLiveLocation. On success, if the edited message was sent by the bot,
/// the edited Message is returned, otherwise True is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EditMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: Option<PolymorphChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to
    /// edit
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop updating a live location message before live_period
/// expires. On success, if the message was sent by the bot, the sent Message is
/// returned, otherwise True is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StopMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: Option<PolymorphChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message
    /// with live location to stop
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to send information about a venue. On success, the sent Message
/// is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example,
    /// “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send phone contacts. On success, the sent Message is
/// returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove keyboard or to force
    /// a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to send a native poll. A native poll can't be sent to a private
/// chat. On success, the sent Message is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername). A native poll can't be sent to a private chat.
    pub chat_id: PolymorphChatId,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method when you need to tell the user that something is happening on
/// the bot's side. The status is set for 5 seconds or less (when a message arrives
/// from your bot, Telegram clients clear its typing status). Returns True on
/// success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Type of action to broadcast. Choose one, depending on what the user is about
    /// to receive: typing for text messages, upload_photo for photos, record_video
    /// or upload_video for videos, record_audio or upload_audio for audio files,
    /// upload_document for general files, find_location for location data,
    /// record_video_note or upload_video_note for video notes.
    pub action: String,
}

/// Use this method to get a list of profile pictures for a user. Returns a
/// UserProfilePhotos object.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos
    /// are returned.
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted. Defaults to 100.
    pub limit: Option<i64>,
}

/// Use this method to get basic info about a file and prepare it for downloading.
/// For the moment, bots can download files of up to 20MB in size. On success, a
/// File object is returned. The file can then be downloaded via the link
/// https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken
/// from the response. It is guaranteed that the link will be valid for at least 1
/// hour. When the link expires, a new one can be requested by calling getFile
/// again.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

/// Use this method to kick a user from a group, a supergroup or a channel. In the
/// case of supergroups and channels, the user will not be able to return to the
/// group on their own using invite links, etc., unless unbanned first. The bot must
/// be an administrator in the chat for this to work and must have the appropriate
/// admin rights. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct KickChatMember {
    /// Unique identifier for the target group or username of the target supergroup
    /// or channel (in the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time. If user is banned for more
    /// than 366 days or less than 30 seconds from the current time they are
    /// considered to be banned forever
    pub until_date: Option<i64>,
}

/// Use this method to unban a previously kicked user in a supergroup or channel.
/// The user will not return to the group or channel automatically, but will be able
/// to join via link, etc. The bot must be an administrator for this to work.
/// Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup
    /// or channel (in the format @username)
    pub chat_id: PolymorphChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

/// Use this method to restrict a user in a supergroup. The bot must be an
/// administrator in the supergroup for this to work and must have the appropriate
/// admin rights. Pass True for all boolean parameters to lift restrictions from a
/// user. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup
    /// (in the format @supergroupusername)
    pub chat_id: PolymorphChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when restrictions will be lifted for the user, unix time. If user is
    /// restricted for more than 366 days or less than 30 seconds from the current
    /// time, they are considered to be restricted forever
    pub until_date: Option<i64>,
    /// Pass True, if the user can send text messages, contacts, locations and
    /// venues
    pub can_send_messages: Option<bool>,
    /// Pass True, if the user can send audios, documents, photos, videos, video
    /// notes and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    /// Pass True, if the user can send animations, games, stickers and use inline
    /// bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    /// Pass True, if the user may add web page previews to their messages, implies
    /// can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
}

/// Use this method to promote or demote a user in a supergroup or a channel. The
/// bot must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Pass False for all boolean parameters to demote a
/// user. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass True, if the administrator can change chat title, photo and other
    /// settings
    pub can_change_info: Option<bool>,
    /// Pass True, if the administrator can create channel posts, channels only
    pub can_post_messages: Option<bool>,
    /// Pass True, if the administrator can edit messages of other users and can pin
    /// messages, channels only
    pub can_edit_messages: Option<bool>,
    /// Pass True, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// Pass True, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// Pass True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    /// Pass True, if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
    /// Pass True, if the administrator can add new administrators with a subset of
    /// his own privileges or demote administrators that he has promoted, directly
    /// or indirectly (promoted by administrators that were appointed by him)
    pub can_promote_members: Option<bool>,
}

/// Use this method to generate a new invite link for a chat; any previously
/// generated link is revoked. The bot must be an administrator in the chat for this
/// to work and must have the appropriate admin rights. Returns the new invite link
/// as String on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method to set a new profile photo for the chat. Photos can't be changed
/// for private chats. The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: String,
}

/// Use this method to delete a chat photo. Photos can't be changed for private
/// chats. The bot must be an administrator in the chat for this to work and must
/// have the appropriate admin rights. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method to change the title of a chat. Titles can't be changed for
/// private chats. The bot must be an administrator in the chat for this to work and
/// must have the appropriate admin rights. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// New chat title, 1-255 characters
    pub title: String,
}

/// Use this method to change the description of a supergroup or a channel. The bot
/// must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// New chat description, 0-255 characters
    pub description: Option<String>,
}

/// Use this method to pin a message in a group, a supergroup, or a channel. The bot
/// must be an administrator in the chat for this to work and must have the
/// ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin
/// right in the channel. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Pass True, if it is not necessary to send a notification to all chat members
    /// about the new pinned message. Notifications are always disabled in channels.
    pub disable_notification: Option<bool>,
}

/// Use this method to unpin a message in a group, a supergroup, or a channel. The
/// bot must be an administrator in the chat for this to work and must have the
/// ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin
/// right in the channel. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method for your bot to leave a group, supergroup or channel. Returns
/// True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup
    /// or channel (in the format @channelusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method to get up to date information about the chat (current name of
/// the user for one-on-one conversations, current username of a user, group or
/// channel, etc.). Returns a Chat object on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup
    /// or channel (in the format @channelusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method to get a list of administrators in a chat. On success, returns
/// an Array of ChatMember objects that contains information about all chat
/// administrators except other bots. If the chat is a group or a supergroup and no
/// administrators were appointed, only the creator will be returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup
    /// or channel (in the format @channelusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat or username of the target supergroup
    /// or channel (in the format @channelusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method to get information about a member of a chat. Returns a
/// ChatMember object on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup
    /// or channel (in the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

/// Use this method to set a new group sticker set for a supergroup. The bot must be
/// an administrator in the chat for this to work and must have the appropriate
/// admin rights. Use the field can_set_sticker_set optionally returned in getChat
/// requests to check if the bot can use this method. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup
    /// (in the format @supergroupusername)
    pub chat_id: PolymorphChatId,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

/// Use this method to delete a group sticker set from a supergroup. The bot must be
/// an administrator in the chat for this to work and must have the appropriate
/// admin rights. Use the field can_set_sticker_set optionally returned in getChat
/// requests to check if the bot can use this method. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup
    /// (in the format @supergroupusername)
    pub chat_id: PolymorphChatId,
}

/// Use this method to send answers to callback queries sent from inline keyboards.
/// The answer will be displayed to the user as a notification at the top of the
/// chat screen or as an alert. On success, True is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the
    /// user, 0-200 characters
    pub text: Option<String>,
    /// If true, an alert will be shown by the client instead of a notification at
    /// the top of the chat screen. Defaults to false.
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a Game and
    /// accepted the conditions via @Botfather, specify the URL that opens your game
    /// – note that this will only work if the query comes from a callback_game
    /// button.Otherwise, you may use links like t.me/your_bot?start=XXXX that open
    /// your bot with a parameter.
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query
    /// may be cached client-side. Telegram apps will support caching starting in
    /// version 3.14. Defaults to 0.
    pub cache_time: Option<i64>,
}

/// Use this method to edit text and game messages. On success, if edited message is
/// sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EditMessageText {
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: Option<PolymorphChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to
    /// edit
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
    /// New text of the message
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    pub parse_mode: Option<String>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit captions of messages. On success, if edited message is
/// sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EditMessageCaption {
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: Option<PolymorphChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to
    /// edit
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
    /// New caption of the message
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit animation, audio, document, photo, or video messages. If
/// a message is a part of a message album, then it can be edited only to a photo or
/// a video. Otherwise, message type can be changed arbitrarily. When inline message
/// is edited, new file can't be uploaded. Use previously uploaded file via its
/// file_id or specify a URL. On success, if the edited message was sent by the bot,
/// the edited Message is returned, otherwise True is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EditMessageMedia {
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: Option<PolymorphChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to
    /// edit
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit only the reply markup of messages. On success, if edited
/// message is sent by the bot, the edited Message is returned, otherwise True is
/// returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EditMessageReplyMarkup {
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: Option<PolymorphChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to
    /// edit
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop a poll which was sent by the bot. On success, the
/// stopped Poll with the final results is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    /// A JSON-serialized object for a new message inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to delete a message, including service messages, with the
/// following limitations:- A message can only be deleted if it was sent less than
/// 48 hours ago.- Bots can delete outgoing messages in private chats, groups, and
/// supergroups.- Bots can delete incoming messages in private chats.- Bots granted
/// can_post_messages permissions can delete outgoing messages in channels.- If the
/// bot is an administrator of a group, it can delete any message there.- If the bot
/// has can_delete_messages permission in a supergroup or a channel, it can delete
/// any message there.Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeleteMessage {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Identifier of the message to delete
    pub message_id: i64,
}

/// Use this method to send .webp stickers. On success, the sent Message is
/// returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in
    /// the format @channelusername)
    pub chat_id: PolymorphChatId,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to
    /// get a .webp file from the Internet, or upload a new one using
    /// multipart/form-data. More info on Sending Files »
    pub sticker: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard or to
    /// force a reply from the user.
    pub reply_markup: Option<PolymorphReplyMarkup>,
}

/// Use this method to get a sticker set. On success, a StickerSet object is
/// returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

/// Use this method to upload a .png file with a sticker for later use in
/// createNewStickerSet and addStickerToSet methods (can be used multiple times).
/// Returns the uploaded File on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions
    /// must not exceed 512px, and either width or height must be exactly 512px.
    /// More info on Sending Files »
    pub png_sticker: String,
}

/// Use this method to create new sticker set owned by a user. The bot will be able
/// to edit the created sticker set. Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g.,
    /// animals). Can contain only english letters, digits and underscores. Must
    /// begin with a letter, can't contain consecutive underscores and must end in
    /// “_by_<bot username>”. <bot_username> is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions
    /// must not exceed 512px, and either width or height must be exactly 512px.
    /// Pass a file_id as a String to send a file that already exists on the
    /// Telegram servers, pass an HTTP URL as a String for Telegram to get a file
    /// from the Internet, or upload a new one using multipart/form-data. More info
    /// on Sending Files »
    pub png_sticker: String,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// Pass True, if a set of mask stickers should be created
    pub contains_masks: Option<bool>,
    /// A JSON-serialized object for position where the mask should be placed on
    /// faces
    pub mask_position: Option<MaskPosition>,
}

/// Use this method to add a new sticker to a set created by the bot. Returns True
/// on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions
    /// must not exceed 512px, and either width or height must be exactly 512px.
    /// Pass a file_id as a String to send a file that already exists on the
    /// Telegram servers, pass an HTTP URL as a String for Telegram to get a file
    /// from the Internet, or upload a new one using multipart/form-data. More info
    /// on Sending Files »
    pub png_sticker: String,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on
    /// faces
    pub mask_position: Option<MaskPosition>,
}

/// Use this method to move a sticker in a set created by the bot to a specific
/// position . Returns True on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i64,
}

/// Use this method to delete a sticker from a set created by the bot. Returns True
/// on success.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}

/// Use this method to send answers to an inline query. On success, True is
/// returned.No more than 50 results per query are allowed.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query
    /// may be cached on the server. Defaults to 300.
    pub cache_time: Option<i64>,
    /// Pass True, if results may be cached on the server side only for the user
    /// that sent the query. By default, results may be returned to any user who
    /// sends the same query
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same
    /// text to receive more results. Pass an empty string if there are no more
    /// results or if you don‘t support pagination. Offset length can’t exceed 64
    /// bytes.
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches
    /// the user to a private chat with the bot and sends the bot a start message
    /// with the parameter switch_pm_parameter
    pub switch_pm_text: Option<String>,
    /// Deep-linking parameter for the /start message sent to the bot when user
    /// presses the switch button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are
    /// allowed.Example: An inline bot that sends YouTube videos can ask the user to
    /// connect the bot to their YouTube account to adapt search results
    /// accordingly. To do this, it displays a ‘Connect your YouTube account’ button
    /// above the results, or even before showing any. The user presses the button,
    /// switches to a private chat with the bot and, in doing so, passes a start
    /// parameter that instructs the bot to return an oauth link. Once done, the bot
    /// can offer a switch_inline button so that the user can easily return to the
    /// chat where they wanted to use the bot's inline capabilities.
    pub switch_pm_parameter: Option<String>,
}

/// Use this method to send invoices. On success, the sent Message is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendInvoice {
    /// Unique identifier for the target private chat
    pub chat_id: i64,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the
    /// user, use for your internal processes.
    pub payload: String,
    /// Payments provider token, obtained via Botfather
    pub provider_token: String,
    /// Unique deep-linking parameter that can be used to generate this invoice when
    /// used as a start parameter
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a list of components (e.g. product price, tax, discount,
    /// delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// JSON-encoded data about the invoice, which will be shared with the payment
    /// provider. A detailed description of required fields should be provided by
    /// the payment provider.
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a
    /// marketing image for a service. People like it better when they see what they
    /// are paying for.
    pub photo_url: Option<String>,
    /// Photo size
    pub photo_size: Option<i64>,
    /// Photo width
    pub photo_width: Option<i64>,
    /// Photo height
    pub photo_height: Option<i64>,
    /// Pass True, if you require the user's full name to complete the order
    pub need_name: Option<bool>,
    /// Pass True, if you require the user's phone number to complete the order
    pub need_phone_number: Option<bool>,
    /// Pass True, if you require the user's email address to complete the order
    pub need_email: Option<bool>,
    /// Pass True, if you require the user's shipping address to complete the order
    pub need_shipping_address: Option<bool>,
    /// Pass True, if user's phone number should be sent to provider
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True, if user's email address should be sent to provider
    pub send_email_to_provider: Option<bool>,
    /// Pass True, if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total
    /// price' button will be shown. If not empty, the first button must be a Pay
    /// button.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// If you sent an invoice requesting a shipping address and the parameter
/// is_flexible was specified, the Bot API will send an Update with a shipping_query
/// field to the bot. Use this method to reply to shipping queries. On success, True
/// is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if
    /// there are any problems (for example, if delivery to the specified address is
    /// not possible)
    pub ok: bool,
    /// Required if ok is True. A JSON-serialized array of available shipping
    /// options.
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that explains
    /// why it is impossible to complete the order (e.g. "Sorry, delivery to your
    /// desired address is unavailable'). Telegram will display this message to the
    /// user.
    pub error_message: Option<String>,
}

/// Once the user has confirmed their payment and shipping details, the Bot API
/// sends the final confirmation in the form of an Update with the field
/// pre_checkout_query. Use this method to respond to such pre-checkout queries. On
/// success, True is returned. Note: The Bot API must receive an answer within 10
/// seconds after the pre-checkout query was sent.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the
    /// bot is ready to proceed with the order. Use False if there are any problems.
    pub ok: bool,
    /// Required if ok is False. Error message in human readable form that explains
    /// the reason for failure to proceed with the checkout (e.g. "Sorry, somebody
    /// just bought the last of our amazing black T-shirts while you were busy
    /// filling out your payment details. Please choose a different color or
    /// garment!"). Telegram will display this message to the user.
    pub error_message: Option<String>,
}

/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Short name of the game, serves as the unique identifier for the game. Set up
    /// your games via Botfather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// A JSON-serialized object for an inline keyboard. If empty, one ‘Play
    /// game_title’ button will be shown. If not empty, the first button must launch
    /// the game.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to set the score of the specified user in a game. On success, if
/// the message was sent by the bot, returns the edited Message, otherwise returns
/// True. Returns an error, if the new score is not greater than the user's current
/// score in the chat and force is False.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: i64,
    /// Pass True, if the high score is allowed to decrease. This can be useful when
    /// fixing mistakes or banning cheaters
    pub force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to include
    /// the current scoreboard
    pub disable_edit_message: Option<bool>,
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent
    /// message
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
}

/// Use this method to get data for high score tables. Will return the score of the
/// specified user and several of his neighbors in a game. On success, returns an
/// Array of GameHighScore objects.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i64,
    /// Required if inline_message_id is not specified. Unique identifier for the
    /// target chat
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent
    /// message
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    pub inline_message_id: Option<String>,
}

