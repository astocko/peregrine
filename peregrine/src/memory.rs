struct MemoryAny {}
struct Memory8 {}
struct Memory16 {}
struct Memory16KZ {}
struct Memory32 {}
struct Memory32K {}
struct Memory32KZ {}
struct Memory64 {}
struct Memory64K {}
struct Memory64KZ {}
struct Memory80 {}
struct Memory128 {}
struct Memory128KZ {}
struct Memory256 {}
struct Memory256KZ {}
struct Memory512 {}
struct Memory512KZ {}


struct Memory32Bcast {}
struct Memory64Bcast {}

enum BroadcastM64M32 {
    Memory(Memory64),
    Broadcast(Memory32Bcast),
}

enum BroadcastM128M32 {
    Memory(Memory128),
    Broadcast(Memory32Bcast),
}

enum BroadcatM256M32 {
    Memory(Memory256),
    Broadcast(Memory32Bcast),
}

enum BroadcastM512M32 {
    Memory(Memory512),
    Broadcast(Memory32Bcast),
}

enum BroadcastM128M64 {
    Memory(Memory128),
    Broadcast(Memory64Bcast),
}

enum BroadcastM256M64 {
    Memory(Memory256),
    Broadcast(Memory64Bcast),
}

enum BroadcastM512M64 {
    Memory(Memory512),
    Broadcast(Memory64Bcast),
}

struct RIPRelativeOffset8 {
}

struct RIPRelativeOffset32 {
}

struct VMemory32XMM {}
struct VMemory32XMMK {}

struct VMemory32YMM {}
struct VMemory32YMMK {}

struct VMemory32ZMM {}
struct VMemory32ZMMK {}

struct VMemory64XMM {}
struct VMemory64XMMK {}

struct VMemory64YMM {}
struct VMemory64YMMK {}

struct VMemory64ZMM {}
struct VMemory64ZMMK {}
