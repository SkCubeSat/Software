



## SPACE IS AWESOME
## Clyde Space Ltd.
## Skypark 5,
## 45 Finnieston Street,
Glasgow G3 8JU, U.K.
t: + 44 (0) 141 946 4440

e: enquiries@clyde.space
w: www.clyde.space

Registered in Scotland No. SC285287
at 123 St Vincent Street Glasgow G2 5EA







User Manual: 3rd Generation CubeSat
## Battery Family

Document No.: USM-1192
## Issue: E
## Date: 26/06/2018

## Name Date Signed
## Author Edgars Pavlovskis 18/12/2015
Updated Anne McLaren 26/06/2018
## Approved Colin Waddell 26/06/2018


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 2 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## Document Control
Issue Date Section Description of Change Reason for Change
A 15/02/2016 All First Revision Based on NanoRacks 3G battery
## USM
## B 19/04/2016 Section 4
## Section 8
## Section 10
Updated weights
Updated grounding
description and pinout
Addition of cell-level
protection circuit diagram
See ECN648
## Section 13

## Section 4
## Section 5 & 10
Addition of Launch Provider
Information section
Update integrated battery
heights
Added battery discharge
warning if inhibits are not
enabled


See ECN710
C 14/07/2016 Section 2 Add 80Wh to list of products
covered
## ECN759
Section 3 Add 80Wh battery maximum
ratings

Section 4 Add electrical and physical
characteristics of 80Wh
battery

Section 5 Add note about installing
inhibits during storage

Section 7 Add 80Wh drawing
Section 8 Describe interface of 80Wh
battery

Section 10 Describe inhibit configuration
of 80Wh battery

Section 12 Correct information error in
## Table 12-6

Section 15 Add reminder to use standoffs
with 80Wh battery

Section 16 Add 80Wh battery to
compatibility matrix

D 17 Apr 2017 Section 5 Update storage temperatures
and that unit is shipped with
inhibit shorting links in place.
## DCR038
Section 7 Correct typographical error –
80Wh battery P/N in Table 7-1

Section 8 Clarify wording of CSK header
pin use

E 26 Jun 2018 Table 8-1 H1 - HEADER 1 column
Pin 41 Bus name changed
from NC to I2C_DATA
Pin 43 Bus name changed
from NC to I2C_CLK
## DCR170



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 3 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## Related Products
Assembly # Assembly name Notes
01-02683 10Wh Standalone Battery
01-02684 20Wh Standalone Battery
01-02685 30Wh Standalone Battery
01-02686 40Wh Standalone Battery
01-02687 80Wh Standalone Battery
01-02681 10Wh Integrated Battery
01-02682 20Wh Integrated Battery

## Related Documents
## No. Document Name Doc Ref.
## RD-1 3
rd
Generation EPS No Inhibits User Manual
## USM-1335
RD-2 NASA General Environmental Verification Standard GSFC-STD-7000 April 2005
## RD-4
Use of the Clyde Space 3rd Generation CubeSat Battery
on manned missions TN-1404


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 4 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

Acronyms and Abbreviations
## Ah Ampere Hour
AIT Assembly, Integration and Testing
BCR Battery Charge Regulator
DoD Depth of Discharge
EoC End of Charge
EPS Electrical Power System
ESD Electro Static Discharge
## Isc Short Circuit Current
MPPT Maximum Power Point Tracker
PDM Power Distribution Module
rh Relative Humidity
TTC Telemetry and Telecommand
## Voc Open Circuit Voltage
## Wh Watt Hour




## #

## Warning

## Risk

Ensure headers H1 and H2 are correctly aligned before
mating boards
If  misaligned, battery  positive  can  short  to  ground,
causing failure of the battery and EPS

Observe ESD precautions at all times
The  battery  is  a  static  sensitive  system.    Failure  to
observe  ESD  precautions  can  result  in  failure  of  the
battery

Ensure not to exceed the maximum stated limits
Exceeding any of the stated maximum limits can result
in failure of the battery

Ensure batteries are fully isolated during storage
If   not   fully   isolated   (by   switch   configuration   or
separation)  the  battery  may  over-discharge,  resulting
in failure of the battery.  This is particularly important
for the integrated battery

No  connection  should  be  made  to H2.35 – H2.36  and
## H2.41 – H2.44
These unprotected pins   are   used   to connect   the
battery to the EPS.  Any connections to the unregulated
battery bus should be made to pins H2.45-H2.46

H1 and H2 pins should not be shorted at any time
These headers have exposed live pins which should not
be shorted at any time.  Particular care should be taken
regarding the surfaces these are placed on.

Battery should only be operated when integrated with
an EPS
The EPS includes a number of protection circuits for the
battery.  Operation without these protections may lead
to damage of the batteries

Do not discharge batteries below 6V
If  the  battery  is  discharged  to  a  voltage  below  6V  the
cells  have  been  compromised  and  will  no  longer  hold
capacity

If  batteries  are  over-discharged  DO  NOT  attempt  to
recharge
If  the  battery  is  over  discharged  (below  6V)  it  should
not be recharged as this may lead to cell rupture.



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 5 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## CONTENTS
Contents ........................................................................................................................................... 5
1 Introduction .......................................................................................................................... 7
1.1 Additional Information Available Online ............................................................................................ 7
1.2 Continuous Improvement .................................................................................................................. 7
2 Overview ............................................................................................................................... 8
2.1 Products Covered ............................................................................................................................... 8
3 Maximum Ratings ................................................................................................................. 9
4 Electrical and Physical Characteristics ................................................................................ 11
4.1 10Wh Standalone Battery ................................................................................................................ 11
4.2 20Wh Standalone Battery ................................................................................................................ 12
4.3 30Wh Standalone Battery ................................................................................................................ 13
4.4 40Wh Standalone Battery ................................................................................................................ 14
4.5 10Wh Integrated Battery ................................................................................................................. 16
4.6 20Wh Integrated Battery ................................................................................................................. 17
5 Handling and storage .......................................................................................................... 18
5.1 Electro Static Discharge (ESD) Protection ........................................................................................ 18
5.2 General Handling ............................................................................................................................. 18
5.3 Shipping and Storage ....................................................................................................................... 18
6 Materials and Processes ..................................................................................................... 19
6.1 Materials Used ................................................................................................................................. 19
6.2 Processes and Procedures ............................................................................................................... 19
7 System Description ............................................................................................................. 20
7.1 3G Battery Family ............................................................................................................................. 20
7.2 Protection and Redundancy ............................................................................................................. 21
7.3 Quiescent Power Consumption ....................................................................................................... 21
7.4 Dimensions ...................................................................................................................................... 22
8 Interfacing ........................................................................................................................... 30
8.1 Standalone Connector Layout and Mounting Locations .................................................................. 30
8.2 Integrated Battery Connector Layout .............................................................................................. 31
8.3 3G Standalone Battery Bus Headers ................................................................................................ 32
8.4 3G Integrated Battery Bus Headers ................................................................................................. 32
8.5 3G Standalone Battery Header Pinouts ........................................................................................... 33
8.6 EPS and Battery Integration ............................................................................................................. 34
8.7 Buses ................................................................................................................................................ 34
8.8 Grounding ........................................................................................................................................ 34
9 Charge and Discharge Modes ............................................................................................. 36
9.1 Discharge ......................................................................................................................................... 36
9.2 Lot Acceptance Testing .................................................................................................................... 36

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 6 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

10 General protection ............................................................................................................. 41
10.1 Protection Overview ........................................................................................................................ 41
10.2 End of Charge ................................................................................................................................... 42
10.3 Over-Current Bus Protection (OCP).................................................................................................. 42
10.4 Battery Under-voltage Protection (UVP) .......................................................................................... 42
10.5 Cell Level Protection Circuit ............................................................................................................. 42
10.6 Over-current Polyswitch Protection ................................................................................................. 45
10.7 Inhibit Operation .............................................................................................................................. 45
10.8 Overvoltage protection (OVP) .......................................................................................................... 46
11 Heater Operation ................................................................................................................ 47
12 Telemetry and Telecommand ............................................................................................. 48
12.1 Communications .............................................................................................................................. 48
12.2 Command Protocol .......................................................................................................................... 48
12.3 List of Available Commands ............................................................................................................. 49
12.4 Housekeeping and Status Commands .............................................................................................. 50
12.5 Telemetry ......................................................................................................................................... 53
12.6 Get Telemetry (0x10) ....................................................................................................................... 54
12.7 Watchdogs and Reset Counters ....................................................................................................... 56
12.8 Heater Commands ........................................................................................................................... 57
13 Launch Provider Information .............................................................................................. 58
15 Test ..................................................................................................................................... 59
15.1 Solar Array Input .............................................................................................................................. 60
15.2 Power Up/Down Procedure ............................................................................................................. 60
15.3 Configuration and Testing ................................................................................................................ 60
16 Compatible Systems ........................................................................................................... 61




## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 7 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 1 INTRODUCTION
This document provides information on the features, operation, handling and storage of Clyde Space
manned flight ISS compatible 3G battery family. The manned flight ISS compatible range utilise inhibits
included in the battery which allow  for efficient and safe battery switching to comply with manned
flight and ISS regulations. Several different capacities and form factors are available. The battery family
is designed to integrate with a suitable EPS and solar arrays to form a complete power system for a
CubeSat.


## Figure 1-1 Complete Power System Diagram

## 1.1 Additional Information Available Online
Additional information on CubeSats and Clyde Space Systems can be found at www.clyde.space.

## 1.2 Continuous Improvement
At Clyde Space  we are continuously  improving our processes  and products.  We aim  to provide  full
visibility of the changes and updates that we make, and information of these changes can be found by
visiting our website: www.clyde.space.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 8 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 2 OVERVIEW
The Clyde  Space  CubeSat Battery range  has  been developed  by  our  team  of highly  experienced
Spacecraft Power Systems and Electronics Engineers.
Since introducing the first generation in 2006, Clyde Space has shipped over 500 battery systems to
customers in Europe, Asia and North America. The batteries utilise Lithium Ion Polymer technology to
offer world leading power to mass ratios in a form factor ideally suited to the volume constraints of
CubeSats.  In addition to this, testing has been carried out by both ESA and NASA.
Clyde Space is the World leading supplier of power system components for CubeSats. We have been
designing, manufacturing, testing and supplying batteries, power system electronics and solar panels
for  space  programmes  since  2006.  Our  customers  range  from  universities  running  student  led
missions, to major space companies and government organisations.

## 2.1 Products Covered
## Battery Product Code Notes
10Wh Standalone 01-02683
The standalone battery interfaces with the EPS within
the stack using the standard CubeSat PC104 interface.
20Wh Standalone 01-02684
30Wh Standalone 01-02685
40Wh Standalone 01-02686
80Wh Standalone 01-02687
The 80Wh standalone battery comprises two 40Wh
standalone batteries, each of which can be placed
anywhere in the stack using the PC104 interface.
10Wh Integrated 01-02681
An integrated battery is mated to a Clyde Space EPS as a
daughterboard and uses a single PC104 interface.
20Wh Integrated
## 01-02682


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 9 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 3 MAXIMUM RATINGS
## 1


## MAX RATINGS OVER OPERATING TEMPERATURE RANGE (UNLESS OTHERWISE STATED)


## Value Unit
10Whr

## Charge Limits
Voltage max 8.4 V
Current max 2 A
Current Rate max 1.53C Fraction of Capacity
## Discharge Limits
Voltage min 6.2 V
Current max 2 A
Current Rate max 1.53C Fraction of Capacity
20Whr

## Charge Limits
Voltage max 8.4 V
Current max 4 A
Current Rate max 1.53C Fraction of Capacity
## Discharge Limits
Voltage min 6.2 V
Current max 4 A
Current Rate max 1.53C Fraction of Capacity
30Whr

## Charge Limits
Voltage max 8.4 V
Current max 6 A
Current Rate max 1.53C Fraction of Capacity
## Discharge Limits
Voltage min 6.2 V
Current max 6 A
Current Rate max 1.53C Fraction of Capacity
40Whr

## Charge Limits
Voltage max 8.4 V
Current max 8 A
Current Rate max 1.53C Fraction of Capacity
## Discharge Limits
Voltage min 6.2 V
Current max 8 A
Current Rate max 1.53C Fraction of Capacity
## 80
## Whr

## Charge Limits
Voltage max 8.4 V
Current max 16 A
Current Rate max 1.53C Fraction of Capacity
## Discharge Limits
Voltage min 6.2 V
Current max 16 A
Current Rate max 1.53C Fraction of Capacity


## 1
Stresses beyond  those  listed  under  maximum  ratings may  cause  permanent  damage  to  the Battery.    These  are  the  stress  ratings  only.
Operation of the Battery at conditions beyond those indicated is not recommended.  Exposure to absolute maximum ratings for extended
periods may affect reliability

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 10 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## MAX RATINGS OVER OPERATING TEMPERATURE RANGE (UNLESS OTHERWISE STATED)
## Value Unit
## All

Operating Temperature -10 to 50 °C
## Storage Temperature
Recommended:   -10 to +10
1 Year:       -20 to +20
3 Months: -20 to +45
1 Month:   -20 to +60
## °C
## Vacuum  10
## -5
torr
Vibration  To [RD-2]

Table 3-1 Maximum Ratings of Clyde Space Batteries




## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 11 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 4 ELECTRICAL AND PHYSICAL CHARACTERISTICS
4.1 10Wh Standalone Battery
## Description Notes Min Typical Max Unit
## Charge Conditions


EoC Voltage  8.22 8.26 8.3 V
Charge Current Recommended maximum C/2 -- 0.65 -- A
## Discharge Conditions


## Full Discharge Voltage  6.16 6.2 6.24 V
Discharge Current Recommended maximum C/2 -- 0.65 -- A
Depth of Discharge Recommended  -- 20% -- N/A
## Capacity

Discharge rate C/15
-20°C -- 1.21 -- Ah
0°C -- 1.31 -- Ah
20°C -- 1.35 -- Ah
40°C -- 1.34 -- Ah
Discharge rate C/10
-20°C -- 1.19 -- Ah
0°C -- 1.29 -- Ah
20°C -- 1.34 -- Ah
40°C -- 1.38 -- Ah
Discharge rate C/5
-20°C -- 1.13 -- Ah
0°C -- 1.29 -- Ah
20°C -- 1.34 -- Ah
40°C -- 1.34 -- Ah
Discharge rate C/2
-20°C -- 0.83 -- Ah
0°C -- 1.2 -- Ah
20°C -- 1.26 -- Ah
40°C -- 1.3 -- Ah
## Operating Conditions
## Quiescent Power
## Consumption
Draw from 3V3 (and negligible power
from 5V) -- -- < 0.1 W
## Heater


Power Draw Heater active (3V3 powered heater) -- 0.20 -- W
## Temperature
Enable heater -- 1 -- °C
Disable heater -- 6.5 -- °C
## Communications
## Protocol

## -- I
## 2
## C --

Transmission speed

## -- 100 -- Kbits
## -1

Bus voltage

## 3.26V 3.3V 3.33V

Node address

-- 0x2A -- Hex
Address scheme

## -- 7bit --

Node operating freq.

-- 27MHz --

## Physical


## Dimensions
Height from    top PCB to    lowest
component
9.95 mm
Mass  115 121 127 g
Table 4-4-1 Performance Characteristics of the 10Wh Standalone Battery (01-02683)

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 12 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

4.2 20Wh Standalone Battery
## Description Notes Min Typical Max Unit
## Charge Conditions


EoC Voltage  8.22 8.26 8.3 V
Charge Current Recommended maximum C/2 -- 1.3 -- A
## Discharge Conditions


## Full Discharge Voltage  6.16 6.2 6.24 V
Discharge Current Recommended maximum C/2 -- 1.3 -- A
Depth of Discharge Recommended  -- 20% -- Capacity
## Capacity

Discharge rate C/15
-20°C -- 2.42 -- Ah
0°C -- 2.62 -- Ah
20°C -- 2.69 -- Ah
40°C -- 2.68 -- Ah
Discharge rate C/10
-20°C -- 2.38 -- Ah
0°C -- 2.59 -- Ah
20°C -- 2.68 -- Ah
40°C -- 2.77 -- Ah
Discharge rate C/5
-20°C -- 2.26 -- Ah
0°C -- 2.58 -- Ah
20°C -- 2.67 -- Ah
40°C -- 2.68 -- Ah
Discharge rate C/2
-20°C -- 0.83 -- Ah
0°C -- 1.2 -- Ah
20°C -- 1.26 -- Ah
40°C -- 1.3 -- Ah
## Operating Conditions
## Quiescent Power
## Consumption
Draw from 3V3 (and negligible power
from 5V)
## -- -- < 0.1 W
## Heater
Power Draw Heater active (3V3 powered heater) -- 0.40 -- W
## Temperature
Enable heater -- 1 -- °C
Disable heater -- 6.5 -- °C
## Physical


## Dimensions
Height from   top PCB   to   lowest
component
15.75 mm
Mass  186 196 205 g
Table 4-4-2 Performance Characteristics of the 20Wh Standalone Battery (01-02684)



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 13 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

4.3 30Wh Standalone Battery
## Description Notes Min Typical Max Unit
## Charge Conditions


EoC Voltage  8.22 8.26 8.3 V
Charge Current Recommended maximum C/2 -- 1.95 -- A
## Discharge Conditions


## Full Discharge Voltage  6.16 6.2 6.24 V
Discharge Current Recommended maximum C/2 -- 1.95 -- A
Depth of Discharge Recommended  -- 20% -- Capacity
## Capacity

Discharge rate C/15
-20°C -- 3.64 -- Ah
0°C -- 3.92 -- Ah
20°C -- 4.04 -- Ah
40°C -- 4.02 -- Ah
Discharge rate C/10
-20°C -- 3.57 -- Ah
0°C -- 3.88 -- Ah
20°C -- 4.03 -- Ah
40°C -- 4.15 -- Ah
Discharge rate C/5
-20°C -- 3.4 -- Ah
0°C -- 3.87 -- Ah
20°C -- 4.01 -- Ah
40°C -- 4.02 -- Ah
Discharge rate C/2
-20°C -- 2.49 -- Ah
0°C -- 3.59 -- Ah
20°C -- 3.78 -- Ah
40°C -- 3.89 -- Ah
## Operating Conditions
## Quiescent Power
## Consumption
Draw  from  3V3 (and  negligible  power
from 5V)
## -- -- < 0.1 W
## Heater
Power Draw Heater active (3V3 powered heater) -- 0.60 -- W
## Temperature
Enable heater -- 1 -- °C
Disable heater -- 6.5 -- °C
## Physical


## Dimensions
Height from    top PCB    to    lowest
component
21.55 mm
Mass  254 268 281 g
Table 4-4-3 Performance Characteristics of the 30Wh Standalone Battery (01-02685)

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 14 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

4.4 40Wh Standalone Battery
## Description Notes Min Typical Max Unit
## Charge Conditions


EoC Voltage  8.22 8.26 8.3 V
Charge Current Recommended maximum C/2 -- 2.6 -- A
## Discharge Conditions


## Full Discharge Voltage  6.16 6.2 6.24 V
Discharge Current Recommended maximum C/2 -- 2.6 -- A
Depth of Discharge Recommended  -- 20% -- Capacity
## Capacity

Discharge rate C/15
-20°C -- 4.85 -- Ah
0°C -- 5.23 -- Ah
20°C -- 5.39 -- Ah
40°C -- 5.36 -- Ah
Discharge rate C/10
-20°C -- 4.76 -- Ah
0°C -- 5.18 -- Ah
20°C -- 5.37 -- Ah
40°C -- 5.53 -- Ah
Discharge rate C/5
-20°C -- 4.53 -- Ah
0°C -- 5.16 -- Ah
20°C -- 5.34 -- Ah
40°C -- 5.36 -- Ah
Discharge rate C/2
-20°C -- 3.32 -- Ah
0°C -- 4.79 -- Ah
20°C -- 5.04 -- Ah
40°C -- 5.19 -- Ah
## Operating Conditions
## Quiescent Power
## Consumption
Draw from 3V3 (and negligible power
from 5V)
## -- -- < 0.1 W
## Heater
Power Draw Heater active (3V3) -- 0.80 -- W
## Temperature
Enable heater -- 1 -- °C
Disable heater -- 6.5 -- °C
## Physical


## Dimensions
Height from   top PCB   to   lowest
component
27.35 mm
Mass  318 335 351 g
Table 4-4-4 Performance Characteristics of the 40Wh Standalone Battery 01-02686)


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 15 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

4.5 80Wh Standalone Battery
## Description Notes Min Typical Max Unit
## Charge Conditions


EoC Voltage  8.22 8.26 8.3 V
Charge Current Recommended maximum C/2 -- 5.2 -- A
## Discharge Conditions


## Full Discharge Voltage  6.16 6.2 6.24 V
Discharge Current Recommended maximum C/2 -- 5.2 -- A
Depth of Discharge Recommended  -- 20% -- Capacity
## Capacity

Discharge rate C/15
-20°C -- 9.7 -- Ah
0°C -- 10.46 -- Ah
20°C -- 10.78 -- Ah
40°C -- 10.72 -- Ah
Discharge rate C/10
-20°C -- 9.52 -- Ah
0°C -- 10.36 -- Ah
20°C -- 10.74 -- Ah
40°C -- 11.06 -- Ah
Discharge rate C/5
-20°C -- 9.06 -- Ah
0°C -- 10.32 -- Ah
20°C -- 10.68 -- Ah
40°C -- 10.72 -- Ah
Discharge rate C/2
-20°C -- 6.64 -- Ah
0°C -- 9.58 -- Ah
20°C -- 10.08 -- Ah
40°C -- 10.38 -- Ah
## Operating Conditions
## Quiescent Power
## Consumption
Draw from 3V3 (and negligible power
from 5V)
## -- -- < 0.2 W
## Heater
Power Draw Heater active (3V3) -- 1.60 -- W
## Temperature
Enable heater -- 1 -- °C
Disable heater -- 6.5 -- °C
## Physical


## Dimensions
Height from   top PCB   to   lowest
component
## 56.94
## 1
mm
## Mass  636 670
## 2
702 g
Table 4-4-5 Performance Characteristics of the 80Wh Standalone Battery (01-02687)

## 1
Assumes batteries are stacked as close as possible in the stack, as shown in Section 7.4.5.
## 2
Mass excludes any standoffs

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 16 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

4.6 10Wh Integrated Battery
## Description Notes Min Typical Max Unit
## Charge Conditions


EoC Voltage  8.22 8.26 8.3 V
Charge Current Recommended maximum C/2 -- 0.65 -- A
## Discharge Conditions


## Full Discharge Voltage  6.16 6.2 6.24 V
Discharge Current Recommended maximum C/2 -- 0.65 -- A
Depth of Discharge Recommended  -- 20% -- Capacity
## Capacity

Discharge rate C/15
-20°C -- 1.21 -- Ah
0°C -- 1.31 -- Ah
20°C -- 1.35 -- Ah
40°C -- 1.34 -- Ah
Discharge rate C/10
-20°C -- 1.19 -- Ah
0°C -- 1.29 -- Ah
20°C -- 1.34 -- Ah
40°C -- 1.38 -- Ah
Discharge rate C/5
-20°C -- 1.13 -- Ah
0°C -- 1.29 -- Ah
20°C -- 1.34 -- Ah
40°C -- 1.34 -- Ah
Discharge rate C/2
-20°C -- 0.83 -- Ah
0°C -- 1.2 -- Ah
20°C -- 1.26 -- Ah
40°C -- 1.3 -- Ah
## Operating Conditions
## Quiescent Power
## Consumption
Draw from 3V3 (and negligible power
from 5V)
## -- -- < 0.1 W
## Heater
Power Draw Heater active (3V3 powered heater) -- 0.20 -- W
## Temperature
Enable heater -- 1 -- °C
Disable heater -- 6.5 -- °C
Physical (without EPS)


## Dimensions
Height from   top PCB   to   lowest
component
## 9.95
mm
Mass  81 85 89 g
Physical (with EPS)
## Dimensions
Height from   top PCB   to   lowest
component
19.6 mm
Mass  166 171 175 g
Table 4-4-6 Performance Characteristics of the 10Wh Integrated Battery (01-02681)


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 17 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

4.7 20Wh Integrated Battery
## Description Notes Min Typical Max Unit
## Charge Conditions


EoC Voltage  8.22 8.26 8.3 V
Charge Current Recommended maximum C/2 -- 1.3 -- A
## Discharge Conditions


## Full Discharge Voltage  6.16 6.2 6.24 V
Discharge Current Recommended maximum C/2 -- 1.3 -- A
Depth of Discharge Recommended  -- 20% -- Capacity
## Capacity

Discharge rate C/15
-20°C -- 2.42 -- Ah
0°C -- 2.62 -- Ah
20°C -- 2.69 -- Ah
40°C -- 2.68 -- Ah
Discharge rate C/10
-20°C -- 2.38 -- Ah
0°C -- 2.59 -- Ah
20°C -- 2.68 -- Ah
40°C -- 2.77 -- Ah
Discharge rate C/5
-20°C -- 2.26 -- Ah
0°C -- 2.58 -- Ah
20°C -- 2.67 -- Ah
40°C -- 2.68 -- Ah
Discharge rate C/2
-20°C -- 0.83 -- Ah
0°C -- 1.2 -- Ah
20°C -- 1.26 -- Ah
40°C -- 1.3 -- Ah
## Operating Conditions
## Quiescent Power
## Consumption
Draw from 3V3 (and negligible
power from 5V)
## -- -- < 0.1 W
## Heater


Power Draw Heater active (3V3) -- 0.40 -- W
## Temperature
Enable heater -- 1 -- °C
Disable heater -- 6.5 -- °C
Physical (without EPS)


## Dimensions
Height from   top PCB   to   lowest
component
## 15.75
mm
Mass  152 160 168 g
Physical (with EPS)
## Dimensions
Height from   top PCB   to   lowest
component
27.41 mm
Mass  238 246 254 g
Table 4-4-7 Performance Characteristics of the 20Wh Integrated Battery (01-02682)



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 18 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 5 HANDLING AND STORAGE
The  batteries require specific  guidelines  to  be  observed  for handling,  transportation  and storage.
These  are stated below.  Failure  to  follow the guidelines may  result  in  damage  to  the  units  or
degradation in performance.

5.1 Electro Static Discharge (ESD) Protection
The batteries incorporate static sensitive devices and care should be taken during handling.  Do not
touch the batteries without proper electrostatic protection in place. All work carried out on the system
should be done in a static dissipative environment.

## 5.2 General Handling
The batteries  are robust and designed  to  withstand  flight  conditions.  However, care must  be  taken
when  handling  the  device.  Do  not  drop the  device  as  this  can  damage  the cells.  There  are  live
connections  between  the  battery  systems  and  the batteries on  the  CubeSat  Kit  headers.    All  metal
objects (including probes) should be kept clear of these headers.
Gloves should be worn when handling all flight hardware.
Flight  hardware  should  only  be  removed  from  packaging  in  a  class  100000  (or  better)  clean  room
environment.
The  exterior  surface  of  the  cells is covered  with  space  grade  Kapton  adhesive  tape;  this  provides
insulation for the cells and is not to be removed.
Note: The  inhibits must  be  enabled  while  the system  is  not in  use to  prevent  battery  discharge –
even  with  no  load  connected,  there  is  a  small  current  draw  from  the  battery.  This  is  particularly
important for the integrated battery as the EPS is always mated to the battery.

5.3 Shipping and Storage
The devices are shipped in anti-static and enclosed in a hard protective case.  This case should be used
for storage.  All hardware should be stored in anti-static containers.
The unit is shipped with links shorting the inhibits to prevent battery discharge during shipping.
Rate  of  capacity  degradation  of  lithium  polymer  cells  in  storage  is  dependent  on  the  storage
environment, particularly temperature, and cell state of charge.  It is recommended that the batteries
are stored with voltages approximately 7.6V (50% DoD), at a temperature between -10°C and +10°C
and  in  a  humidity-controlled  environment  of  40-60%rh.  The  product  can  be  stored  at  other
temperatures defined within the non-operational range but may experience a reduced shelf life under
these conditions. The cell should be allowed to return to >10°C before attempting to charge the cell.
The most serious degradation occurs when cells are stored in a fully charged state.
If batteries are stored for long periods of time, they may over discharge.  To prevent this, batteries
should be charged periodically to maintain ≈7.6V. It is also essential that inhibits on the battery or an
EPS are left in place/replaced during periods of storage.
The shelf-life of this product is estimated at 5 years when stored appropriately.



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 19 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 6 MATERIALS AND PROCESSES
## 6.1 Materials Used

Material Manufacturer %TML %CVCM %WVR Application
Araldite 2014 Epoxy Huntsman 0.97 0.05 0.33 Adhesive fixing
## Arathane 5750 Huntsman 0.41 0.03 - Conformal Coating
DC 6-1104 Dow Corning 0.17 0.02 0.06 Adhesive fixing on
modifications
## Stycast 5952
## Emerson &
## Cuming
1.38 0.62 0.01 Thermally Conductive RTV
Stycast 2850 Henkel Electronics 0.25 0.02 - IP protective layer
PCB material

## FR4 0.62 0 0.1
Note: worst case on NASA
out-gassing list
## Solder Resist

## CARAPACE
EMP110 or
## XV501T-4
## 0.95
or 0.995
## 0.02
## Or 0.001
## 0.31 -
Solder Sn62 or Sn63
(Tin/Lead)
## - - - -
## Flux
## Alpha  Rosin Flux,
## RF800, ROL 0
- - - ESA Recommended
## Table 6-1 Materials List


## Part Used Manufacturer Contact Insulator Type Use Required Mating Connector
ESQ-126-39-G-D Samtec Gold Plated Black Glass
## Filled
## Polyester
PTH CubeSat
## Header
## Stack Connector
ESQ-126-39-G-D Samtec Gold Plated Black Glass
## Filled
## Polyester
PTH CubeSat
## Header
## Stack Connector
DF13-6P-125DSA Hirose Gold Plated Polyamide PTH Programming
## Header
## DF13-6S-1.25C
DF13-2P-125DSA Hirose Gold Plated Polyamide PTH Separation
## Switch
## DF13-2S-1.25C
DF13-2P-125H Hirose Gold Plated Polyamide PTH Separation
## Switch
## DF13-2S-1.25C
DF13-3P-125H Hirose Gold Plated Polyamide PTH RBF DF13-3S-1.25C
DF13-8P-1.25DSA Hirose Gold Plated Polyamide PTH Overvoltage
resistor
## DF13-8S-1.25C
## Table 6-2 Connector Headers

6.2 Processes and Procedures
All assembly is carried out to IPC610 Class 3 standard.





## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 20 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 7 SYSTEM DESCRIPTION
The Clyde Space 3G battery family is optimised for Low Earth Orbit (LEO) missions with a maximum
altitude  of  850km.   The batteries  are designed  for  integration  with spacecraft  that  have an  EPS
compatible with lithium ion polymer technology.
Clyde  Space  batteries  offer  high  capacity  with  low mass and  volume.  The  battery  systems  all  have
autonomous integrated heater systems to enhance operation at low temperatures.
The battery heater is an independent analogue circuit which maintains the battery temperature above
0°C. The heater is thermostatically controlled to automatically turn on when the battery temperature
falls below 1°C, and switch off again when the temperature rises above 5°C.

Figure 7-1  3G Standalone Battery Configuration

Figure 7-2  3G Integrated Battery Configuration

7.1 3G Battery Family
The 3G manned flight ISS compatible battery family has a variety of different configurations. The two
form  factors  available  are  the  standalone  battery  and  integrated  battery.  The  standalone  battery
interfaces with the EPS within the stack using the standard CubeSat PC104 interface.  The integrated
battery is  mated  to a  Clyde  Space  EPS as  a daughterboard.    This  provides  tighter  mechanical
integration and uses a single PC104 interface for both the EPS and battery.

## Battery Product Code Notes
10Wh Standalone 01-02683
The standalone battery interfaces with the EPS within
the stack using the standard CubeSat PC104 interface.
20Wh Standalone 01-02684
30Wh Standalone 01-02685
40Wh Standalone 01-02686
80Wh Standalone
01-02687 The 80Wh standalone battery comprises two 40Wh
standalone batteries, each of which can be placed
anywhere in the stack using the PC104 interface.
10Wh Integrated 01-02681
An integrated battery is mated to a Clyde Space EPS as a
daughterboard and uses a single PC104 interface.
20Wh Integrated 01-02682
Table 7-1 3G Battery Manned Flight family configurations


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 21 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

7.2 Protection and Redundancy
All Clyde Space batteries have as standard multiple protection systems in place at a cell, battery, and
system-level  which  will  automatically  respond  to  external  fault  conditions  in  order  to  protect  the
battery and wider-system from irrecoverable damage.
For a full breakdown of the protection systems associated with this product and their operation refer
to section 10 General protection.

## 7.3 Quiescent Power Consumption
The quiescent power consumption of the battery is ≈ 0.1W.  This power is drawn from the 3.3V and
5V available on the header to power the heater control circuitry.

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 22 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 7.4 Dimensions
7.4.1 10Whr Standalone


Figure 7-3 10Whr (01-02683) Standalone Battery External Dimensions


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 23 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

7.4.2 20Whr Standalone


Figure 7-4 20Whr (01-02684) Standalone Battery External Dimensions




## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 24 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

7.4.3 30Whr Standalone


Figure 7-5 30Whr (01-02685) Standalone Battery External Dimensions



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 25 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

7.4.4 40Whr Standalone


Figure 7-6 40Whr (01-02686) Standalone Battery External Dimensions




## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 26 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

7.4.5 80Wh Standalone

Figure 7-7 80Whr (01-02687) Standalone Battery External Dimensions (Example stack)
The 80Wh battery is supplied as two separate 40Wh batteries, which may be stacked as desired by
the customer. A recommended configuration, in which the batteries are stacked adjacent to each
other and connected with ESQ-126-38-G-D headers (supplied with the assembly), is shown above.
Standoffs are not shown in this drawing as they will vary depending on the design of the satellite,
but must be used to space the batteries apart in the stack. Minimum spacing, as governed by

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 27 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

acceptable distances between boards, is shown here. Maximum spacing is governed by connector
insertion depths. Refer to connector datasheets for more information.

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 28 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

7.4.6 10Whr Integrated



Figure 7-8 10Whr (01-02681) Integrated Battery and EPS External Dimensions



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 29 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

7.4.7 20Whr Integrated



Figure 7-9 20Whr (01-02682) Integrated Battery and EPS External Dimensions



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 30 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 8 INTERFACING
8.1 Standalone Connector Layout and Mounting Locations



## Connector Function
## H2 Main Bus Header
## H1 Main Bus Header
## J1 High Side Inhibit 1
## J2 High Side Inhibit 2
## J3 High Side Inhibit 3
## J4 High Side Inhibit 4
## J7 Low Side Inhibit
## J12 Remove Before Flight
J8 Overvoltage shunt resistor interface
J1_IC1 Programming Header – not for customer use

Figure 8-1 Standalone battery external connector layout and functions



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 31 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 8.2 Integrated Battery Connector Layout
The integrated battery must be used in combination with a compatible EPS; for mounting hole locations refer to
the applicable EPS user manual.


## Connector Function
## J1 High Side Inhibit 1
## J2 High Side Inhibit 2
## J3 High Side Inhibit 3
## J4 High Side Inhibit 4
## J7 Low Side Inhibit
J8 Overvoltage shunt resistor interface
## J10 Remove Before Flight
J1_IC1 Programming Header – not for customer use

Not shown in this diagram are the EPS connections.
These can be found in USM-1335.
Figure 8-2 Integrated battery external connector layout and functions
(left – battery motherboard top view, right – bottom view)


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 32 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

8.3 3G Standalone Battery Bus Headers
Connections  from  the battery to  the  bus  of  the  satellite  are  made  via  the CubeSat Kit compatible
header H1 and H2.

Figure 8-3  CubeSat Kit Header Schematic
Please note that pins marked for IDIODE_OUT and PCM_IN use highlighted in the above diagram are
used internally between the EPS and the battery. Placing any other connection on these pins can result
in the incorrect operation or terminal failure of the EPS or battery.
In addition to the pins highlighted above there are additional pins allocated on the EPS header which
must  be  taken  into  account  when  making connections  to  the  stack  header  of  a  combined  EPS  and
standalone battery assembly. For a list of header pin allocations for the EPS refer to the applicable EPS
user manual.

8.4 3G Integrated Battery Bus Headers
These can be found in USM-1335.

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 33 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

8.5 3G Standalone Battery Header Pinouts

## H1 - HEADER 1

## H2 - HEADER 2
## Pin Name Use Notes Pin Name Use Notes
## 1 NC - - 1 NC - -
## 2 NC - - 2 NC - -
## 3 NC - - 3 NC - -
## 4 NC - - 4 NC - -
## 5 NC - - 5 NC - -
## 6 NC - - 6 NC - -
## 7 NC - - 7 NC - -
## 8 NC - - 8 NC - -
## 9 NC - - 9 NC - -
## 10 NC - - 10 NC - -
## 11 NC - - 11 NC - -
## 12 NC - - 12 NC - -
## 13 NC - - 13 NC - -
## 14 NC - - 14 NC - -
## 15 NC - - 15 NC - -
## 16 NC - - 16 NC - -
## 17 NC - - 17 NC - -
## 18 NC - - 18 NC - -
## 19 NC - - 19 NC - -
## 20 NC - - 20 NC - -
## 21 NC - - 21 NC - -
## 22 NC - - 22 NC - -
## 23 NC - - 23 NC - -
## 24 NC - - 24 NC - -
25 NC - - 25 5VBUS 5V Bus Power Bus
26 NC - - 26 5VBUS 5V Bus Power Bus
27 NC - - 27 3V3BUS 3V3 Bus Power Bus
28 NC - - 28 3V3BUS 3V3 Bus Power Bus
29 NC - - 29 GND Ground System Ground
30 NC - - 30 GND Ground System Ground
## 31 NC - - 31 NC - -
32 NC - - 32 GND Ground System Ground
33 NC - - 33 Reserved Do Not Use -
34 NC - - 34 Reserved Do Not Use -
35 NC - - 35 PCM_IN Power Convertor In Supply to Power Bus Regulators
36 NC - - 36 PCM_IN Power Convertor In Supply to Power Bus Regulators
## 37 NC - - 37 NC - -
## 38 NC - - 38 NC - -
## 39 NC - - 39 NC - -
## 40 NC - - 40 NC - -
## 41 I2C_DATA I
## 2
## C SDA I
## 2
C Data Line 41 IDIODE_OUT Ideal Diode Output Solar Array Regulator Output
42 NC - - 42 IDIODE_OUT Ideal Diode Output Solar Array Regulator Output
## 43 I2C_CLK I
## 2
## C SCL I
## 2
C Clock Line 43 IDIODE_OUT Ideal Diode Output Solar Array Regulator Output
44 NC - - 44 IDIODE_OUT Ideal Diode Output Solar Array Regulator Output
45 NC - - 45 BatVBUS Unregulated Battery Bus Power Bus
46 NC - - 46 BatVBUS Unregulated Battery Bus Power Bus
## 47 NC - - 47 NC - -
## 48 NC - - 48 NC - -
## 49 NC - - 49 NC - -
## 50 NC - - 50 NC - -
## 51 NC - - 51 NC - -
## 52 NC - - 52 NC - -
Table 8-1  Pin Descriptions for 3G Standalone Battery Header H1 and H2


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 34 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

8.6 EPS and Battery Integration
8.6.1 Standalone Battery (10Wh-40Wh)
Connection of the  battery systems to the EPS is via the  main bus headers. Ensure that the pins are
aligned,  and  located  in  the  correct  position,  as  any  offset  can  cause  the  battery  to  be  shorted  to
ground, leading to catastrophic failure of the battery and damage to the EPS. Failure to observe these
precautions will result in the voiding of any warranty.

Ensure that the battery is fully isolated during periods of extended storage.
When a battery board is connected to the header, there are live battery pins accessible (H2.35 – H2.36
and H2.41 – H2.44). These pins should not be routed to any connections other than the Clyde Space
EPS, otherwise protections will be bypassed and significant battery damage may be sustained.
8.6.2 Standalone Battery (80Wh)
Integration of the 80Wh batteries is as described in section 8.6.1, except that the 80Wh is supplied as
two  separate 40Wh  batteries,  each  with  its  own  PC104  header. The  two  batteries  can  be  mounted
adjacent   in   the   stack,   using   the   supplied   spacing   header   to connect them -- mechanical
spacers/standoffs vary between applications and are thus left as the customer’s responsibility.
Alternatively, the individual 40Wh units can be placed separately in the stack as desired.
The two batteries should be stored in the same conditions and used together so that degradations of
the individual units are matched.
## 8.6.3 Integrated Battery
The  customer  will  receive  an  integrated  battery  already  fully  integrated  with  an  EPS,  however,  the
battery positive pins on the EPS are still exposed. Failure to isolate the battery will allow the battery
to discharge into the EPS and cause battery failure once the battery voltage drops below the minimum
value.
Further details about safe handling can be found in the EPS user manual.
## 8.7 Buses
The 3V3 and 5V power buses must be supplied to the battery from the EPS to power the I
## 2
C node and
heater control circuitry.
## 8.8 Grounding
To  ensure optimum operation  it  is  recommended  that  a  star  grounding  scheme  should  be  used  on
satellites.  Connection  of  all  ground return  paths  to  a  single  point  helps  reduce noise  emission  and
reception  and  reduces  any  magnetic  moments  created by uncontrolled  current loops.  The  battery
negative terminal is the designated star point. The battery acts as a large capacitance which provides
the perfect location for the connection of the star ground point.
The chassis of the satellite should be connected to the power or signal ground only at the star point.
System ground is connected to the mounting holes which in turn connects to the chassis. Therefore,
the  chassis  should  not  be  directly connected to  any  PCBs  at  any  other  point  in  the  satellite.    This
prevents any ground loops or unwanted power return paths. Capacitors may be  used to shunt high
frequency components to the chassis to provide shielding. This is acceptable as no DC current can pass
through the capacitors.
On the 80Wh battery, a DC chassis ground connection is provided only on one of the two 40Wh units.

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 35 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016


Figure 8-4  Star Grounding Point at Battery Negative Terminal

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 36 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 9 CHARGE AND DISCHARGE MODES
Clyde  Space  batteries  should  always  be  charged  using  a  Clyde  Space  EPS.  The  EPS  uses  a  constant
current and constant voltage tapered charging regime. Further details about this can be found in the
Clyde Space EPS user manual.

Figure 9-1 Tapered charging method
## 9.1 Discharge
Figure 9.1 shows the profile of a full discharge of the battery at a C/5 rate.  A full discharge cycle is
carried out on all Clyde Space batteries prior to shipment to verify their capacity.  In order to maximise
the cycle life of the battery, it is recommended to discharge the battery to a maximum of 20% DoD.
## 9.2 Lot Acceptance Testing
In order to determine the cell’s suitability for space applications, Clyde Space undertakes an extensive
Lot  Acceptance  Testing  regime.   An  abbreviated  set  of  results is  detailed  in  this  section;  for  a  full
description of the lot acceptance process and results please contact Clyde Space.
9.2.1 Cell Capacity Variation with Discharge Rate and Temperature
Discharge plots are shown in Figure 9-2 for rates of C/15, C/10, C/5, C/2 and C at 40°C.  In Figure 9-3,
capacities for each discharge rate are compared for all temperatures.  Note that these measurements
were carried out per cell.  A summary of the results is shown in Table 9-1.

Discharge Rate and Measured Capacity (Ah)
## T (°C) C/15 C/10 C/5 C/2
## -20
## 1.21 1.19 1.13 0.83
## 0
## 1.31 1.29 1.29 1.2
## 20
## 1.35 1.34 1.34 1.26
## 40
## 1.34 1.38 1.34 1.3
Table 9-1 Measured capacities at different discharge rates and temperatures.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 37 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016









Figure 9-2 Cell discharge rates vs Time









Figure 9-3 Cell discharge rates vs Capacity


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 38 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 9.2.2 Vacuum Cycling
Vacuum  cycling  was  carried  out  in  a  chamber  at less  than  200mbar  pressure and  at  ambient
temperature in order to characterise the effects of low pressure on the lithium polymer cell.
Tolerance of a cell to low pressure and vacuum conditions depends on the internal arrangement of
the cell electrodes and how well they remain in contact; to make the results more representative of a
CubeSat battery under vacuum the cell under test was mechanically constrained as it would be in a
battery assembly.
A plot of cell voltage vs. time for 5 cycles is shown in Figure 9-4; the cell was discharged at a rate of
C/5.  Capacity variation with cycle number is indicated in Table 9-1.


Figure 9-4 Cell cycled at C/5 rate in a vacuum

Cycle number Cell Capacity (Ah)
## 1 1.272
## 2 1.268
## 3 1.257
## 4 1.251
## 5 1.241
Table 9-1 Cell capacity variation with vacuum cycle number
No change  in cell mass was observed following the vacuum cycling (masses measured to 2 decimal
places), and there was no evidence of any cell leakage, or any unusual behaviour in the cycling profile.
Standard capacity measurements were carried out following the vacuum cycling.  Very little difference
was seen in the capacity measured before and after vacuum cycling (1.322Ah before, 1.311Ah after).
Vacuum cycling therefore did not have any significant detrimental effect on the cell capacity.
Although the cells ‘bulge’ in a vacuum, the spiral wound arrangement of the cell, and use of polymer
electrolyte  means  that  there  is  no  separation  of  cell  components  in  a  vacuum,  and  therefore  little
effect on the cell cyclability.
## 2.5
## 2.7
## 2.9
## 3.1
## 3.3
## 3.5
## 3.7
## 3.9
## 4.1
## 4.3
## 0102030405060
Voltage (V)
## Time (hours)

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 39 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

9.2.3 EMF vs SoC
Cells were cycled at a slow rate, C/50, in order to minimise the cell internal resistance and therefore
measure the cell capacity.   This test was carried out at room temperature.
A plot of voltage vs. capacity is shown in Figure 9-5.

Figure 9-5 Discharge trace at C/50 rate at 20°C.
The  capacity  of  the  cell  discharged  at  C/50  was approximately 1.452Ah,  which  is comparable  to
capacities at higher discharge rates and indicates the consistency of internal resistance with discharge
rate for this particular brand of cell. Internal resistances have been estimated from previous figures at
the  cross-over  point  from  discharge  to  charge.   For  cells  cycled  between  C/2  up  to  C/50,  internal
resistances estimates were between 140millihoms up to 190millohms.
In Table 9-2,  the  cell  voltage  at  different  depth of  discharge  is  shown  for  discharge  rates  of  C/5
compared  with  C/50.    It  is  clear  from  the  table  that  the  voltage  remains  higher  as  the  discharge
progresses  at  C/50  rate  compared  to  C/5;  this  is  due  to  the  instantaneous  effect  of  cell  internal
resistance causing a drop at the battery output proportional to the discharge current.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 40 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016


DoD (%)
Voltage of cell
discharged at C/5
## (V)
Voltage of cell
discharged at
## C/50 (V)
## 0 4.2 4.2
## 5 4.08 4.14
## 10
## 4.03 4.07
## 15
## 3.98 4.03
## 20
## 3.93 3.98
## 25
## 3.9 3.95
## 30
## 3.86 3.92
## 35
## 3.82 3.98
## 40
## 3.79 3.95
## 45
## 3.77 3.82
## 50 3.75 3.8
## 55
## 3.73 3.79
## 60
## 3.72 3.78
## 65
## 3.71 3.77
## 70
## 3.7 3.76
## 75
## 3.69 3.74
## 80 3.67 3.72
## 85
## 3.65 3.7
## 90
## 3.61 3.67
## 95 3.52 3.61
## 100
## 3 3
Table 9-2 Voltage variation with DoD at C/5, and at C/50

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 41 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 10  GENERAL PROTECTION
The  battery  (and  wider  power  system)  has  a  number  of  inbuilt  protections  and  safety  features
designed to maintain safe operation of the EPS, battery and all subsystems supplied by the EPS buses.
## 10.1 Protection Overview

Figure 10-1 Integrated EPS and Battery Protection Architecture
A  general  overview  of  the  protection  systems  implemented  at  both  a  battery  level  and  EPS  level  is
shown  above in Figure 10-1;  combined  these  protections  are  in  place  on  all  standard  Clyde  Space
power systems, using the manned flight configuration.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 42 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

10.2 End of Charge
Once  the  EoC  voltage  has  been  reached by  the  battery,  the  BCR  changes  to  EoC  mode,  which  is  a
constant voltage charging regime. Please see Clyde Space EPS user manual for further details.

10.3 Over-Current Bus Protection (OCP)
Over-current protection is carried out on the EPS. Please see Clyde Space EPS user manual for further
details.

10.4 Battery Under-voltage Protection (UVP)
Under-voltage protection is carried out on the EPS. Please see Clyde Space EPS user manual for further
details.

## 10.5 Cell Level Protection Circuit

Figure 10-2 Cell level protection circuit schematic

Each  cell  within  a  battery  assembly  has  a  protection  circuit  fitted  at  its  terminals – this  protection
circuit  provides  four  different  protection  modes  that  inhibit  charge  or  discharge when  a battery is
exposed to conditions beyond its rated limits. The protection circuit consists of a control IC with two
back to back MOSFETs on the low-side connection.
It is the responsibility of the user to ensure that the electrical characteristics of the battery defined in
this  manual  are  not  exceeded  during  normal  operation;  the  cell  level  protection  circuit  will  only
activate when these values are exceeded. This means that consideration must be taken to ensure that
a battery is appropriately sized in relation to its solar array and payload configuration for any given
application.
Whilst each protection circuit is implemented at a cell level, the activation of a protection mode on
any  particular  cell  will  have  an  impact  on  the  entire  battery  assembly.  A  protection  mode  being
triggered in one string of a battery will cause the remaining strings in the battery to also trigger as long
as the external conditions persist. In this way the activation of multiple individual protections at a cell
level will collectively inhibit or allow charge or discharge out of the battery assembly.
The four different protection modes are summarised below.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 43 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## Protection Mode Activation Release
## Cell Overcurrent – Charge
Charge current through string
>2A for >10ms
Removal of charge via
BCRs
## Cell Overcurrent – Discharge
Discharge current through
string >2A for >10ms
Application of charge and
load current <2A
Cell Overvoltage Cell voltage >4.275V Removal of charge
## Cell Undervoltage
Cell voltage drops below
## <2.3V
Application of charge via
BCRs



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 44 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016


## 10.5.1 Cell Overcurrent – Discharge
In the event of a discharge current greater than 2A per string within the battery for a period of time
greater than 10ms, the protection circuit will activate and will inhibit further discharge to the entire
battery  assembly.  This  means  that  all  circuitry  downstream  of  the  batteries – power  buses,  EPS
undervoltage  monitoring, and I
## 2
C  nodes  will  be  completely  disabled  whilst  this  protection  mode  is
active.
Discharge  protection  will  be  disabled  once  a  charge  voltage  is  applied  to  the  battery  via  the  BCRs.
During this time the output voltage measured at the terminals of the battery will not reflect the real
battery  voltage – care  should  be  taken  to  distinguish  between  a  battery  that  has  triggered  the
overdischarge protection mode and a battery that has been physically damaged.

## 10.5.2 Cell Overcurrent – Charge
In  the  event  of a  charge  current  greater  than  2A  per  string  within  the  battery  for  a  period  of  time
greater  than  10ms,  the  protection  circuit  will  activate and  will  inhibit  further  charge  to the entire
battery  assembly.  The  only  way  to  activate  this  condition  is  to  supply  excessively  large  amounts  of
current  via  the  BCRs – it  should  be  ensured  that a  battery  is  appropriately  sized  in  relation  to  any
connected panel configuration.
Charge protection will be disabled once a load of any magnitude is applied to the battery.

10.5.3 Cell Overvoltage/Cell Undervoltage
Under  normal  conditions  for  a  battery  integrated  with  an  EPS,  the  two  cell  level  protection  modes
overvoltage and undervoltage will always be superseded by the EPS level protections for Undervoltage
and End of Charge.
The main protection provided by the overvoltage and undervoltage modes is for the case of batteries
in transit or otherwise not integrated with an EPS; for these scenarios the undervoltage and end of
charge  protection  for  the  battery  will  not  be  present  as  these  are  based  on  the  EPS.  The  cell  level
overvoltage  and  under  voltage  modes  provide  an  extra  layer  of  protection  for  an  un-integrated
battery, particularly in case of accidental over-discharge for when the battery is left unattended for
long periods of time.
Whilst cell undervoltage protection is enabled, the  battery voltage may read approximately 0V – as
with the overdischarge protection mode, care should be taken to distinguish between a battery that
has triggered the protection mode and a battery that has been physically damaged.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 45 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016


## 10.6 Over-current Polyswitch Protection
A polyswitch is fitted in line with each string of the battery.   This is a resettable fuse, designed to trip
when an over-current, either charge or discharge,  is observed by the string.  The  polyswitch should
only activate in case of all other protections within the system being bypassed.

Temperature (°C) Approximate Trip Current (A)
## -40 7.0
## -20 6.3
## 0 5.5
## 20 5.0
## 40 4.0
## 60 3.3
Table 10-1 Polyswitch Trip Current Variation with Temperature

The  approximate  fusing  currents  are  shown  in Table  10-1 above. If  the  cause  of  the  over-current
subsequently clears, the fuse will reset, allowing current to flow to and from the battery again.
Once a polyswitch has been fused and reset once the resistance is unknown – as such the efficiency
may be degraded following this event.  Hence, if a polyswitch is fused during ground testing, it should
be replaced.

## 10.7 Inhibit Operation
There are five separation switch operated inhibits built into the battery motherboard.  When pins 1
and  2  of  an  inhibit  connector  are  shorted  together  the  inhibit  is  activated  and  no  current  can  flow
through the inhibit.  When the pins 1 and 2 of the inhibit connector are open circuited the inhibit is
disabled and current can freely flow through it.
As shown in Figure 10-1, there are four high side inhibits to prevent current flowing from the battery
and solar arrays to the PCMs, as well as current from solar arrays to the battery. There is a low side
inhibit on the battery to completely isolate the battery from the  satellite. The RBF (Remove  Before
Flight)  operates  using  the  same  circuitry  as  high  side  inhibit 3. Both  the  separation  switch  and  RBF
connector pins need to be open circuited before current can flow.  Note that the arrows above inhibits
in Figure 10-1 indicate the current direction that can be blocked when the inhibit is activated.
The  80Wh  battery  comprises  two  40Wh  units,  each with  independent  inhibits. The  two  40Wh
assembly inhibit connections must be harnessed together. This will ensure that the inhibits of the two
units operate together. An example of this harnessing for high side inhibit 1 is shown in Figure 10-3.
All other inhibits, including the RBF, should use this same split harnessing.
The inhibits consist of solid state switching circuitry. The switching MOSFETs used have extremely low
drain-source  resistance  for  high  efficiency. By  using MOSFET  switches  directly  on  the  battery
motherboard  compared  to  the  conventional  approach  of  wiring the  separation  switches  to  the
mechanical switches, greater performance can be delivered.  The main current loop is much smaller
and this provides lower losses and decreases failure points.  The electromagnetic interactions between

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 46 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

the  power  system are minimised both internally to subsystems in the satellite and the earth’s
magnetic field.
Note:  When the battery is mated to an EPS, the inhibits must be enabled while the system is not in
use to prevent battery discharge.  This is particularly important for the integrated battery as the EPS
is always mated to the battery.

Figure 10-3 - Connection of high side inhibit 1 for 80Wh battery
10.8 Overvoltage protection (OVP)
In addition to the cell level overvoltage protection there is a safety mechanism for dealing with large
voltage spikes at the positive terminal of the battery. This feature protects the battery in case of back
EMF and other overvoltage events.
The excess energy will be discharged through an off-board shunt resistor that interfaces the battery
board through connector J8 (see Section 8 Interfacing). The connector is an 8 pin male header (Hirose
DF13-8P-1.25DSA(50)): pins 1-4 connect to one end of the resistor and pins 5-8 connect to the other
end.
Shunt resistor requirements:
## • 15 Ohms
- 25W rated (e.g. THS2515RJ)
- Mounted to a heat tolerant/dissipative part of the structure
Clyde Space offers a shunt resistor with an integrated harness (27-01472). Please contact Clyde Space
for further information.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 47 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 11 HEATER OPERATION
Each  battery  string  has  its  own  autonomous  heater,  designed  to  maintain  the  temperature  of  the
batteries above 0°C to maximise the capacity of the battery.
The heater is controlled by a thermostat circuit with hysteresis. When the temperature of the board
drops below 1°C the heater on each board will switch on, drawing power from the 3V3 Bus (or 5V if
configured for it).  This can be observed via the heater telemetry.  Once the temperature rises above
5°C the heater will switch off.

Figure 11-1 Operation of the Heater Control circuitry






## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 48 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 12 TELEMETRY AND TELECOMMAND
## 12.1 Communications
All communications to the Telemetry and Telecommand (TTC) node are made using an I²C interface
which  is configured  as  a  slave  and  only  responds  to  direct  commands  from a master  I²C node - no
unsolicited telemetry is transmitted. The 7-bit I
## 2
C address of the TTC node is factory set at 0x2A (10-
40Wh batteries only) and the I
## 2
C node operates at a 100kHz bus clock. For the 80Wh battery, one of
the 40Wh units operates on I
## 2
C address 0x2A and the other on 0x2D. Refer to the CoC for each unit to
determine the I
## 2
C address of each.
## 12.2 Command Protocol
Two  message  structures  are  available  to  the  master;  a  write  command  and  a  read  command.    The
write command is used to initiate an event and the read command returns the result.  All commands
start with the 7 bit slave address and are followed by the data bytes. When reading responses, all data
bytes should be read out together. Each command has a delay associated with it, this is required to
allow the microcontroller time to process each request.
For a write command the first data byte will determine the command to be initiated. The second byte
contains  the  parameters  associated  with  that  command.  For  commands  which  have  no  specific
requirement for a parameter the second data byte should be set to 0x00.
For  a  read  command,  the  first  data  byte  represents  the  most  significant  byte  of  the  result  and  the
second data byte represents the least significant byte.
Before  sending a command, the master is required to set a start condition on the I
## 2
C bus. Between
each byte the receiving device is required to acknowledge receipt of the previous byte in accordance
with the I
## 2
C protocol. This will often be accommodated within the driver hardware or software of the
## I
## 2
C master however the user should ensure that this is the case.
The read and write command definitions are illustrated in Figure 12-1.


## Write
## Command
S 7 bit node address W A Command A Data Parameter A


## Read
## Command
S 7 bit node address R A Data[1] A Data[0] N P


S Start Condition P Stop Condition   Transmitted from Master (OBC)

A Acknowledge W Write bit


N Not Acknowledged R Read bit   Transmitted from Slave (TTC node)

## Figure 12-1  I
## 2
C Write and Read of 2 byte command packet

If an error has been generated from a command then the return value will be 0xFFFF. If this value is
returned it is recommended to either inspect the status bytes or to request the code representing the
last error generated on the board as described in Section 12.4.2.





## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 49 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

12.3 List of Available Commands

## Name Command Data[1]
## 1
Data[0] Bytes Returned W/R Delay
Board Status 0x01 NA 0x00 2 < 1
Get Last Error 0x03 NA 0x00 2 < 1
Get Version 0x04 NA 0x00 2 < 1
Get Checksum 0x05 NA 0x00 2 35
## Get Telemetry 0x10 Table 12-8 2 5
Get Number of Brown-out Resets 0x31 NA 0x00 2 <1
Get Number of Auto Software Resets 0x32 NA 0x00 2 < 1
Get Number of Manual Resets 0x33 NA 0x00 2 < 1
Get Heater Controller Status 0x90 NA 0x00 2 < 1
Set Heater Controller Status 0x91 NA Mode 2 -
Manual Reset 0x80 NA 0x00 0 -


## 1
Where a command has Data[1] listed as NA, the command only requires a single data byte to be transmitted.
This is given by Data[0].

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 50 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

12.4 Housekeeping and Status Commands
## 12.4.1 Board Status (0x01)
Command Data[0] Bytes Returned Delay, ms
## 0x01 0x00 2 < 1
The status bytes are designed to supply operational data about the I
## 2
C Node. To retrieve the data that
represent the status, the command 0x01 should be sent followed by 0x00. The meaning of each bit of
the returned status  bytes is  shown below.  Please  note that  Data[1]  is  the  first  byte  returned  and
Data[0] is the last, this is shown in detail by Figure 12-1.

## Data[n] Bit Description
## 0
0 Set HIGH if last command not recognised
1 Set HIGH if a watchdog error occurred, resetting the device
2 Set HIGH if the data sent along with the last command was incorrect
3 Set HIGH if the channel passed with the last command was incorrect
4 Set HIGH if there has been an error reading the EEPROM
5 Set HIGH if a Power On Reset error occurred
6 Set HIGH if a Brown Out Reset occurred
7 Set HIGH if Heater Thermostat Circuitry is Active
## 1
## 0
## Unused ...
## 7
Table 12-2  Status bits for 3G Battery


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 51 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 12.4.2 Get Last Error (0x03)
Command Data[0] Bytes Returned Delay, ms
## 0x03 0x00 2 < 1
If an error has been generated after attempting to execute a user’s command the value 0xFFFF is
returned. To find out the details of the last error, send the command 0x03 followed by the data byte
0x00. This will return the code of the last error generated. Details of each error code are given by Table
## 12-3.

## Code Description
0x10 CRC code does not match data
0x01 Unknown command received
0x02 Supplied data incorrect when processing command
0x03 Selected channel does not exist
0x04 Selected channel is currently inactive
0x13 A reset had to occur
0x14 There was an error with the ADC acquisition
0x20 Reading from EEPROM generated an error
0x30 Generic warning about an error on the internal SPI bus (only if daughterboard is connected)
Table 12-3 List of Clyde Space Error Codes

## 12.4.3 Get Version (0x04)
Command Data[0] Bytes Returned Delay, ms
## 0x04 0x00 2 < 1
The version number of the firmware will be returned on this command. The firmware version number
is encoded in the following way:

## Data[1] Data[0]
## Bit 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
## Value Battery Firmware Rev Battery Firmware Number
## Table 12-4 Version Number Breakdown
The revision number returns the current revision of the  firmware that is present on the board. The
firmware number returns the current firmware on the board.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 52 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 12.4.4 Get Checksum (0x05)
Command Data[0] Bytes Returned Delay, ms
## 0x05 0x00 2 35
This command instructs the node to self-inspect its ROM contents in order to generate a checksum.
The value retrieved can be used to determine whether the contents of the ROM have changed during
the operation of the device.

## Data[1] Data[0]
## Bit 7 6 5 4 3 2 1 0 7 6 5 4 3 2 1 0
## Value Firmware Checksum


## 12.4.5 Manual Reset (0x80)
Command Data[0] Bytes Returned Delay, ms
## 0x80 0x00 0 -
If required the user can reset the TTC node  using this command. When issued, the board will reset
within 1 second. This command will result in the board being brought up in its defined initial condition.
Resetting the board in this fashion will increment the Manual Reset Counter. More details about this
counter are found in Section 12.7.3.



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 53 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 12.5 Telemetry
The node telemetries allow the satellite’s on board computer (OBC) to monitor the operation of the
battery.
The  telemetry  node  interfaces  to  the  various  sensing  circuits  on  the battery through  an  analogue
multiplexer.  In  response  to  I
## 2
C  telemetry  requests,  the  microcontroller  will  configure  the  analogue
multiplexer to connect the desired telemetry channel to the analogue to digital converter (ADC).  The
microcontroller will sample the desired channel and allow it to be read over the I
## 2
C bus. In response
to a telecommand the telemetry node will decode the incoming message and reset the desired power
bus.
An abridged example of the I
## 2
C node is illustrated by Figure 12-1

Figure 12-1 Abridged telemetry functional diagram
Each available telemetry is represented by a two byte code. These codes consist of:
- What type of telemetry is requested, i.e. PDM or PCM, analogue inputs, or some other form
of sensor.
- The channel being requested.
- The reading to take; whether it’s voltage, current, temperature etc.
A break-down of the telemetry structure is given in Table 12-5. The Telemetries which are available
for  this  board  are  given  in Table 12-8.  If  a  telemetry  is  requested which  is  not  available,  a  Channel
Error will be generated.

## Data[1] Data[0]
## Nibble 3 Nibble 2 Nibble 1 Nibble 0
Family Code TLM Type Code Channel Code Attribute Code
## Power
## Systems
## E Main Power 2
## Core Bus
## Miscellaneous
0 to 7
8 to F
## Voltage 0
## Current A
## Power
## Systems
E Temperature 3 Main Board 0 to 7 Temperature 8
Table 12-5 Break down of Clyde Space telemetry code structure.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 54 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 12.6 Get Telemetry (0x10)
Command Data[1] Data[0] Bytes Returned Delay, ms
0x10 0xE? 0x?? 2 5

As described above, requesting telemetry involves sending the command 0x10 plus a 2 byte telemetry
code to the node. Once transmitted, the node will configure itself to read the requested value. The
data returned will be in the format shown in Table 12-6.
## Data[1] Data[0]
## 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
0 0 0 0 0 0 ADC Result
Table 12-6  ADC result return format

The result should then be converted to physical units via the conversion equations in Table 12-8. The
equations  provided  in Table 12-8 are  the  theoretical  equations  for  the  system.  If  more  accurate
telemetry results are required, tailored equations are available from the test report for the individual
product which will be supplied with the hardware. The advantage of using tailored equations is that
they  compensate  for  component  tolerances  and  parasitic  losses  in  an  individual  build  of  a battery,
however the tailored equations will vary slightly for every battery manufactured and therefore may
be different between flight and engineering model hardware.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 55 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016




## Availability
## 1

10Whr

20Whr

30Whr

40Whr

Name TLE Code Description Uncalibrated Conversion Equation Units
## TLM_VBAT
0xE280
## Battery Output Voltage
0.008993 x ADC V

## ✓ ✓ ✓ ✓
## TLM_IBAT
0xE284
## Battery Current Magnitude
14.662757 x ADC mA

## ✓ ✓ ✓ ✓
## TLM_IDIRBAT
0xE28E
Battery Current Direction ADC < 512 Charging; Else Discharging -
## ✓ ✓ ✓ ✓

## TLM_TBRD
0xE308
Motherboard Temperature (0.372434 x ADC) -273.15 °C
## ✓ ✓ ✓ ✓

## TLM_IPCM5V
0xE214
Current Draw of 5V Bus
1.327547 x ADC mA

## ✓ ✓ ✓ ✓
## TLM_VPCM5V
0xE210
Output Voltage of 5V Bus
0.005865 x ADC V

## ✓ ✓ ✓ ✓
## TLM_IPCM3V3
0xE204
Current Draw of 3.3V Bus
1.327547 x ADC mA

## ✓ ✓ ✓ ✓
## TLM_VPCM3V3
0xE200
Output Voltage of 3.3V Bus
0.004311 x ADC V

## ✓ ✓ ✓ ✓

## TLM_TBAT1
0xE398
## Daughterboard 1 Temperature
(0.397600 x ADC) -238.57 °C

## ✓ ✓ ✓ ✓
## TLM_HBAT1
0xE39F
## Daughterboard 1 Heater Status
ADC < 512 Heater Off; Else On. -

## ✓ ✓ ✓ ✓
## TLM_TBAT2
0xE3A8
## Daughterboard 2 Temperature
(0.397600 x ADC) -238.57 °C

## - ✓ ✓ ✓
## TLM_HBAT2
0xE3AF
## Daughterboard 2 Heater Status
ADC < 512 Heater Off; Else On. -

## - ✓ ✓ ✓
## TLM_TBAT3
0xE3B8
## Daughterboard 3 Temperature
(0.397600 x ADC) -238.57 °C

## - - ✓ ✓
## TLM_HBAT3
0xE3BF
## Daughterboard 3 Heater Status
ADC < 512 Heater Off; Else On. -

## - - ✓ ✓
## TLM_TBAT4
0xE3C8
## Daughterboard 4 Temperature
(0.397600 x ADC) -238.57 °C

## - - - ✓
## TLM_HBAT4
0xE3CF
## Daughterboard 4 Heater Status
ADC < 512 Heater Off; Else On. -

## - - - ✓
Table 12-7 List of Telemetry Codes for the 3G Battery range




## 1
If  telemetry  is  requested  from  a TLE code  which  is  not  available  on your battery  model,  the  returned  data  should  be
discarded.

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 56 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

12.7 Watchdogs and Reset Counters
12.7.1 Get Number of Brown-out Resets (0x31)
Command Data[0] Bytes Returned Delay, ms
## 0x31 0x00 2 <1
This  counter  is  designed  to  keep  track  of the  number  of brown-out  resets that  have  occurred.  This
counter will roll over at 255 to 0.

12.7.2 Get Number of Automatic Software Resets (0x32)

Command Data[0] Bytes Returned Delay, ms
## 0x32 0x00 2 < 1
If the on-board microcontroller has experienced a malfunction, such as being stuck  in a loop, it will
reset  itself  into  a  pre-defined  initial  state.  Using  this  command, 0x32, it  is  possible  to  retrieve  the
number of times this reset has occurred.

12.7.3 Get Number of Manual Resets (0x33)
Command Data[0] Bytes Returned Delay, ms
## 0x33 0x00 2 < 1
A count is kept of the number of times the device has been manually reset using the Reset command.
Sending the command 0x33 with data byte 0x00 will return the number of times the device has been
reset in this fashion.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 57 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 12.8 Heater Commands
## 12.8.1 Get Heater Controller Status (0x90)
Command Data[0] Bytes Returned Delay, ms
## 0x90 0x00 2 1

Return the current status of the battery heater controller. Return codes listed in Table 12-8.

## 12.8.2 Set Heater Controller Status (0x91)
Command Data[0] Bytes Returned Delay, ms
## 0x91 Heater Code 0 1

Control  the  operation  of  the  battery  heater  circuitry.  If enabled the  battery  will  activate  its  heater
when the cells temperature drops below a predefined value. When disabled the heater will remain
off regardless of the sensed temperature. An illustration of the heater control operation is shown by
## Figure 11-1

## Code Description
## 0x00
Thermostat control circuitry disabled.
Heater will remain off, regardless of conditions.
## 0x01
Thermostat control circuitry enabled.
Heater will switch on when on-board thermostat senses it’s appropriate.

Table 12-8 Description of heater status byte


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 58 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 13 LAUNCH PROVIDER INFORMATION
When integrated into suitable platforms the Clyde  Space  3
rd
Generation CubeSat  battery family are
compatible with a wide variety of launch platforms including manned missions. If you are intending to
launch your CubeSat from a manned flight platform please refer to [RD-4] ‘TN-1404 Use of the Clyde
Space 3rd Generation CubeSat Battery on manned missions’ for further information including safety
qualification,  answers  to  common  launch  provider  questions  and  additional acceptance testing
required to meet the necessary safety conditions.


## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 59 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 15 TEST
All  batteries  are  fully  tested  prior  to  shipping, and  test  reports  are  supplied. In  order  to  verify  the
operation of the batteries please use the following instructions. In order to safely test the battery, it
should be connected to a Clyde Space EPS.
NOTE:  Any charging and discharging of the battery will reduce the battery capacity, especially large DoD (Depth
of  Discharge)  testing.    Consider  this  when  performing  any  testing  of  the  battery  mentioned  below.    Particular
care should be taken not to reduce flight model battery capacity before launch.
The following is a step by step introduction in how to connect the battery and verify its operation.
In order to test the functionality of the battery you will require:
## • EPS
- Array Input (test panel, solar array simulator or power supply with limiting resistor)
## • Oscilloscope
## • Multimeter
## • Electronic Load
## • Aardvark I
## 2
C adaptor (or other means of communicating on the I
## 2
C bus)


## Figure 15-1 Suggested Test Setup



## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 60 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 15.1 Battery Stackup
If using an 80Wh battery, it will have to be stacked up prior to test. Ensure that appropriate standoffs
are used to support the battery and ensure that the CSK header is not damaged. Dimensions are shown
in section 7.4.5.
## 15.2 Solar Array Input
Testing should be performed with an EPS. See the appropriate user manual for this product.
15.3 Power Up/Down Procedure
The order of assembly should follow:
- Connect battery to stack
- Connect electronic load to battery bus
- Activate  separation  and  RBF  switches  to  link  the  connections  between  BCR_OUT,  BAT_POS
and PCM_IN to allow current to flow.
- Connect array input
When powering down, this process should be followed in reverse.
15.4 Configuration and Testing
The    following    section    outlines    the    procedure    for    performing    basic    functional    testing.
An EPS is required to safely test the battery ensuring the EPS protects the battery from being damaged.
Therefore  some  of  the  following  sections  mention  functions  within  the  EPS  necessary  for  battery
testing.
## 15.4.1 Battery Discharge
Ensure that the separation and RBF switches are not activated. All buses will be activated and can be
measured  with  a multimeter. By  increasing  the  load on  the  battery  bus you will  be  able  to  see  the
battery voltage decrease and battery current show discharge status.
## 15.4.2 Undervoltage Protection
All testing should be performed in conjunction with a Clyde Space EPS. Please refer to the EPS user
manual for further details on performing undervoltage protection testing.
15.4.3 BCR Testing
All testing should be performed in conjunction with a Clyde Space EPS. Please refer to the EPS user
manual for further details on performing BCR testing.
15.4.4 EoC Operation
All testing should be performed in conjunction with a Clyde Space EPS. Please refer to the EPS user
manual for further details on performing End of Charge testing.
## 15.4.5 Telemetry Testing
The 3V3 and 5V bus voltage and current telemetry can be queried using the I
## 2
C interfacing shown in
the  telemetry   section  of  this  user  manual. This  provides information  on  the  current  power
consumption of the battery I
## 2
C node.
While  charging  and  discharging,  the  current  magnitude  and  direction  telemetry  can  be  read.    The
voltage of the battery can also be read.
If the battery is used in a thermal chamber to drop the temperature, the heater telemetries can be
read to monitor the operation of the heaters for each board in the battery stack.  The temperature of
each board can also be read.

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 61 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016


## 16 COMPATIBLE SYSTEMS
Battery compatibility must take into account the BCR capability provided by the EPS as well as the peak output
power provided by the connected solar arrays.

Stacking Connector EPS Arrays Notes
Standalone 10Whr (01
## -
## 0
## 2683
## )

CubeSat Kit Bus
## 25-02451 –
## 3G EPS (1UB)
## No Inhibits
Clyde Space -2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 2A
N/A - Clyde Space 4-8 Cell solar array
Other array technologies
## 1

## 25-02452 –
## 3G EPS (3UA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 2A
Clyde Space 4-8 Cell solar array
Other array technologies
## 01-02453 –
## 3G EPS (XUA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 2A
Clyde Space 4-8 Cell solar array
Other array technologies
## 6

Standalone 20Whr (01
## -
## 02684)

CubeSat Kit Bus
## 25-02451 –
## 3G EPS (1UB)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 4A
N/A - Clyde Space 4-8 Cell solar array
Other array technologies
## 25-02452 –
## 3G EPS (3UA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 4A
Clyde Space 4-8 Cell solar array
Other array technologies
## 01-02453 –
## 3G EPS (XUA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 4A
Clyde Space 4-8 Cell solar array
Other array technologies
## 6

Standalone 30Whr (01
## -
## 02685)

CubeSat Kit Bus
## 25-02451 –
## 3G EPS (1UB)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 6A
N/A - Clyde Space 4-8 Cell solar array
Other array technologies
## 25-02452 –
## 3G EPS (3UA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 6A
Clyde Space 4-8 Cell solar array
Other array technologies
## 01-02453 –
## 3G EPS (XUA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 6A
Clyde Space 4-8 Cell solar array
Other array technologies
## 6

Standalone 40Whr (01
## -
## 02686
## )

CubeSat Kit Bus
## 25-02451 –
## 3G EPS (1UB)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 8A
N/A - Clyde Space 4-8 Cell solar array
Other array technologies
## 25-02452 –
## 3G EPS (3UA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 8A
Clyde Space 4-8 Cell solar array
Other array technologies
Clyde Space 2-3 Cell solar array
Clyde Space 4-8 Cell solar array

## 1
Any array technologies that conform to the BCR input ratings for Voltage and Current. Consult EPS user manual or contact
Clyde Space for more details.

## USM-1192

User Manual: 3rd Generation CubeSat Battery Family

Issue: E Date: 26/06/2018 Page: 62 of 62
## Skypark 5, 45 Finnieston Street
Glasgow, G3 8JU, United Kingdom

Space is Awesome www.clyde.space
PROPRIETARY & CONFIDENTIAL INFORMATION © Clyde Space Limited 2016

## 01-02453 –
## 3G EPS (XUA)
## No Inhibits
Other array technologies
## 6

Peak battery charge current
following BCR stage not to
exceed 8A
Standalone 80Whr (01
## -
## 02687
## )

CubeSat Kit Bus
## 25-02451 –
## 3G EPS (1UB)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 16A
N/A - Clyde Space 4-8 Cell solar array
Other array technologies
## 25-02452 –
## 3G EPS (3UA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 16A
Clyde Space 4-8 Cell solar array
Other array technologies
## 01-02453 –
## 3G EPS (XUA)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 16A
Clyde Space 4-8 Cell solar array
Other array technologies
## 6



Stacking Connector EPS Arrays Notes
## Integrated
10Whr (01
## -
## 02681
## )

N/A – Stacking
connector present
on 1U/3U EPS
## Motherboard
## 25-02451 –
## 3G EPS (1UB)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 2A
N/A - Clyde Space 4-8 Cell solar array
Other array technologies
## Integrated
20Whr (01
## -
## 02682
## )

N/A – Stacking
connector present
on 1U/3U EPS
## Motherboard
## 25-02451 –
## 3G EPS (1UB)
## No Inhibits
Clyde Space 2-3 Cell solar array
Peak battery charge current
following BCR stage not to
exceed 4A
N/A - Clyde Space 4-8 Cell solar array
Other array technologies
