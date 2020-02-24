export class Document {
    constructor(
        public id: string = "",
        public reference: string = "",
        public folder_id: string = "",
        public title: string = "",
        public description: string = "",
        public due_date: Date = new Date(),
        public file_id: null | string = null,
        public created_by: string = "",
        public date_created: Date = new Date(),
        public is_active: boolean = true,
    ) { }
}

export class DocumentNew {
    constructor(
        public reference: string = "",
        public title: string = "",
        public description: string = "",
    ) { }
}