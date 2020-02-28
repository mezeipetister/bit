export class Account {
    constructor(
        public id: String = "",
        public name: String = "",
        public description: String = "",
        public created_by: String = "",
        public date_created: Date = new Date(),
        public is_working: boolean = true,
        public is_inverse: boolean = true,
        public is_active: boolean = true,
    ) { }
}

export class AccountNew {
    constructor(
        public id: String = "",
        public name: String = "",
        public description: String = "",
        public is_working: boolean = true,
        public is_inverse: boolean = true,
    ) { }
}
