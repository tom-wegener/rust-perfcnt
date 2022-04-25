use x86::perfcnt::intel::events;

fn print_stats(year: &'static str, name: &'static str, size: usize) {
    println!("{}, {}, {}", year, name, size);
}

fn main() {
    // 2008, 4
    print_stats("2008", "Bonnell core", events::BONNELL.len());

    // 2008, 4
    print_stats("2008", "Nehalem EP core", events::NEHALEMEP.len());

    print_stats("2008", "Nehalem EX core", events::NEHALEMEX.len());

    // 2010, 4
    print_stats("2010", "Westmere EP DP core", events::WESTMEREEP_DP.len());

    print_stats("2010", "Westmere EP SP core", events::WESTMEREEP_SP.len());

    print_stats("2010", "Westmere EX", events::WESTMEREEX.len());

    // 2011, 8
    print_stats("2011", "SandyBridge", events::SANDYBRIDGE.len());

    // 2011, 8
    print_stats("2011", "Jaketown", events::JAKETOWN.len());

    // 2012, 8
    print_stats("2012", "IvyBridge", events::IVYBRIDGE.len());

    // 2013, 8
    print_stats("2012", "IvyTown", events::IVYTOWN.len());

    // 2013, 8
    print_stats("2013", "Silvermont", events::SILVERMONT.len());

    // 2013, 8
    print_stats("2013", "Haswell", events::HASWELL.len());

    // 2013, 8
    print_stats("2013", "HaswellX", events::HASWELLX.len());

    // 2015, 8
    print_stats("2015", "Broadwell", events::BROADWELL.len());

    // 2015, 8
    print_stats("2015", "Broadwell DE", events::BROADWELLDE.len());

    // 2015, 8
    print_stats("2015", "Skylake", events::SKYLAKE.len());
}
