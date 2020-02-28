export class RepositoryNew {
    constructor(
        public name: String = '',
        public description: String = '',
    ) { }
}

export class RepositoryShort {
    constructor(
        public id: String = '',
        public name: String = '',
        public description: String = '',
        public created_by: String = '',
        public date_created: Date = new Date(),
        public is_active: boolean = true,
    ) { }
}