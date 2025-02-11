import Icon from "./Icon.svelte";

enum Icons {
    Home,
    Close,
    Minimise,
    Library,
    Settings,
    Notification,
    DocSearch,
    Plus,
    Down
}

function getIconPath(icon: Icons): string {
    switch (icon) {
        case Icons.Home:
            return "M240-200h147.69v-203.08q0-13.73 9.29-23.02 9.29-9.28 23.02-9.28h120q13.73 0 23.02 9.28 9.29 9.29 9.29 23.02V-200H720v-347.69q0-6.16-2.69-11.16t-7.31-8.84L494.62-730q-6.16-5.38-14.62-5.38-8.46 0-14.62 5.38L250-567.69q-4.62 3.84-7.31 8.84-2.69 5-2.69 11.16V-200Zm-40 0v-347.69q0-15.35 6.87-29.08 6.86-13.73 18.98-22.61l215.38-163.08q16.91-12.92 38.65-12.92t38.89 12.92l215.38 163.08q12.12 8.88 18.98 22.61 6.87 13.73 6.87 29.08V-200q0 16.08-11.96 28.04T720-160H564.62q-13.74 0-23.02-9.29-9.29-9.29-9.29-23.02v-203.07H427.69v203.07q0 13.73-9.29 23.02-9.28 9.29-23.02 9.29H240q-16.08 0-28.04-11.96T200-200Zm280-268.46Z";
        case Icons.Minimise:
            return "M260-460q-8.5 0-14.25-5.76T240-480.03q0-8.51 5.75-14.24T260-500h440q8.5 0 14.25 5.76t5.75 14.27q0 8.51-5.75 14.24T700-460H260Z";
        case Icons.Close:
            return "M480-451.69 270.15-241.85q-5.61 5.62-13.77 6-8.15.39-14.53-6-6.39-6.38-6.39-14.15 0-7.77 6.39-14.15L451.69-480 241.85-689.85q-5.62-5.61-6-13.77-.39-8.15 6-14.53 6.38-6.39 14.15-6.39 7.77 0 14.15 6.39L480-508.31l209.85-209.84q5.61-5.62 13.77-6 8.15-.39 14.53 6 6.39 6.38 6.39 14.15 0 7.77-6.39 14.15L508.31-480l209.84 209.85q5.62 5.61 6 13.77.39 8.15-6 14.53-6.38 6.39-14.15 6.39-7.77 0-14.15-6.39L480-451.69Z";
        case Icons.Library:
            return "M420-420h99.23q8.54 0 14.27-5.73t5.73-14.27q0-8.54-5.73-14.27T519.23-460H420q-8.54 0-14.27 5.73T400-440q0 8.54 5.73 14.27T420-420Zm0-120h238.46q8.54 0 14.27-5.73t5.73-14.27q0-8.54-5.73-14.27T658.46-580H420q-8.54 0-14.27 5.73T400-560q0 8.54 5.73 14.27T420-540Zm0-120h238.46q8.54 0 14.27-5.73t5.73-14.27q0-8.54-5.73-14.27T658.46-700H420q-8.54 0-14.27 5.73T400-680q0 8.54 5.73 14.27T420-660Zm-95.38 380q-27.62 0-46.12-18.5Q260-317 260-344.62v-430.76q0-27.62 18.5-46.12Q297-840 324.62-840h430.76q27.62 0 46.12 18.5Q820-803 820-775.38v430.76q0 27.62-18.5 46.12Q783-280 755.38-280H324.62Zm0-40h430.76q9.24 0 16.93-7.69 7.69-7.69 7.69-16.93v-430.76q0-9.24-7.69-16.93-7.69-7.69-16.93-7.69H324.62q-9.24 0-16.93 7.69-7.69 7.69-7.69 16.93v430.76q0 9.24 7.69 16.93 7.69 7.69 16.93 7.69Zm-120 160q-27.62 0-46.12-18.5Q140-197 140-224.61v-450.77q0-8.54 5.73-14.27t14.27-5.73q8.54 0 14.27 5.73t5.73 14.27v450.77q0 9.23 7.69 16.92 7.69 7.69 16.93 7.69h450.76q8.54 0 14.27 5.73t5.73 14.27q0 8.54-5.73 14.27T655.38-160H204.62ZM300-800v480-480Z";
        case Icons.Settings:
            return "M438.38-120q-13.92 0-24.19-9.15-10.27-9.16-12.73-22.85l-10.54-83.69q-19.15-5.77-41.42-18.16-22.27-12.38-37.88-26.53L235-247.46q-12.69 5.61-25.77 1.23-13.08-4.39-20.15-16.62l-43.16-74.3q-7.07-12.23-4.15-25.54 2.92-13.31 13.92-21.85l66.85-50q-1.77-10.84-2.92-22.34-1.16-11.5-1.16-22.35 0-10.08 1.16-21.19 1.15-11.12 2.92-25.04l-66.85-50q-11-8.54-13.54-22.23-2.53-13.69 4.54-25.93l42.39-72q7.07-11.46 20.15-16.23 13.08-4.77 25.77.85l75.85 32.15q17.92-14.92 38.77-26.92 20.84-12 40.53-18.54L401.46-808q2.46-13.69 12.73-22.85 10.27-9.15 24.19-9.15h83.24q13.92 0 24.19 9.15 10.27 9.16 12.73 22.85l10.54 84.46q23 8.08 40.65 18.54 17.65 10.46 36.35 26.15L725.77-711q12.69-5.62 25.77-.85 13.08 4.77 20.15 16.23l42.39 72.77q7.07 12.23 4.15 25.54-2.92 13.31-13.92 21.85l-69.93 52.31q3.31 12.38 3.7 22.73.38 10.34.38 20.42 0 9.31-.77 19.65-.77 10.35-3.54 25.04l67.62 50.77q11 8.54 14.31 21.85 3.3 13.31-3.77 25.54l-42.62 73.53q-7.07 12.24-20.54 16.62-13.46 4.38-26.15-1.23l-76.92-32.92q-18.7 15.69-37.62 26.92-18.92 11.23-39.38 17.77L558.54-152q-2.46 13.69-12.73 22.85-10.27 9.15-24.19 9.15h-83.24Zm1.62-40h78.23L533-268.31q30.23-8 54.42-21.96 24.2-13.96 49.27-38.27L736.46-286l39.77-68-87.54-65.77q5-17.08 6.62-31.42 1.61-14.35 1.61-28.81 0-15.23-1.61-28.81-1.62-13.57-6.62-29.88L777.77-606 738-674l-102.08 42.77q-18.15-19.92-47.73-37.35-29.57-17.42-55.96-23.11L520-800h-79.77l-12.46 107.54q-30.23 6.46-55.58 20.81-25.34 14.34-50.42 39.42L222-674l-39.77 68L269-541.23q-5 13.46-7 29.23t-2 32.77q0 15.23 2 30.23t6.23 29.23l-86 65.77L222-286l99-42q23.54 23.77 48.88 38.12 25.35 14.34 57.12 22.34L440-160Zm38.92-220q41.85 0 70.93-29.08 29.07-29.07 29.07-70.92t-29.07-70.92Q520.77-580 478.92-580q-42.07 0-71.04 29.08-28.96 29.07-28.96 70.92t28.96 70.92Q436.85-380 478.92-380ZM480-480Z";
        case Icons.Notification:
            return "M220-209.23q-8.5 0-14.25-5.76T200-229.26q0-8.51 5.75-14.24t14.25-5.73h44.62v-316.92q0-78.39 49.61-137.89 49.62-59.5 125.77-74.11V-800q0-16.67 11.64-28.33Q463.28-840 479.91-840t28.36 11.67Q520-816.67 520-800v21.85q76.15 14.61 125.77 74.11 49.61 59.5 49.61 137.89v316.92H740q8.5 0 14.25 5.76 5.75 5.75 5.75 14.27 0 8.51-5.75 14.24T740-209.23H220Zm260-286.15Zm-.14 390.76q-26.71 0-45.59-18.98-18.89-18.98-18.89-45.63h129.24q0 26.85-19.03 45.73-19.02 18.88-45.73 18.88ZM304.62-249.23h350.76v-316.92q0-72.93-51.23-124.16-51.23-51.23-124.15-51.23-72.92 0-124.15 51.23-51.23 51.23-51.23 124.16v316.92Z";
        case Icons.DocSearch:
            return "M381.54-350.77q-95.92 0-162.58-66.65-66.65-66.66-66.65-162.58 0-95.92 66.65-162.58 66.66-66.65 162.58-66.65 95.92 0 162.58 66.65 66.65 66.66 66.65 162.58 0 41.69-14.77 80.69t-38.77 66.69l236.31 236.31q5.61 5.62 6 13.77.38 8.16-6 14.54-6.39 6.38-14.16 6.38-7.76 0-14.15-6.38L528.92-404.31q-30 25.54-69 39.54t-78.38 14Zm0-40q79.61 0 134.42-54.81 54.81-54.8 54.81-134.42 0-79.62-54.81-134.42-54.81-54.81-134.42-54.81-79.62 0-134.42 54.81-54.81 54.8-54.81 134.42 0 79.62 54.81 134.42 54.8 54.81 134.42 54.81Z";
        case Icons.Plus:
            return "M460-460H260q-8.5 0-14.25-5.76T240-480.03q0-8.51 5.75-14.24T260-500h200v-200q0-8.5 5.76-14.25t14.27-5.75q8.51 0 14.24 5.75T500-700v200h200q8.5 0 14.25 5.76t5.75 14.27q0 8.51-5.75 14.24T700-460H500v200q0 8.5-5.76 14.25T479.97-240q-8.51 0-14.24-5.75T460-260v-200Z";
        case Icons.Down:
            return "M480-384.85q-6.46 0-11.92-2.11-5.46-2.12-10.7-7.35L281.85-569.85q-5.62-5.61-6-13.77-.39-8.15 6-14.53 6.38-6.39 14.15-6.39 7.77 0 14.15 6.39L480-428.31l169.85-169.84q5.61-5.62 13.77-6 8.15-.39 14.53 6 6.39 6.38 6.39 14.15 0 7.77-6.39 14.15L502.62-394.31q-5.24 5.23-10.7 7.35-5.46 2.11-11.92 2.11Z";
    }
}

export { Icon, Icons, getIconPath };
