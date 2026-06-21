// =========================================================================
// TEST 1: Variable Inheritance (Based on Lecture Example 3)
// c2 extends c1, and shadowing y
// =========================================================================

struct C1Fields {
    x: i32,
    y: i32,
}

struct C2Fields {
    base: C1Fields, // c1 extends object / c2 extends c1 
    y: i32,         // shadowing y that defined by c2
}

impl C2Fields {
    fn new() -> Self {
        C2Fields {
            base: C1Fields { x: 0, y: 0 },
            y: 0,
        }
    }

    // c1 method simulation
    fn setx1(&mut self, v: i32) { self.base.x = v; }
    fn sety1(&mut self, v: i32) { self.base.y = v; }
    fn getx1(&self) -> i32 { self.base.x }
    fn gety1(&self) -> i32 { self.base.y }

    // c2 method simulation
    fn sety2(&mut self, v: i32) { self.y = v; }
    fn getx2(&self) -> i32 { self.base.x }
    fn gety2(&self) -> i32 { self.y }
}


// =========================================================================
// TEST 2: Method Dispatch (Based on Lecture Example 5 & Final Example)
// call method by self and super
// =========================================================================

trait C1Trait {
    fn m1(&self) -> i32;
    fn m2(&self) -> i32;
}

trait SampleTrait {
    fn get(&self) -> i32;
    fn m1_sample(&self) -> i32 {
        // "send self get()"
        self.get()
    }
}

// --- Example 5 class construction ---
struct C1;
impl C1Trait for C1 {
    fn m1(&self) -> i32 { self.m2() } // send self m2()
    fn m2(&self) -> i32 { 13 }
}

struct C2;
impl C1Trait for C2 {
    fn m1(&self) -> i32 { 22 }
    fn m2(&self) -> i32 { 23 }
}
impl C2 {
    // method m3 () super m1 ()
    fn m3(&self) -> i32 {
        let super_instance = C1;
        super_instance.m1() // static dispatch
    }
}

struct C3;
impl C1Trait for C3 {
    fn m1(&self) -> i32 { 32 }
    fn m2(&self) -> i32 { 33 }
}
impl C3 {
    // c3 inheritance c2
    fn m3(&self) -> i32 {
        let super_instance = C2;
        super_instance.m3()
    }
}

// --- self get() test ---
struct SampleC2 {
    x: i32,
    y: i32,
}
impl SampleTrait for SampleC2 {
    // =cs get() return y
    fn get(&self) -> i32 { self.y }
}


fn main() {
    println!("=== OOP Lecture Examples Empirical Test in Rust ===\n");

    // ---------------------------------------------------------------------
    // [Test 1: Variable Inheritance & Shadowing (Example 3)]
    // ---------------------------------------------------------------------
    println!("[Test 1: Variable Inheritance (Example 3)]");
    let mut o2 = C2Fields::new();
    
    o2.setx1(101);
    o2.sety1(102);
    o2.sety2(999);
    
    println!("send o2 getx1() -> {}", o2.getx1());
    println!("send o2 gety1() -> {}", o2.gety1());
    println!("send o2 getx2() -> {}", o2.getx2());
    println!("send o2 gety2() -> {}\n", o2.gety2());


    // ---------------------------------------------------------------------
    // [Test 2: Method Dispatch & Super (Example 5)]
    // ---------------------------------------------------------------------
    println!("[Test 2: Method Dispatch with 'super' (Example 5)]");
    let o3 = C3;
    // send o3 m3(). Expected result is 13.
    println!("send o3 m3() -> {} (Expected: 13)\n", o3.m3());


    // ---------------------------------------------------------------------
    // [Test 3: Dynamic Dispatch with 'self' (Final Lecture Example)]
    // ---------------------------------------------------------------------
    println!("[Test 3: Dynamic Dispatch with 'self' (Final Example)]");
    let o2_sample = SampleC2 { x: 3, y: 4 }; 
    
    // dynamic dispatch
    let dynamic_obj: &dyn SampleTrait = &o2_sample;
    
    println!("send o2 m1() -> {} (Expected: 4)", dynamic_obj.m1_sample());
}