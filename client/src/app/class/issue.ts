/**
 * =============
 * Napi teendőim
 * => 2020.02.11.
 * =============
 * ( ) AKG-s vagyok-e? Kell-e képzés?
 * ( ) Skoda kamat befizetés
 * ( ) Kata+ befizetés
 * (*) jelentkezés Farmmix
 * (*) Peti jön 12.00-13.00 között
 * (*) Atisék jönnek?
 */

/**
 * New issue scheme
 */
export class IssueNew {
    constructor(
        public title: string = "",
        public description: string = ""
    ) { }
}

/**
 * Issue short scheme
 */
export class IssueShort {
    constructor(
        public id: string = "",
        public title: string = "",
        public description: string = "",
        public created_by: string = "",
        public date_created: Date = new Date(),
        public labels: Label[],
        public assigned_to: string = "",
        public comment_count: number = 0,
        public is_open: boolean = true
    ) { }
}

/**
 * Issue long scheme
 */
export class IssueLong {
    constructor(
        public id: string = "",
        public title: string = "",
        public description: string = "",
        public created_by: string = "",
        public date_created: Date = new Date(),
        public labels: Label[] = [new Label()],
        public assigned_to: string = "",
        public comment_count: number = 0,
        public events: Event[] = [new Event()],
        public followed_by: string[],
        public is_open: boolean = true
    ) { }
}

/**
 * Label scheme
 */
export class Label {
    constructor(
        public subject: string = "",
        public text_color: string = "",
        public background_color: string = "",
    ) { }
}

export class Event {
    constructor(
        public date_created: Date = new Date(),
        public created_by: string = "",
        public kind: Kind = new Kind(),
    ) { }
}

export class Kind {
    constructor(
        public type: string = "",
        public body: string | NewComment | LabelAdded | LabelRemoved = ""
    ) { }
}

export interface NewComment {
    id: number,
    liked: string[],
    text: string
}

export interface LabelAdded {
    subject: string,
    text_color: string,
    background_color: string,
}

export interface LabelRemoved {
    subject: string,
    text_color: string,
    background_color: string,
}