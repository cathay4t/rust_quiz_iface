// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, PartialEq, Eq)]
struct BondIface {
    name: String,
    ports: Vec<BondPort>,
}

impl BondIface {
    fn sort_ports(&mut self) {
        self.ports
            .sort_unstable_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BondPort {
    name: String,
    priority: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgeIface {
    name: String,
    ports: Vec<BridgePort>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgePort {
    name: String,
    stp: bool,
}

impl BridgeIface {
    fn sort_ports(&mut self) {
        self.ports
            .sort_unstable_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Interface {
    Bond(BondIface),
    Bridge(BridgeIface),
}

impl Interface {
    fn sort_ports(&mut self) {
        match self {
            Interface::Bond(bond_iface) => bond_iface.sort_ports(),
            Interface::Bridge(br_iface) => br_iface.sort_ports(),
        }
    }
}

fn main() {
    let mut ifaces = vec![
        Interface::Bond(BondIface {
            name: "bond1".into(),
            ports: vec![
                BondPort {
                    name: "eth1".into(),
                    priority: 10,
                },
                BondPort {
                    name: "eth0".into(),
                    priority: 10,
                },
            ],
        }),
        Interface::Bridge(BridgeIface {
            name: "br0".into(),
            ports: vec![
                BridgePort {
                    name: "eth4".into(),
                    stp: true,
                },
                BridgePort {
                    name: "eth3".into(),
                    stp: true,
                },
            ],
        }),
    ];

    for iface in ifaces.as_mut_slice() {
        iface.sort_ports();
    }
    println!("HAHA {:?}", &ifaces);
}
