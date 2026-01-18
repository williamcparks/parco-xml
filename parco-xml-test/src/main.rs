mod control_ser;
mod ser;
mod soap;

fn main() {
    ser::ser();
    control_ser::control_ser();
    soap::soap();
}
