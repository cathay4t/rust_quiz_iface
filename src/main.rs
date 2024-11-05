// SPDX-License-Identifier: Apache-2.0

trait Controller {
    type Port: PortConfig;

    fn ports_mut(&mut self) -> &mut [Self::Port];

    fn sort_ports(&mut self) {
        self.ports_mut()
            .sort_unstable_by(|a: &Self::Port, b| a.name().cmp(b.name()));
    }
}

trait PortConfig {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BondIface {
    name: String,
    ports: Vec<BondPort>,
}

impl Controller for BondIface {
    type Port = BondPort;

    fn ports_mut(&mut self) -> &mut [BondPort] {
        self.ports.as_mut_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BondPort {
    name: String,
    priority: u32,
}

impl PortConfig for BondPort {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgeIface {
    name: String,
    ports: Vec<BridgePort>,
}

impl Controller for BridgeIface {
    type Port = BridgePort;

    fn ports_mut(&mut self) -> &mut [BridgePort] {
        self.ports.as_mut_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgePort {
    name: String,
    stp: bool,
}

impl PortConfig for BridgePort {
    fn name(&self) -> &str {
        self.name.as_str()
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
