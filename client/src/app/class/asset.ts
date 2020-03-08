export class Asset {
    constructor(
        public id: number = 0,
        public name: string = "",
        public description: string = "",
        public account: string = "",
        public account_clearing: string = "",
        public value: number = 0,
        public date_activated: string = "",
        public depreciation_key: number = 0,
        public residual_value: number = 0,
        public date_created: Date = new Date(),
        public created_by: string = "",
        public is_active: boolean = true,
        public depreciation_last_day_value: number = 0,
        public depreciation_last_day: Date = new Date(),
        public depreciation_daily_value: number = 0,
        public depreciation_monthly: DepreciationMonthly[] = []
    ) { }
}

export class DepreciationMonthly {
    constructor(
        public month: string = "",
        public monthly_value: number = 0,
        public cumulated: number = 0,
    ) { }
}

export class AssetNew {
    constructor(
        public name: string = "",
        public description: string = "",
        public account: string = "",
        public account_clearing: string = "",
        public value: string = "",
        public date_activated: string = new Date().toISOString().split('T')[0],
        public depreciation_key: number = 0,
        public residual_value: number = 0
    ) { }
}