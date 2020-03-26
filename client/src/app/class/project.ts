export class Project {
    constructor(
        public id: String = "",
        public name: String = "",
        public description: String = "",
        public created_by: String = "",
        public date_created: Date = new Date(),
        public is_enabled: boolean = true,
    ) { }
}

export class ProjectUpdate {
    constructor(
        public name: String = "",
        public description: String = "",
        public is_enabled: boolean = true,
    ) { }
}

export class ProjectNew {
    constructor(
        public name: String = "",
        public description: String = "",
    ) { }
}
