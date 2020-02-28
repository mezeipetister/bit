export class Ledger {
    constructor(
        public account_id: string = "",
        public account_name: string = "",
        public is_working: boolean = true,
        public is_inverse: boolean = true,
        public is_active: boolean = true,
        public debit_total: number = 0,
        public credit_total: number = 0,
        public total: number = 0,
    ) { }
}
