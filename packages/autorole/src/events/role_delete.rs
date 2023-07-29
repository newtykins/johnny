use common::preludes::event::*;
use db::{autorole::*, prelude::*};

pub async fn role_delete(role_id: &RoleId, db: &DatabaseConnection) -> Result<()> {
    // delete the associated autorole document
    let model = Entity::find()
        .filter(Column::RoleId.eq(role_id.to_string()))
        .one(db)
        .await?;

    if let Some(model) = model {
        model.delete(db).await?;
    }

    Ok(())
}
