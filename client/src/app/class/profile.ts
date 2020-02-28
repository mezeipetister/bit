export class Profile {
    constructor(
        public username: string = "",
        public name: string = "",
        public email: string = "",
        public phone: string = "",
        public created_by: string = "",
        public date_created: Date = null,
        public customers: string[] = []
    ) { }
}

export class ProfileNew {
    constructor(
        public username: string = "",
        public name: string = "",
        public email: string = "",
        public phone: string = "",
    ) { }
}
