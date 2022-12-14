use async_graphql::{Context, Result, SimpleObject};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use entity::{self, prelude::User as UserEntity};

use crate::context::SharedContext;
use crate::graphql::user::types::{User, UserError, UserErrorCode};
use crate::services::auth::Token;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct Me {
    user: Option<User>,
    error: Option<UserError>,
}

impl Me {
    pub async fn exec(ctx: &Context<'_>) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(jwt) = ctx.data_opt::<Token>() {
            let claims = context.services.auth.verify_token(jwt).unwrap();

            if let Some(user) = UserEntity::find()
                .filter(entity::user::Column::Id.eq(claims.uid))
                .one(&context.conn())
                .await
                .unwrap()
            {
                return Ok(Me {
                    user: Some(User {
                        id: user.id,
                        email: user.email,
                        name: user.name,
                        last_name: user.last_name,
                    }),
                    error: None,
                });
            }
        }

        Ok(Self {
            user: None,
            error: Some(UserError {
                code: UserErrorCode::Unauthorized,
                message: String::from("Invalid token provided"),
            }),
        })
    }
}
