export class Transaction {
    constructor(
        public id: number = 0,
        public subject: string = "",
        public debit: string = "",
        public credit: string = "",
        public amount: number = 0,
        public date_created: Date = new Date(),
        public date_settlement: Date = new Date(),
        public created_by: String = ""
    ) { }
}

export class TransactionNew {
    constructor(
        public subject: string = "",
        public debit: string = "",
        public credit: string = "",
        public amount: number = 0,
        public date_settlement: string = new Date().toLocaleDateString(),
    ) { }
}