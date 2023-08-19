@0xa5e994c4ed48b34c;

using Rust = import "/rust.capnp";
$Rust.parentModule("schemas");

# Simple math types
struct IVec2 @0x82293be591f48c52 {
    x @0 :Int32;
    y @1 :Int32;
}

struct IVec3 @0x8656ec7ddc60888e {
    x @0 :Int32;
    y @1 :Int32;
    z @2 :Int32;
}

struct I64Vec2 @0xf5a1c5adf2c416d4 {
    x @0 :Int64;
    y @1 :Int64;
    z @2 :Int64;
}

struct I64Vec3 @0x928035efab755766 {
    x @0 :Int64;
    y @1 :Int64;
    z @2 :Int64;
}

struct Vec2 @0xa6a42c0dee272cb9 {
    x @0 :Float32;
    y @1 :Float32;
}

struct Vec3 @0xed69b4c78460e1c0 {
    x @0 :Float32;
    y @1 :Float32;
    z @2 :Float32;
}

struct Quat @0xfab98c6be0936a7d {
    x @0 :Float32;
    y @1 :Float32;
    z @2 :Float32;
    w @3 :Float32;
}

# Named registry-related types
struct RegistryName @0xbbde17bbe6af716a {
    ns @0 :Text;
    key @1 :Text;
}

struct RegistryIdMappingBundle @0xe1c96c086209943d {
    # Inlined RegistryName for efficiency
    # All lists must have equal length
    nss @0 :List(Text);
    keys @1 :List(Text);
    ids @2 :List(UInt32); # NonZero
}
