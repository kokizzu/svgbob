extern crate svgbob;
extern crate svg;

use svgbob::Grid;
use svgbob::Settings;


fn main() {
    let file = "examples/demo.svg";
    let g = Grid::from_str(get_arg());
    let svg = g.get_svg(&Settings::compact());
    svg::save(file, &svg).unwrap();
    println!("Saved to {}",file);
}

fn get_arg() -> &'static str{

let arg = r#"

    -o----o---
          |                    .--> D
           \                  /
A -> B -> C -----------------*---> .
            \         ^  ^ ^  \
             B -> C _/  / /    '--> D
 *----->           \   / /
                    *

                         180 ohms    .47uH
from inverter output----\/\/\/--+---(((((----- xtal --+-- to input
                                |             series  |
                               _|_                   _|_
                               ___ 120pF             ___ 100pF
                                |                     |
                               _|_       _|_         _|_      .       .
                               \ /       \ /         \ /     / \     /_\
                                '         .           '     '-|-'      

      |   |  | |     / \   /  \
       \ /   \ /     | |  |    |

        .                          .
      .' `.                      ,' `.       .'.
    .'     `.  tilted square   .'     `,   .'   `,
     `.   .'                    `.   ,'     `. ,'
       `.'                        `.'         `


                                                   +-----+------+
                                             .---> |-----|------|
                                            /      |-----|------|
                                           /       +-----+------+
                                          /                 .--.
                                         /                  |  |
                                        /                   v  |
  .-------.                            /           .-. .-. .-. |
  | Table |-.                         /        .-->'-' '-' '-' |
  '-------'  \                       / .-----> |     \  |  /   |
              \                     / /        |      v | v    |
.------------. \                   / /         '_______/ \_____|
| Flowcharts |--.                 / /                  \ /
'------------'   \               / /                    |      ____    
                  v _______     / /                     '---> /___/ 
.--------.         /       \---' /                      
| Graphs |------->/ Sponge  \---'-.
'--------'     .->\  down   /----. \           ^  .  /\  .-.
              / .->\_______/-.    \ \          |_/ \/  \/   \
.--------.   / /              \    \ `-------> +------------->
| Comics |--' /                \    \
'--------'   /        .`.       \    \         +------------+
            /       .'   `,      \    \        |   .-----.  |
   .----------.      `. ,'        \    \       |  (       ) +------------+
   | Diagrams |        `           \    \      |   `-, .-'  |  .-----.   |
   '----------'                     \    `---> |    /,'     | (       )  |
                                     \         |   /'       |  `-. .-'   |
                                      \        |            |     `.\    |
                                       \       | ٩(̾●̮̮̃ ̾•̃̾)۶    |       `\   |
                                        \      |            |            |
                 .`.                     \     +------------|   (,⊙–⊙,)७ |
               .'   `,                    `--.              +------------+
                `. ,'                         \
                  `                            v           .-,(  ),-.    
                                            ___  _      .-(          )-.                                       
                                           [___]|=| -->(                )      __________ 
                                           /::/ |_|     '-(          ).-' --->[_...__...°]
                                                           '-.( ).-'                         
                                                                   \      ____   __ 
                                                                    '--->|    | |==|
                                                                         |____| |  | 
                                                                         /:::/  |__|
+------+   +-----+   +-----+   +-----+
|      |   |     |   |     |   |     |
| Foo  +-->| Bar +---+ Baz |<--+ Moo |
|      |   |     |   |     |   |     |
+------+   +-----+   +--+--+   +-----+
              ^         |
              |         V
.-------------+-----------------------.
| Hello here and there and everywhere |
'-------------------------------------'
                        ____________
   .--------------.     \           \
  / a == b         \     \           \     __________
 (    &&            )     ) process   )    \         \
  \ 'string' ne '' /     /           /     / process /
   '--------------'     /___________/     /_________/
    __________________
    \_________________\
     \                 \
      . another process .
     /_________________/
    /_________________/
  User code  ^               ^ OS code
              \             /
               \        .--'
                \      /
  User code  <--- Mode ----> OS code
                /      \
            .--'        \___
           /                \
          v                  v 
       User code            OS code
             .---.  .---. .---.  .---.    .---.  .---.
    OS API   '---'  '---' '---'  '---'    '---'  '---'
               |      |     |      |        |      |
               v      v     |      v        |      v
             .------------. | .-----------. |  .-----.
             | Filesystem | | | Scheduler | |  | MMU |
             '------------' | '-----------' |  '-----'
                    |       |      |        |
                    v       |      |        v
                 .----.     |      |    .---------.
                 | IO |<----'      |    | Network |
                 '----'            |    '---------'
                    |              |         |
                    v              v         v
             .---------------------------------------.
             |                  HAL                  |
             '---------------------------------------'
             
   ____[]
  | ___ |
  ||   ||  device
  ||___||  loads
  | ooo |----------------------------------------------------------.
  | ooo |    |                          |                          |
  | ooo |    |                          |                          |
  '-----'    |                          |                          |
             |                          |                          |
             v                          v                          v
   .-------------------.  .---------------------------.  .-------------------.
   | Loadable module C |  |     Loadable module A     |  | Loadable module B |
   '-------------------'  |---------------------------|  |   (instrumented)  |
             |            |         .-----.           |  '-------------------'
             '------------+-------->| A.o |           |             |
                 calls    |         '-----'           |             |
                          |    .------------------.   |             |
                          |   / A.instrumented.o /<---+-------------'
                          |  '------------------'     |    calls
                          '---------------------------'   
        .--------------.
         \              \
          '--------------'
                                        .--> Base::Class::Derived_A
                                       /
                                      .----> Base::Class::Derived_B    
      Something -------.             /         \
                        \           /           .---> Base::Class::Derived
      Something::else    \         /             \
            \             \       /               '--> Base::Class::Derived
             \             \     /
              \             \   .-----------> Base::Class::Derived_C 
               \             \ /
                '------ Base::Class
                       /  \ \ \
                      '    \ \ \  
                      |     \ \ \
                      .      \ \ '--- The::Latest
                     /|       \ \      \
 With::Some::fantasy  '        \ \      '---- The::Latest::Greatest
                     /|         \ \
         More::Stuff  '          \ '- I::Am::Running::Out::Of::Ideas
                     /|           \
         More::Stuff  '            \
                     /              '--- Last::One
       More::Stuff  V 
 

 +10-15V          ___0,047R
  X------o------o-|___|-o--o---------o----o-------o
         |      |       |  |         |    |       |
        ---     |       | .-.        |    |       |
  470uF ###     |       | | | 2k2    |    |       |
         | +    |       | | |        |    |       |
  X------o      '--.    | '-'       .-.   |       |
         |         |6   |7 |8    1k | |   |       |
        GND      .------------.     | |   |       |
                 |            |     '-'   |       |
                 |            |1     |  |/  BC    |
                 |            |------o--|   547   |
                 |            |      |  |>        |
                 |            |     .-.   |       |
                 |            | 220R| |   o----||-+  IRF9Z34
                 |            |     | |   |    ||->
                 |  MC34063   |     '-'   |    ||-+
                 |            |      |    |       |  BYV29     -12V6
                 |            |      '----'       o--|<-o----o--X OUT
                 |            |2                  |     |    |
                 |            |--|                C|    |    |
                 |            | GND          30uH C|    |   --- 470
                 |            |3      1nF         C|    |   ###  uF
                 |            |-------||--.       |     |    | +
                 '------------'           |      GND    |   GND
                      5|   4|             |             |
                       |    '-------------o-------------o
                       |                           ___  |
                       '-----------------------o--|___|-'
                                               |       1k0
                                              .-.
                                              | | 5k6 + 3k3
                                              | | in Serie
                                              '-'
                                               |
                                              GND

 +12v  (sidelights)
     |
   +-+--+
   |    | Buzzer (12v)
   +-+--+
     |
     +-------|<|-------- To existing drivers interior light switch
     |       Diode
     |
     O
    ----|  Drivers door switch
     O
     |
    Car Chassis

        +---+
        | 2 | ---<640k>---+
        | 3 | ---<320k>---|
        | 4 | ---<160k>---|
        | 5 | ---<80k>----|
        | 6 | ---<40k>----|
        | 7 | ---<20k>----+-----+---- 10 uF -------> Out
        | 8 | ---<10k>----|     |
        | 9 | ---<5k>-----+   <390>
        |   |                   |
        | 25| ------------------+------------------+
        +---+                                      |
                                                  ===
                                                 Ground


      SW3
+6V -o_|_o----+----------+----------->>--------+----------+----->>
              |          |                     |          |
            LAMP1        |                     |        LAMP2
              |          | SW1                 | SW2      |
              +--A>     |o              <A--   o|         |
              |       ==|    --B>          |    |==   <B--+
            -----       |o   |             |   o|       -----
      SCR1  \   /        |   |             |   |        \   /  SCR2
             \ /        R3   ^             ^   R4        \ /
            -----        |  CR1           CR2  |        -----
              |  \       |   |             |   |       /  |
              |   +--R2--+---+             +---+--R5--+   |
              |   |                                   |   |
              |   R1                                 R6   |
              |   |                                   |   |
GND ----------+---+----------------->>----------------+---+--->>
 
SW1,SW2         normally open momentary pushbuttons
SW3             normally closed momentary pushbutton
LAMP1, LAMP2    6V incandescent lamps
R1, R6          470 ohm
R2,R3,R4,R5     1 K
SCR1, SCR2      Small SCRs, not power type
CR1, CR2        1N914 diodes
+               connection
^               cathode of a diode
--A> <A--       are connected (jump)
--B> <B--       same deal

+5 ---+---------------+---+
      |               |   |
      R            +----------+
      |(see below) |  8   4   |
  +---+------+-----|7        3|------/\/\/\---+------- Vout
  :          |     |  LMC555  |               |
  C to test  +-----|6         |             -----
  :                |          |             -----
ground       +-----|2        5|----+          |
             |     |    1     |    |        ground
             |     +----------+  0.1 uF
             |          |          |
   Clock ----+        ground     ground
     ^
(see | next section - CMOS Oscillator)


                                             +-----------> Output
                                             |
                                             |
     |\            |\            |\          |     |\
     |  \          |  \          |  \        |     |  \
     |    \        |    \        |    \      |     |    \
  +--|     >O------|     >O------|     >O----+-----|     >O-----|
  |  |    /        |    /        |    /            |    /       |
  |  |  /          |  /          |  /              |  /         |
  |  |/            |/            |/                |/           |
  |                                                             |
  |                              +5.0V                          |
  |                                |                            |
  |                                \                            |
  |                                /  R1                        |
  |                      /|        \ 10K                        |
  |                    /  |        /                 L1         |
  |                  /    |        |                75 uH       |
  +----------------O<     |--------+---+----------)()()()(------+
                     \    |            |
                       \  |            /
                         \|            \  R2
                                       / 10K
                                       \
                                       |
                                      GND


                         ___+5V
                          |
                         _|_
                        _\_/_  LED
         ____             |
TTL ____|6k8 |__________|/
        |____|    |     |\e
                 _|_     _|_
                |4k7|   |270|
                |_ _|   |_ _|
                  |       |
                 _|_______|_GND


   -----+--------+--------+--------+--> +9v (whatever)
           |        |        |        |
          R1       R2       R3       R4
           |        |        |        |
    +------+        |        |        +------+
    |      +---C1---+        +---C2---+      |
    D1    c|        |        |        |      D2
    |       \       |        |        |c     |
    |    Q1  |------|--------+     b /       |
    |       / b     +---------------|   Q2   |
    |     e|                         \       |
    |      |                          |e     |
   -+------+--------------------------+------+--->GND
 
Q1, Q2  ... anything NPN : BC108, 2N2222, etc
R1, R4  ... 1k0
R2, R3  ... 10k0
C1, C2  ... 10uF
D1, D2  ... Your LEDs


                    +5VDC
                     |
                     \
                     /  270 ohms
                     \
                     /
Control|\ 7407       |
-------| >-----------+
       |/            |
Control = 1 LED on   |
Control = 0 LED off  |
                     |
                    LED
                   anode
                 
Part 1:
~~~~~~~                              C4-C10 - DISK CERAMICS
                    1M-1Watt
                       R3    100p       100p      100p      100p
                 +----====----||---------||--------||--------||------+
    D3-D11:      |            C4    /\   C6   /\   C8   /\  C10   /  |
       high-     |  Diodes follow  /  \      /  \      /  \      /   |
       voltage   |  direction to  D5   D6   D7   D8   D9   D10  D11  |
       3kV diodes|  D11 from D5  /      \  /      \  /      \  /    | |R4
  A              |              /   C5   \/   C7   \/   C9   \/     | |47K
  <--------------|------D3>|--------||--------||--------||-----      |
              _  |             |   100p      100p      100p         | |R5
     Diode   /|\ D4            |                                    | |33K
     going    |  |          C2===0.1/1.6KV                           |
     up.....> |  |             |                                  ___|
  B              |             |                           +VE -_|_
  /_____________/|\____________|                               |   |
  \              |             |                               |   | LASER
                 |             |                               |   | TUBE
  <--To part 2   |          C3===0.1/1.6KV                     |___|
                 |             |                           GND - |
                 |             |                                 |
                 |             |                                 |
                 |_____________|_________________________________|


 CMOS or HCMOS inverter
              |\
           +--| >0---+----> OUT
           |  |/     |
           |         |
           +--\/\/\--+
           | 1 Mohm  |
           |         |
           |         \
           |         / 2.7 kohms
           |         \
           |         /
           |         |
           |   1MHz  |   parallel resonant
           +---|[]|--+
          _|_       _|_
  55pf    ___       ___  60pf
          _|_       _|_
          \ /       \ /
           '         '

                         .47 uH    56 pF        180 ohms
from inverter output-----(((((------| |---------\/\/\/-+- to xtal
                                                       |
                                                      _|_
                                                      ___ 56 pF
                                                       |
                                                      _|_
                                                      \ /
                                                       '

A C-L-C pi filter and series resonant crystal is another solution:
                         180 ohms    .47uH
from inverter output----\/\/\/--+---(((((----- xtal --+-- to input
                                |             series  |
                               _|_                   _|_
                               ___ 120pF             ___ 100pF
                                |                     |
                               _|_                   _|_
                               \ /                   \ /
                                '                     '

        VR1
<G>-+-/\/\/\--+--+-----------+--------+--R6--+----+-----+-----<+12VDC>
    |   ^     ^  |           |        |      |    |     |
    C2  |    CR2 C8          R4     | c      R8 | c    C11
    +---+     |  |           |      |/       |  |/      |
        |  <G>+--+    +--C5--+---+--|        +--|      <G>
audio   R1            |      |   |  |\       |  |\
in      |             |      |   C6 | e      |  | e
O--C1---+--R2--+--C4--+      R3  |    |      |    |
        |      ^      |      |   +----+--C9--+    +--C10--O  RF out
        C3    CR1     L1     |   C7   R5     R7   R9
        |      |      |      |   |    |      |    |
<G>-----+------+------+------+---+----+------+----+-----------<G>
 
                                    Q1          Q2
 
+               connection<G>             ground connection
^               cathode of a diode
 
C1, C2, C8, C11 .1 uF
C3, C4          .001 uF
C5, C6, C7      39 pF NPO or silver mica
C9              10 pF NPO or silver mica
C10             22 pF
CR1             ECG616 varactor (tuning) diode
CR2             9v Zener
L1              5T #20 wire, 1/8 inch I.D., adjust spacing for tuning range
Q1              2N3663
Q2              2N3904
R1, R2          47 K
R3              22 K
R4              15 K
R5              1 K
R6              390 ohm
R7, R8          4.7 K
R9              100 ohm
VR1             100 K, linear taper


          o +5v
             |
         +---+
         |   |
      1k R   |
         |   |c  BC108B
         +-b
2Vpp     |   |e      75ohm
video    |e  +--------R-------> video out
in >---b     |                  1Vpp @ 75 ohm
         |c  R 1k          +-->
   BC178B|   |             |
         +---+            ---
             |
             o -5v (yes, two-sided power supply, not ground)


                               o +5v
                               |
              +----+-----------+
              |    |           |
          3k8 R    R 680R      R 56R
              |    |           |
              |    |           |e BC178B
Video in      |    +---------b
1Vpp/75R   +  |    |c          |c    100n plastic
>---+----||---+--b    BC108B   +-----||--------> to black level clamping
    |  47u    |    |e          |                  2Vp-p
    |         |    |           R 220R
    R 75R     R    +-----------+
    |         |1k8             R 150R
   ---       ---               |
                              --- (single power supply this time)


        D           R
 <------>|-------/\/\/\/---
                    ^
                    |---------------> To anode terminal
 To                   |
 120 VAC              |    C
                      ----|(----
                               |
    <--------------------------|----> To selected cathode
 
D :  any rectifier diode of >= 250 PIV
R :  100 K or so variable resistor
C :  20 uf (or more) at at least 250 WVDC (observe polarity!)


                               capacitor (0.5 uf or so)

                                   | |
    +---------------------+--------| |--------------> to amplifier
    |                     |        | |
    | (positive lead)     |           _   _
    |                     +----------/ \_/ \_/----------+
   MIC                          (resistor 1-2K or so)   |
    |                                                   | +
    | (ground lead) (-)                              -------
    |                              (battery 9v or so)  ---
    |                                                   |
    |                                                   |
    +---------------------------------------------------+----> ground
                                                                to amp

        300 ohm              Charge
               5W             /  Switch
20-40 + O---/\/\/\----o------o  o------------o----------------o
VDC                   |                      |                |
                      |      Zap             |                |
                      |      Switch          |               +|
                      |      ___|___         |           -----------
                      o------o     o---------o              -----
                      |                      | +    Shorted   |
      6000 micro-     | +                 -------    Cell     |
      Farad, 40V  _________               |     |             |
      Capacitor   ---------               |_____| Volt        |
                      |                      |    meter       |
                      |                      |                |
      - O-------------o----------------------o----------------o


   R1=R2                      |---\/\/\/\/\\/\--|
                              |       R2        |
                              |                 |
                 R1           |     |\          |
                              | INV | \         |
INPUT --------\/\/\/\/\/\/----------|  \        |
          |                         |   \       |
          |                         |    \-------- OUTPUT
          |       R3                |    /
          |                 NON-INV |   /
          |---\/\/\/\/\/\/----------|  /
                              |     | /
                              |     |/
                              |
                              |
                           -------  C
                           -------
                              |
                              |
                              |
                              |
                           GROUND

                     V+
                          |
                          R
              V+          |
               |          +---------+
              10K        /          |
     0.1 uF    |       |/           P   0.1 uF
IN>----||------+-------|  2N2222    O<--||---> High impedance output
               |       |\E          T            (should be buffered)
              10K        \          |
               |          +---------+
              GND         |
                          R
                          |
                         GND
                         
+V --o----------o--------------------o-------------o-------o
      |          |                    |             |       |
      |          |     R3            /              |       |
      |          |   +----+       /-C               |       |
      |          o---+ 15K+---o---B | TR1           |       |
      |          |   +----+   |   \-E              /        |
      |          |            |      \          /-C         | 47nF
     +++        +++           |       ----o-----B | TR2   =====
     | | 2M2    | | 10K       |        C1 |     \-E         | C2
     | | R1     | | R2        |  D1     =====      \        |
     +++        +++           |   | /|    | 10nF    |       |
      |          |        /---o---|< |----o---------o-------o
      |          |     /-C        | \|                      |
      |       /--o-----B | TR4     1N4148                   |  Earthy
      |    /-C         \-E                                -----  Side
      o----B | TR3        \                                XXX
      |    \-E             |                              -----
      |       \            |                                |  Transducer
      |        |           |                                |
      o--------|-----------|--------------------------------o
      |        |           |
      |  | /|  |           |
      o--|< |--o-----------o-------  0V       All transistors 2N3707
         | \|                                 NPN, Emitter at bottom.
        D2  1N4148

            +--------+
            |        |
            |  7555  |
            |        |
       +----|2       |
       |    |       3|--+---> output
       +----|6       |  |
       |    +--------+  |
       |                |
       +-----/\/\/------+
       |       R
     -----
     ----- C
       |
       |
      gnd


             ^ Vcc
             |
             \
             / R
             \
             /
             |
             |
            ---
            \ / LED   (glows when 555 ouput is high)
             V
            ---
             |
             |
             /
           |/
in -\/\/\--|  3904
      R    |\
             V
             |
             |
            gnd

and:
             ^ Vcc
             |
             /
           |<
in -\/\/\--|  3906
      R    |\
             |
             |
             |
             \
             / R
             \
             /
             |
             |
            ---
            \ / LED   (glows when 555 output is low)
            ---
             |
             |
            gnd


0V--cap-+--resist--+
        |          |                 +5V
        | |\       |                 _|_
 +5     +-|-\      |                 \ / LED
  |       |  \     |     +----resist--+
 resist   |   >----+-----|
  |       |  /     |     +----resist--+
  +-------|+/      |                 _|_
  |       |/       |                 \ / LED
  |                |                  |
  +----resist------+                  0V
  |
  |
resist
  |
  0V


   *VCC     Q1   +-----------------C ||
     o           |                 C ||
     |       B |/ C                C ||
     |  +------|    2N3055         C ||
     |  |      |\ E            5T  C || C-------|>|----------o  +HV
     |  |        |                 C || C     HV Diode, usually
     |  |       -_-                C || C        built in
     |  |                          C || C
     +--|--------------------------C || C
     |  |   Q2  _-_                C || C
     |  |        |                 C || C Secondary (HV) winding,
     |  |    B |/ E            5T  C || C intact.
     |  |  ----|    2N3055         C || C
     |  |  |   |\ C                C || C
     |  |  |     |                 C || C
     |  |  |     +-----------------C || C
     |  |  |                         || C
     |  |  ------------------------C || C-------------------o  -HV
     |  |                      2T  C ||
     |  |               +----------C ||
     |  |               |      2T  C || T1 - Flyback transformer from BW
     |  +--------------------------C ||      or color TV or computer monitor.
     |                  |
     |            R1    |    R2
     +----------/\/\/\--+--/\/\/\---+
                  110        27    _|_
                  2W         5W     -



    +V
           ^
           |
   +-------+---+
   |       |   |                  -V Output
  R1       |8  |4        +----+--->    
   |    7 -------        |    |            Parts List:
   +-----|       |      D2    |            IC1 = 555
   |  +--|       |  +    |    |             R1 = 1.5K
  R2  | 6|  IC1  |---C1--+    |             R2 = 10K
   |  |  |       |3      |    |             C1 = 10uF,16V
   +--+--|       |      D1   C2             C2 = 22uF,16V
   |    2|       |       |    |+            C3 = 1500pF
   |      -------        |    |          D1,D2 = 1N4001 diodes
  C3         |1          |    |
   |         |           |    |
   +---------+-----------+----+
             |
            ===
           (GND)


                                            e->
         <-e                                   |------------|
       |----------                             |            |
       |         |  <<<<  physical            CURRENT     POWER FOR
  --REGULATOR   LOGIC    separation >>>>      DRIVER       DRIVERS
       |         |                              |           |
       |   e->   |      ground                  |   <-e     |
  -----|--------------------------------------------------------------
         ^ thick          ^ thin                 ^
           traces           traces              very thick
           from reg to                          traces from
           logic load                           drivers to supply



                            (hot side)
                    ==============================  <-- ceramic
 /|\                   __________    __________
  | majority          |   |  |   |  |   |  |   |
  | carrier           | N |  | P |  | N |  | P |
  | flow    (-)  _____|___|  |___|__|___|  |___|_____  (+)

                    ===============================  <-- ceramic
                            (cold side)

"#;

arg
}
