export class Customer {
    constructor(
        public id: string = "",
        public date_created: Date = new Date,
        public created_by: string = "",
        public name: string = "",
        public address: CustomerAddress = new CustomerAddress(),
        public email: string = "",
        public phone: string = "",
        public tax_number: string = "",
        public has_user: boolean = false,
        public users: string[] = []
    ) { }
}

export class CustomerAddress {
    constructor(
        public zip: string = "",
        public location: string = "",
        public address: string = ""
    ) { }
}
