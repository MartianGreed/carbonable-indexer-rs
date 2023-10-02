use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GlobalYieldIden::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GlobalYieldIden::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GlobalYieldIden::YielderAddress)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GlobalYieldIden::Deposited)
                            .binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GlobalYieldIden::Claimed).binary().not_null())
                    .col(
                        ColumnDef::new(GlobalYieldIden::Claimable)
                            .binary()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(CustomerYieldIden::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustomerYieldIden::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CustomerYieldIden::Address)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomerYieldIden::YielderAddress)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomerYieldIden::Deposited)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomerYieldIden::Claimed)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CustomerYieldIden::Claimable)
                            .binary()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GlobalYieldIden::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CustomerYieldIden::Table).to_owned())
            .await
    }
}
