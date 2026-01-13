mod control_ser;
mod deser;
mod ser;
mod soap;

fn main() {
    deser::deser();
    ser::ser();
    control_ser::control_ser();
    soap::soap();
}
