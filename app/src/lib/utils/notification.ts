enum NotificationLevel {
    Info,
    Warning,
    Error
}

class NotificationModel {
    title: string;
    body: string;
    level: NotificationLevel;

    constructor(title: string, body: string, level: NotificationLevel) {
        this.title = title;
        this.body = body;
        this.level = level;
    }
}

function getNotificationColour(level: NotificationLevel): string {
    switch (level) {
        case NotificationLevel.Error:
            return "#bf616a";
        case NotificationLevel.Warning:
            return "#ebcb8b";
        case NotificationLevel.Info:
            return "#a3be8c";
    }
}

export { NotificationModel, NotificationLevel, getNotificationColour }
