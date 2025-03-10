use spacetimedb::{table, reducer, Table, ReducerContext, Identity};


#[table(name = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
}

#[table(name = checkbox, public)]
pub struct Checkbox {
    #[primary_key]
    id: u32,
}


/// Takes a name and checks if it's acceptable as a user's name.
fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

/// Takes a message's text and checks if it's acceptable to send.


#[reducer]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User { name: Some(name), ..user });
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

fn validate_id(id: u32) -> Result<u32, String> {
    if id > 1_000_000 {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(id)
    }
}

#[reducer]
/// Clients invoke this reducer to toggle checkboxes.
pub fn toggle(ctx: &ReducerContext, id: u32) -> Result<(), String> {
    log::info!("Before updating {}", id);
    let id = validate_id(id)?;
    log::info!("Updating {}", id);
    if let Some(checkbox) = ctx.db.checkbox().id().find(id) {
        ctx.db.checkbox().id().delete(checkbox.id);
    } else {
        ctx.db.checkbox().insert(Checkbox {
            id
        });
    }

    Ok(())
}

#[reducer(client_connected)]
// Called when a client connects to the SpacetimeDB
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        // If this is a returning user, i.e. we already have a `User` with this `Identity`,
        // set `online: true`, but leave `name` and `identity` unchanged.
        ctx.db.user().identity().update(User { online: true, ..user });
    } else {
        // If this is a new user, create a `User` row for the `Identity`,
        // which is online, but hasn't set a name.
        ctx.db.user().insert(User {
            name: None,
            identity: ctx.sender,
            online: true,
        });
    }
}

#[reducer(client_disconnected)]
// Called when a client disconnects from SpacetimeDB
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User { online: false, ..user });
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!("Disconnect event for unknown user with identity {:?}", ctx.sender);
    }
}