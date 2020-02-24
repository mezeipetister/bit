export class Folder {
    constructor(
        public id: string = "",
        public name: string = "",
        public description: string = "",
        public created_by: string = "",
        public date_created: Date = new Date(),
        public is_active: boolean = true,
    ) { }
}

export class FolderNew {
    constructor(
        public name: string = "",
        public description: string = ""
    ) { }
}

export class FolderNewName {
    constructor(
        public name: string = ""
    ) { }
}

export class FolderNewDescription {
    constructor(
        public description: string = ""
    ) { }
}