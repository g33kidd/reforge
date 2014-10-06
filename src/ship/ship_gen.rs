use net::ClientId;
use ship::ShipRef;
use module::{EngineModule, ProjectileWeaponModule};

pub fn generate_ship(id: ClientId) -> ShipRef {
    let mut ship = ShipRef::new(id);
    
    let mut engine = EngineModule::new();
    engine.get_base_mut().x = 0;
    engine.get_base_mut().y = 0;
    
    let mut weapon = ProjectileWeaponModule::new();
    weapon.get_base_mut().x = 1;
    weapon.get_base_mut().y = 1;
    
    ship.add_module(engine);
    ship.add_module(weapon);
    ship
}