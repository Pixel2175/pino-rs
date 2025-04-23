use std::{process::Command,thread,time::Duration,error::Error};
use dbus::{arg::{RefArg,PropMap},blocking::{Connection,stdintf::org_freedesktop_dbus::Properties}};
use dbus_crossroads::{Crossroads, Context};

const NOTIFICATIONS_INTERFACE: &str = "org.freedesktop.Notifications";
const MEDIA_PLAYER_PATH: &str = "/org/mpris/MediaPlayer2";
const MEDIA_PLAYER_INTERFACE: &str = "org.mpris.MediaPlayer2.Player";

const MEDIA_PLAYERS: &[&str] = &[
    "spotify",
    "mpv",
    "vlc",
    "chromium",
    "firefox",
    "brave",
];

#[derive(Debug)]
struct NotificationData {
    source: String,
    app_name: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: PropMap,
}

impl NotificationData {
    fn print(&self) {
        // Send to pino app
        let summary = self.summary.clone();
        let body = self.body.clone();

        thread::spawn(move || {
            let _ = Command::new("pino")
                .arg("-t")
                .arg(summary)
                .arg("-m")
                .arg(body)
                .output()
                .expect("Failed to execute command");
        });


        println!("\n=== {} Notification ===", self.source);
        println!("App: {}", self.app_name);
        println!("Summary: {}", self.summary);
        if !self.body.is_empty() {
            println!("Body: {}", self.body);
        }
        if !self.actions.is_empty() {
            println!("Actions: {}", self.actions.join(", "));
        }

        // Print relevant hints if they exist
        if !self.hints.is_empty() {
            println!("Additional Info:");
            for (key, value) in &self.hints {
                if let Some(val) = value.as_str() {
                    println!("  {}: {}", key, val);
                }
            }
        }
        println!("========================\n");
    }
}

// Media player information structure
#[derive(Debug)]
struct MediaInfo {
    player: String,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    status: Option<String>,
}

impl MediaInfo {
    fn to_notification(&self) -> NotificationData {
        let title = self.title.clone().unwrap_or_default();
        let artist = self.artist.clone().unwrap_or_default();
        let album = self.album.clone().unwrap_or_default();
        let status = self.status.clone().unwrap_or_default();

        let summary = format!("{} - {}", self.player, status);
        let body = if !artist.is_empty() && !title.is_empty() {
            format!("{} - {}", artist, title)
        } else if !title.is_empty() {
            title
        } else {
            "Now Playing".to_string()
        };

        let mut hints = PropMap::new();
        if !album.is_empty() {
            hints.insert("album".to_string(), dbus::arg::Variant(Box::new(album)));
        }

        NotificationData {
            source: "Media".to_string(),
            app_name: self.player.clone(),
            summary,
            body,
            actions: vec![],
            hints,
        }
    }

    fn print(&self) {
        let notification = self.to_notification();
        notification.print();
    }
}

// Get media information from a specific player
fn get_media_info(conn: &Connection, player: &str) -> Result<Option<MediaInfo>, Box<dyn Error>> {
    let proxy = conn.with_proxy(
        format!("org.mpris.MediaPlayer2.{}", player),
        MEDIA_PLAYER_PATH,
        Duration::from_millis(500),
    );

    // Try to get metadata and playback status
    let metadata_result: Result<PropMap, dbus::Error> = proxy.get(MEDIA_PLAYER_INTERFACE, "Metadata");
    let status_result: Result<String, dbus::Error> = proxy.get(MEDIA_PLAYER_INTERFACE, "PlaybackStatus");

    match metadata_result {
        Ok(metadata) => {
            let title = metadata.get("xesam:title")
                .and_then(|v| v.as_str())
                .map(String::from);
            let artist = metadata.get("xesam:artist")
                .and_then(|v| v.as_iter())
                .and_then(|mut iter| iter.next())
                .and_then(|v| v.as_str())
                .map(String::from);
            let album = metadata.get("xesam:album")
                .and_then(|v| v.as_str())
                .map(String::from);
            let status = status_result.ok();

            Ok(Some(MediaInfo {
                player: player.to_string(),
                title,
                artist,
                album,
                status,
            }))
        }
        Err(_) => Ok(None),
    }
}

fn monitor_media_players(conn: &Connection) {
    for player in MEDIA_PLAYERS {
        if let Ok(Some(info)) = get_media_info(conn, player) {
            info.print();
        }
    }
}

fn process_discord_notification(summary: &str, body: &str) -> NotificationData {
    NotificationData {
        source: "Discord".to_string(),
        app_name: "Discord".to_string(),
        summary: format!("Discord: {}", summary),
        body: body.to_string(),
        actions: vec![],
        hints: PropMap::new(),
    }
}

fn setup_notification_interface(cr: &mut Crossroads) -> dbus_crossroads::IfaceToken<()> {
    cr.register(NOTIFICATIONS_INTERFACE, |b| {
        b.method(
            "Notify",
            ("app_name", "replaces_id", "app_icon", "summary", "body", "actions", "hints", "expire_timeout"),
            ("id",),
            move |_ctx: &mut Context,
                  _: &mut (),
                  (app_name, replaces_id, _app_icon, summary, body, actions, hints, _expire_timeout):
                  (String, u32, String, String, String, Vec<String>, PropMap, i32)| {

                let notification = if app_name.to_lowercase().contains("discord") {
                    process_discord_notification(&summary, &body)
                } else {
                    NotificationData {
                        source: "System".to_string(),
                        app_name: app_name.clone(),
                        summary,
                        body,
                        actions,
                        hints,
                    }
                };

                notification.print();

                if MEDIA_PLAYERS.iter().any(|&p| app_name.to_lowercase().contains(p)) {
                    if let Ok(conn) = Connection::new_session() {
                        monitor_media_players(&conn);
                    }
                }

                Ok((replaces_id,))
            },
        );

        b.method(
            "GetCapabilities",
            (),
            ("capabilities",),
            |_: &mut Context, _: &mut (), _: ()| {
                Ok((vec![
                    "actions",
                    "body",
                    "body-markup",
                    "icon-static",
                ],))
            },
        );

        b.method(
            "CloseNotification",
            ("id",),
            (),
            |_: &mut Context, _: &mut (), (_id,): (u32,)| {
                Ok(())
            },
        );

        b.method(
            "GetServerInformation",
            (),
            ("name", "vendor", "version", "spec_version"),
            |_: &mut Context, _: &mut (), _: ()| {
                Ok((
                    "ComprehensiveNotificationServer",
                    "Custom",
                    "1.0",
                    "1.2",
                ))
            },
        );
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::new_session()?;
    conn.request_name(NOTIFICATIONS_INTERFACE, false, true, false)?;

    println!("\nWaiting for notifications...\n");

    let mut cr = Crossroads::new();
    let iface_token = setup_notification_interface(&mut cr);
    cr.insert("/org/freedesktop/Notifications", &[iface_token], ());

    thread::spawn(move || {
        loop {
            if let Ok(conn) = Connection::new_session() {
                monitor_media_players(&conn);
            }
            thread::sleep(Duration::from_secs(5));
        }
    });

    cr.serve(&conn)?;

    Ok(())
}
