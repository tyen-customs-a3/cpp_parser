class 24Rnd_missiles;
class RHS_AH1Z_base: Heli_Attack_01_base_F
class RHS_AN2_Base: Plane_Base_F
maxAngleX=+30;
y = QUOTE(((((BUTTON_Y - 0.5)*SCALE)+0.5) * safeZoneH) + (((BUTTON_OFFY*safeZoneH* SCALE) + (BUTTON_H*safeZoneH* SCALE)) * BY) + safeZoneY)
y = QUOTE(((((BUTTON_Y - 0.5)*SCALE)+0.5) * safeZoneH) + (((BUTTON_OFFY*safeZoneH* SCALE) + (BUTTON_H*safeZoneH* SCALE)) * BY) + safeZoneY);\
y = QUOTE(((BUTTON_Y) * safeZoneW) + (((BUTTON_OFFY*safeZoneW) + (BUTTON_H*safeZoneW)) * BY) + safeZoneX)
_data = call compile preprocessFileLineNumbers "Littlebird.sqf";
_flightData = call compile preprocessFile "flight.hpp";
_unit setVariable [QGVAR(selectedWeaponOnRearm), currentWeapon _unit]; \
!alive _unit || \
"A3\Characters_F\Common\Data\basicbody.rvmat", \
"a3\sounds_f\characters\ingame\AinvPknlMstpSlayWpstDnon_medic.wss",\
"amovppnemstpsnonwnondnon_amovppnemevasnonwnondl",\
"AnimChanged", \
"BIS_Fades" cutText ["", "BLACK FADED", 10];
"BIS_objBase" call BIS_fnc_missionTasks;
"BIS_objDisarm" call BIS_fnc_missionTasks;
"BIS_objGetIn" call BIS_fnc_missionTasks;
"BIS_objSteal" call BIS_fnc_missionTasks;
"BIS_tskBeach" call BIS_fnc_missionTasks;
"BIS_tskEquipment" call BIS_fnc_missionTasks;
"BIS_tskFOB" call BIS_fnc_missionTasks;
"BIS_tskGetIn" call BIS_fnc_missionTasks;
"BIS_tskVilla" call BIS_fnc_missionTasks;
"concrete", "concrete_exp", "concrete_int", "int_concrete", "int_concrete_exp", \
"Land_Bench_01_F", "Land_Bench_02_F", "Land_Bench_03_F", "Land_Bench_04_F", "Land_Bench_05_F", \
"net_fence_4m_f.p3d",\
"x\tmf\addons\orbat\textures\yellow_inf.paa", \
(uiNamespace getVariable QGVAR(clientTemp))  getVariable [setting, [nil, [setting,  "client"] call FUNC(priority)]] select 1,\
[
["Init"] call BIS_fnc_InstructorFigure;
["pca\addons\suture\functions\fnc_sutureStart.sqf", "pca_suture_fnc_sutureStart"] call CBA_fnc_compileFunction;
[] spawn
[] spawn {
[1, 1, 0, 0.5], \
[65, DIK_A],\
[QPATHTOF(functions\DOUBLES(fnc,fncNameIntercept).sqf), QFUNC(fncName)] call CBA_fnc_compileFunction \
{
{ \
{_x allowFleeing 0} forEach allUnits;
{_x disableAI "MOVE"} forEach (units group BIS_inf + units BIS_IA_group1 - [BIS_inf]);
{_x setMarkerAlpha 0} forEach ["BIS_mrkArtillery", "BIS_Arrow01", "BIS_Arrow02", "BIS_Arrow03"];
{_x setMarkerAlpha 0} forEach ["BIS_spotters_marker1", "BIS_spotters_area"];
{(000/2000), 0},
{(400/2000), 0}, // ignore idle
{(700/2640), 0},
{\
{bone,{0,__EVAL(-scale/XtoYscale)},1},{bone,{__EVAL(+0.7*scale),__EVAL(-0.7*scale/XtoYscale)},1}, \
{boneCenter,{0,	__EVAL(-1.0000 * scale / XtoYscale)}, 1}, \
}
};
/*
/* BASE SOUNDSET */ \
/* Hind */ \
/* RATTLING */ \
// - - - US ARMY AI weapons - - -
// Control SF Teams
// Do not allow units to flee
// FAB-100 & variants
// legacy classes
///! mines & grenades
//wait for the time to get set before the briefing code is executed
#include "defines.inc"
0 fadeSound 0;
0 setFog [0.25, 0.125, 5];
3.5*0.79432821,
accuracy = 3.9;
ACE_OverlayCracked = QPATHTOEF(goggles,textures\HUD\CombatGogglesCracked.paa); \
aiAmmoUsageFlags	= AIAmmoUsageOffensiveVeh + AIAmmoUsageOffensiveArmour + AIAmmoUsageOffensiveAir;
aiAmmoUsageFlags = AIAmmoUsageOffensiveInf + AIAmmoUsageOffensiveVeh + AIAmmoUsageOffensiveArmour;
aiRateOfFire = 1;\
alpha = 0.12;
angle1 = 2 * 3.1415;
armor = 5*0;				/// what protection does the vest provide
author = "$STR_A3_Bohemia_Interactive";\
author = $CUP_AUTHOR_STRING;
author = $STR_A3_Bohemia_Interactive;
author = $STR_CUP_AUTHOR_STRING;
AUTHOR_MACRO
author="RHS (A2 port)";\
author="RHS (TOH port)";\
author=$STR_RHSUSF_AUTHOR_FULL;
BEGIN_CONTROL(ChannelKnob, BF888S_RscPicture, 106)
BEGIN_CONTROL(ChannelKnob, Prc343_RscPicture, 106)
BEGIN_CONTROL(DialBackground, Prc77_RscPicture, 201)
BEGIN_CONTROL(DisplayBackground, Prc117f_RscBackground, 99990)
BEGIN_CONTROL(RadioBackground, SEM52SL_RscPicture, 300)
BEGIN_CONTROL(RadioBackground, SEM70_RscPicture, 300)
BEGIN_CONTROL(Vic3ffcsBackground, VIC3FFCS_RscPicture, 100)
BIS_boatFlee = compile preprocessFile "scenes\boatFlee.sqf";
BIS_crashSite_attackers = (units BIS_IA_group2 + units BIS_IA_group3);
BIS_inf switchmove "amovpercmstpslowwrfldnon";
BIS_insertion = compile preprocessFile "scenes\insertion.sqf";
BIS_leaveRoad = compile preprocessFileLineNumbers "leaveRoad.sqf";
BIS_player createDiaryRecord 
BIS_player createDiaryRecord [
BIS_scriptNull		= [] spawn {};
BIS_survivors = [BIS_exp, BIS_survivor1, BIS_survivor2, BIS_survivor3];
BIS_UGVMove = compile preprocessFile "scenes\UGVMove.sqf";
borderLeft				= 0;
briefingName = $STR_A3_CfgMissions_MPMissions_MP_COOP_m04;
bullet1[] = {"A3\sounds_f\weapons\shells\7_62\metal_762_01", 0.630957, 1, 15};\
class ##classname## { \
class ##classname##_##numb## { \
class ##obj_name##_trans_##frame_no## \
class a29_mainLoop {SCRIPT_PATH(rhs_a29_mainLoop)};
class ACE_Actions { \
class ACE_Actions {\
class ace_explosion_reflection_##range##_##hit: ace_explosion_reflection_base {\
class ACE_MainActions { \
class ACE_SelfActions { \
class ACEGVAR(zeus,moduleName): ACEGVAR(zeus,moduleBase) { \
class ACRE_VRC110 : ACRE_BaseRack {
class Attenuation { \
class Attributes \
class bomb_04_f;
class button##ID: button {\
class button##ID: listButton {\
class CA_Distance; \
class CA_VehicleTogglesBackground: CA_VehicleTogglesBackground\
class CannonCore;
class CfgAddons { \
class CfgIdentities
class child : par\
class classname##_max : classname\
class CUP_B_USMC_Navy_##color : ItemCore { \
class CUP_C_TK_Man_##num : CUP_Creatures_Civil_Takistan_Base { \
class CUP_H_Navy_CrewHelmet_##color : ItemCore \
class CUP_H_TKI_SkullCap_##num : ItemCore \
class CUP_I_TKG_Man_##num : CUP_Creatures_Military_TKG_Guerrilla_Base { \
class CUP_O_TKI_Khet_Partug_##num : ItemCore \
class CUP_O_TKI_Man_##num : CUP_Creatures_Military_TKI_Insurgent_Base { \
class Disabled { \
class DOGTAGBASE##_##DOGTAGID: DOGTAGBASE { \
class DOUBLES(squawkName,10) { \
class EGVAR(CSW,assemblyMode) { \
class Exhausts \
class Exhausts\
class Extended_PostInit_EventHandlers {                 \
class Flashlight { \
class GVAR(DOUBLES(scream,no)) { \
class GVAR(x): EGVAR(modules,moduleBase) {\
class HitLeftArm: HitHands {\
class HitRotor##num##: HitRotor1\
class HMG_127;
class Item_##nameN## : Item_Base_F {\
class Level2P##XX##: Level00 \
class Level2PH##XX: Level00 \
class LevelP##XX: Level0 															\
class LevelP##XX: Level0 \
class LinkedItems
class listButton##ID: listButton {\
class m113_commander_turret 				{file = ADDON_PATH\rhs_m113_commander_turret.sqf;};
class m11xx_init { file = ADDON_PATH\rhs_m11xx_init.sqf; };
class MFD
class Mode_##numb##: Mode_base\
class MP_Marksmen_01
class player: player { S_Macro };
class player: player\
class Pointer { \
class PREFIX##_##TAILNAME##_Meadows_close_SoundShader { \
class PREFIX##_##WPNNAME##_shot_close_soundShader { \
class PREFIX##_##WPNNAME##_shot_fp_soundShader { \
class PREFIX##_##WPNNAME##_shot_inside_soundShader { \
class PREFIX##_Heli_##HELINAME##_ext_close_base_SoundShader { \
class PREFIX##_Heli_Ext_Base_SoundSet { \
class PREFIX##_Plane_##PLANENAME##_ext_close_low_SoundShader { \
class R_80mm_HE;
class RHS_##cargo##_V##version: RHS_##cargo## \
class RHS_##TAILNAME##_Meadows_close_SoundShader { \
class RHS_##TYPENAME##_Meadows_0m_SoundShader { \
class RHS_##WPNNAME##_Closure_SoundShader { \
class RHS_##WPNNAME##_punch_close_soundShader { \
class RHS_Composite_##num\
class RHS_veh_##TAILNAME##_Meadows_close_SoundShader { \
class RHS_veh_##WPNNAME##_punch_close_soundShader { \
class rhs_weap_##BOMB_NAME##: rhs_weap_##INHERIT	\
class RHSGREF_##TAILNAME##_Meadows_close_SoundShader { \
class RHSGREF_##WPNNAME##_punch_close_soundShader { \
class RHSSAF_##TAILNAME##_Meadows_close_SoundShader { \
class RHSSAF_##WPNNAME##_punch_close_soundShader { \
class RHSUSF_##TAILNAME##_Meadows_close_SoundShader { \
class RHSUSF_##TYPENAME##_Meadows_0m_SoundShader { \
class RHSUSF_##WPNNAME##_Closure_SoundShader { \
class RHSUSF_##WPNNAME##_punch_close_soundShader { \
class RHSUSF_veh_##TAILNAME##_Meadows_close_SoundShader { \
class RHSUSF_veh_##WPNNAME##_punch_close_soundShader { \
class SmokeShell: SmokeShell { \
class StandardSound\
class stryker_turnActions 				{file = ADDON_PATH\rhs_stryker_turnActions.sqf;};
class TransportBackpacks									\
class TransportBackpacks\
class TransportMagazines									\
class TransportWeapons										\
class WP##num##_PosX: MovementX 				\
CLASS_DEF(VEH_NAME,MOD_TAG,CAMPOS,RPM0)
CLASS_DEF(VEH_NAME,MOD_TAG,CAMPOS)
color[] = {1*50,0,0.025*50,1};
cost						= 50;
cost = 200000;
createCenter EAST;
crew = "rhs_vdv_armoredcrew";\
crew = "rhs_vdv_crew";\
crew = "rhs_vdv_driver_armored";\
crew = "rhs_vdv_driver";\
crew="rhsusf_army_ucp_crewman";\
CSTRING(Disabled), CSTRING(FirstPerson), CSTRING(ThirdPerson)
CUP_HEADER_PROTECTED; \
CUP_HEADER_PUBLIC; \
CUP_HEADER_PUBLIC; scopeCurator = 2; \
damperSize = .1;
db - 10,
defaultValue = 'default';
delete EGVAR(compat_sog,digSpiderholeDual);
delete GVAR(Gear);
deleted = QUOTE(if (isServer) then {_this call FUNC(deleteCamouflage)}); \
description = "Find player controled unit";
description = "Spalling (WIP)";
descriptionShort = $STR_RHS_CFGWEAPONS_RIFLE_AK74M1;
discreteDistance[]={25,100,200,300,400,500,600};\
discreteDistanceInitIndex = 0; \
displayName			= $STR_RHS_CFGMAGAZINES_20RND_9X19_MAG_7N21;
displayName			= $STR_RHSUSF_MAG_SMOKEGEN;
displayName	= $STR_RHSUSF_WEAP_SMOKEGEN;
displayName 	= $STR_b_soldier_repair_f0;
displayName  = $STR_CUP_dn_RUS_H_TSh4_Brown; // "TSh-4 (Brown)";
displayName = "$str_a3_cfgweapons_abr_base_f_weaponslotsinfo_muzzleslot0";
displayName = "$STR_RHS_INF_RIFLEMAN";
displayName = "Eagle A-III Coyote (Medic)";
displayName = "Flight recorder";
displayname = "M-21OF High Explosive";
displayName = "Operator"; // TODO: Localize
displayName = "Standard";
displayName = $STR_A3_cfggroups_empty_guerrilla_camps;						//"Camps"
displayName = $STR_a3_cfgvehicles_b_soldier_f0;
displayName = $STR_A3_CfgVehicles_B_Soldier_F0;
displayName = $STR_A3_CfgVehicles_b_survivor_f_displayName;
displayName = $STR_A3_FIRING_DRILLS_COF_PURPLE01_NAME;
displayName = $STR_A3_moduleMPTypeSeize;
displayName = $STR_a3_mp_coop_04_cfgrespawninventory_light;
displayName = $STR_A3_Role_Assault;
displayName = $STR_A3_TIME_TRIALS_SP_TT06_NAME;
displayName = $STR_B_officer_F0;
displayName = $STR_B_recon_F0;
displayName = $STR_b_soldier_ar_f0;
displayName = $STR_B_Soldier_TL_F0;
displayName = $STR_CUP_dn_RUS_EMR_1;
displayName = $STR_CUP_dn_US_BDUv2_##VARIANT##;\
displayName = $STR_HEADGEAR_RHSSAF_HELMET_M59_85_NOCAMO_DISPLAYNAME;
displayName = $STR_ITEM_RHSSAF_ZRAK_RD7J_DISPLAYNAME;
displayName = $STR_RHS_ACC_PSO1M2;
displayName = $STR_RHS_btr60_Name;
displayName = $STR_RHS_DN_KORD;
displayname = $STR_RHS_WEAP_AG36_BASE_DISPLAYNAME;
displayName = $STR_RHS_WEAP_CZ99_DISPLAYNAME;
displayname = $STR_RHSSAF_ARMY_D30_DISPLAYNAME;
displayName = $STR_RHSSAF_FAC_ARMY;
displayname = $STR_RHSSAF_KITBAG_MD2CAMO_DISPLAYNAME;
displayName = $STR_RHSSAF_SOLDIER_ARMY_DIGITAL_BASE_DISPLAYNAME;
displayName = $STR_RHSSAF_SOLDIER_ARMY_OAKLEAF_BASE_DISPLAYNAME;
displayName = $STR_RHSSAF_SOLDIER_ARMY_PARA_DIGITAL_BASE_DISPLAYNAME;
displayName = $STR_RHSSAF_SOLDIER_BASE_DISPLAYNAME;
displayName = $STR_RHSSAF_SOLDIER_UN_DESERT_BASE_DISPLAYNAME;
displayName = $STR_RHSSAF_SOLDIER_UN_DIGITAL_BASE_DISPLAYNAME;
displayName = $STR_RHSSAF_UNIFORM_M10_DIGITAL_DISPLAYNAME;
displayName = CSTRING(name); \
displayName=$STR_CUP_dn_Core_UsrA_Long_lights;
displayNameDefault	= <img image='\rhsafrf\addons\rhs_main\data\actions\rhs_tow_detach_ca.paa' size='3.5' />;
dlc = "RHS_AFRF";\
dlc = "RHS_GREF";
dlc = "RHS_USAF";
DLC_AUTHOR;
DOUBLES(ItemType,TYPE_FIRST_AID_KIT)[] = {
draconicTorqueYCoef[] = {3.2,2.5,1,,0,0,0,0,0,0,0,0,0,0,0};
driverCanSee				= 2+4+8+16;
drivercansee = 2+4+16;
drySound[]				= {"A3\sounds_f\weapons\Other\dry_1",0.01,1};
editorSubcategory = "rhs_EdSubcat_infantry_emr_des";\
editorSubcategory = "rhs_EdSubcat_infantry_emr";\
editorSubcategory = "rhs_EdSubcat_infantry_flora";\
editorSubcategory = "rhs_EdSubcat_infantry_mflora";\
editorSubcategory = "rhs_EdSubcat_infantry_ocp_arb";\
editorSubcategory = "rhs_EdSubcat_infantry_ocp";\
editorSubcategory = "rhs_EdSubcat_infantry_recon";\
editorSubcategory = "rhs_EdSubcat_infantry_ucp";\
editorSubcategory = "rhs_EdSubcat_infantry";\
EdPrev(rhsusf_airforce_jetpilot)	scope = 2;
EGVAR(arsenal,hide) = -1;
EGVAR(arsenalStats,frequencyMin) = 2.4e9;
EGVAR(arsenalStats,frequencyMin) = 30e6;
EGVAR(arsenalStats,frequencyMin) = 400;
EGVAR(arsenalStats,frequencyMin) = 46e3;
EGVAR(cargo,hasCargo) = 0;
EGVAR(cargo,hasCargo) = 1;
EGVAR(cargo,size) = 1;
EGVAR(csw,kordTripod) = "rhsgref_ins_DSHKM";
EGVAR(csw,m3Tripod) = "vn_b_army_static_m2_high";
EGVAR(csw,mortarBaseplate) = "rhs_2b14_82mm_msv";
EGVAR(csw,mortarBaseplate) = "RHS_M252_WD";
EGVAR(csw,mortarBaseplate) = "SPE_M1_81";
EGVAR(dragging,canCarry) = 0;
EGVAR(dragging,canDrag) = 1;
EGVAR(explosives,defuseObjectPosition)[] = {-0.005, 0.025, 0.06};
EGVAR(explosives,defuseObjectPosition)[] = {-0.034, 0, 0.202};
EGVAR(explosives,defuseObjectPosition)[] = {0, 0, 0.15};
EGVAR(explosives,defuseObjectPosition)[] = {0.055, 0, 0.038};
EGVAR(explosives,delayTime) = 1;
EGVAR(explosives,DelayTime) = 1;
EGVAR(explosives,Placeable) = 1;
EGVAR(explosives,SetupObject) = "ACE_Explosives_Place_US85_ATMine_mag";
EGVAR(explosives,SetupObject) = QEGVAR(explosives,Place_rhs_mine_tm62m);
EGVAR(explosives,SetupObject) = QEGVAR(explosives,Place_rhssaf_mine_mrud_a);
EGVAR(fastroping,enabled) = 0;
EGVAR(fastroping,enabled) = 2;
EGVAR(frag,enabled) = 0;
EGVAR(frag,enabled) = 1;
EGVAR(frag,skip) = 1;
EGVAR(interaction,replaceTerrainObject) = 1;
EGVAR(javelin,enabled) = 1;
EGVAR(javelin,enabled) = 1; // needs to be explicitly enabled
EGVAR(laser,canSelect) = 1; // can ace_laser lock (allows switching laser code)
EGVAR(logistics_wirecutter,isFence) = 1;
EGVAR(missileGuidance,attackProfile) = "JAV_DIR";
EGVAR(missileGuidance,attackProfile) = QGVAR(directAttack);
EGVAR(nightvision,border) = QPATHTOEF(nightvision,data\nvg_mask_4096.paa);
EGVAR(overheating,allowSwapBarrel) = 1;
EGVAR(overheating,closedBolt) = 0;
EGVAR(overheating,dispersion) = 0.75;
EGVAR(overheating,jamTypesAllowed)[] = {"Fire", "Dud"};
EGVAR(overheating,mrbs) = 3000;
EGVAR(overpressure,angle) = 30;
EGVAR(overpressure,angle) = 90;
EGVAR(overpressure,damage) = 0;
EGVAR(overpressure,priority) = 99;
EGVAR(rearm,caliber) = 178;
EGVAR(rearm,caliber) = 70;
EGVAR(refuel,canReceive) = 0;
EGVAR(refuel,canReceive) = 1;
EGVAR(refuel,fuelCapacity) = 1200;EGVAR(refuel,fuelCapacity) = 1447;
EGVAR(refuel,fuelCapacity) = 189;
EGVAR(refuel,fuelCapacity) = 2773;
EGVAR(refuel,fuelCapacity) = 530;
EGVAR(refuel,fuelCapacity) = 7548;
EGVAR(refuel,fuelCapacity) = 916;
EGVAR(refuel,hooks)[] = {{-0.23,-2.58,-0.59}};
EGVAR(refuel,hooks)[] = {{0, 0.4, -0.5}, {0, -0.4, -0.5}};
EGVAR(reloadlaunchers,enabled) = 1;
EGVAR(trenches,diggingDuration) = QEGVAR(trenches,smallEnvelopeDigDuration);
EGVAR(trenches,entrenchingTool) = 1;
EGVAR(vehicle_damage,incendiary) = 0;
EGVAR(vehicle_damage,incendiary) = 0.2;
EGVAR(vehicle_damage,incendiary) = 0.5;
EGVAR(vehicle_damage,incendiary) = 1.0;
EGVAR(vehicle_damage,incendiary) = 1.0; // Raufoss Mk 211 HEIAP (high-explosive, incendiary, armor-piercing)
enableRadio false;
enableSentences FALSE;
enum {StabilizedInAxesNone,StabilizedInAxisX,StabilizedInAxisY,StabilizedInAxesBoth, StabilizedInAxesXYZ};
enum {StabilizedInAxesNone,StabilizedInAxisX,StabilizedInAxisY,StabilizedInAxesBoth,StabilizedInAxesXYZ};
execVM "functions.sqf";
EXGVAR(field_rations,consumeAnims)[] = { \
faction = "rhs_faction_vvs";
faction = rhsgref_faction_cdf_ground_b; \
faction = rhsgref_faction_cdf_ng_b; \
faction = rhsgref_faction_chdkz_g; \
faction = rhsgref_faction_tla_g; \
faction = rhssaf_faction_airforce_opfor; \
faction = rhssaf_faction_army_opfor; \
file			= ANIM_PATH\rhs_BMKT_driver_in_kia.rtm;
file			= ANIM_PATH\rhs_m113_commander_in_kia.rtm;
file			= ANIM_PATH\rhs_m1a1_loader_in_kia.rtm;
file			= ANIM_PATH\stryker_m1126_driver_in_kia.rtm;
file = 	\CUP\WheeledVehicles\CUP_WheeledVehicles_BTR40\Data\Anim\BTR40_Cargo_V1.rtm;
file = \rhsafrf\addons\rhs_a2port_armor\anim\KIA_##name##.rtm;\
file = \rhsafrf\addons\rhs_bmp3\anim\KIA_##name##.rtm;\
file = \rhsusf\addons\rhsusf_a2port_armor\data\anim\M2A2_Commander.rtm;
file = ADDON_PATH\rhs_rhino_system.sqf;
file = AnimKa52\KIA_Ka52_Pilot.rtm;
file = AnimP\rhs_pakfa_pilot.rtm;
file = AnimP\SU25Pilot.rtm;
file = QUOTE(DOUBLES(fnc,func).sqf);\
file = rhsusf\addons\rhsusf_c_ch53\anims\rhs_CH53_Pilot.rtm;
file=\CUP\AirVehicles\CUP_Airvehicles_KA52\DATA\anim\kia_ka52_pilot.rtm;
file=\CUP\WheeledVehicles\CUP_WheeledVehicles_mtvr\data\anim\kia_mtvr_Driver.rtm;
file=\CUP\WheeledVehicles\CUP_WheeledVehicles_Tractor\data\anim\kia_truck_Driver.rtm;
file=\CUP\WheeledVehicles\CUP_WheeledVehicles_v3s\data\anim\V3S_KIA_Driver.rtm;
file=\rhsafrf\addons\rhs_heavyweapons\mg\bis_kord\anim\sittingGunner.rtm;
file=AnimPath\KIA_Mig29_Pilot.rtm;
file=rhsusf\addons\rhsusf_heavyweapons\Anim\LowTripod_Gunner.rtm;
fired = "call cba_xeh_fnc_fired"; \
forceInGarage			= 0;
format ['%1: A=%2', MESSAGE, RETNIL(A)]
frequency	=	0.9	+	HZ_IDLE_Ural*0.2;
frequency	=	0.9	+	HZ_IDLE*0.2;
frequency = 0.95	+	HZ_IDLE*0.15;
GROUNDHOLDER_MACRO(rhsgref_uniform_ttsko_forest,"M88 TTsKO (Forest)",ItemsHeadgear,EdCat_Equipment,EdSubcat_Uniforms)
GROUNDHOLDER_PISTOL_MACRO(rhs_weap_M320,rhs_mag_M441_HE,"$STR_RHS_CFGWEAPONS_RIFLE_M320",EdSubcat_Pistols)
GROUNDHOLDER_WEAPON_MACRO(rhs_weap_ak74m,rhs_30Rnd_545x39_7N10_AK,"$STR_RHS_CFGWEAPONS_RIFLE_AK74M0",EdSubcat_AssaultRifles)
GROUNDHOLDER_WEAPON_MACRO(rhs_weap_kar98k,rhsgref_5Rnd_792x57_kar98k,"$STR_RHS_WEAP_KAR98K_DISPLAYNAME",EdSubcat_SniperRifles)
gunnerCanSee = 2+4+8+16;
GVAR(BodyDay) = "*"; \
GVAR(failureDelay) = 2;
h = (16.75+0.45) * SIZE_M * GRID_H;
h = (9+0.2) * SIZE_M * GRID_H;
h = 0.308 * safezoneH;
h = 5 * SIZE_M * GRID_H;
h = POS_H(1) - pixelH;
h = safeZoneH - POS_EDGE(9.3,8.3) * GUI_GRID_H;
head = "headDefault"; \
HELISOUNDSETS_DEFAULT(hind,PREFIX)
HELISOUNDSHADERS_DEFAULT(hind,2500,15000,FILEPATH,PREFIX)
helmetPosition[] = {-HMD_SIZE, __EVAL(HMD_SIZE/2), 0.1};
hiddenSelections[] =                                                                              \
hiddenSelections[] = {"Camo1", "Camo2", "camo3", "flag", "clan", "insignia"};\
hiddenSelectionsTextures[] =\
hit = 480;
HIT_GLASS(1,0.37,-5,2.0,0.1)
HP_PYLON(1,0.7)
HUD_TEXT_BONE(Center,-0.0, -0.015,0.03,	0.035,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_BONE(Center,0.0, -0.015,0.03,	0.035,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_SOURCE_INDEX(0.78,0.42,0.31,0.55,left,ammo,1,ammoCounter)
HUD_TEXT_SOURCE(-0.005, 0.06,0.045,	0.045,center,pylonAmmo,PylonValue1)
HUD_TEXT_STATIC(-0.005, 0.02,0.05,	0.045,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(-0.005, 0.06,0.045,	0.045,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(0.00,0.02+0.00,0.025,	0.02,right,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(0.005, 0.02,0.045,	0.085,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(0.01,0.510,FONT_SIZE,FONT_SIZE,right,"LRF:",Range_Text)
HUD_TEXT_STATIC(0.02,	+0.01,0.03,	0.025,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(0.03,	+0.05,0.03,	0.025,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(0.03,0.85,0.1,0.12,right,"GOTO WP",GoToWP_Text)
HUD_TEXT_STATIC(0.03+MFD_OFFSET_X,	-0.01,0.015,	0.015,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(0.03+MFD_OFFSET_X,	+0.02,0.025,	0.025,center,MFD_PYLON_NAME,PylonText1)
HUD_TEXT_STATIC(0.04,0.13,0.07,0.07,right,"UHF-1",UHF1_Text)
HUD_TEXT_STATIC(0.04,0.15,0.06,0.06,right,"ВПУ",AmmoGunText)
HUD_TEXT_STATIC(0.1,-0.25,FONT_SIZE,FONT_SIZE_Y,right,"R",Gear_Text1)
HUD_TEXT_STATIC(0.2,0.010,FONT_SIZE,FONT_SIZE,right,"MANUAL",ManualEngage_1)
HUD_TEXT_STATIC(0.74,0.1,0.045,0.045,center,"КСБ 08.125.000.000",RadioFQ_1)
HUD_TEXT_STATIC(0.875,0.82,0.035,0.14,center,"К",AmmoText)
iconScale = 1.0;\
idc = 12##var1;\
IDC_CENTER, IDC_CROSSHAIR,\
IDC_RSCDISPLAYCURATOR_CREATE_UNITS_WEST, \
IDC_RSCDISPLAYCURATOR_MODEUNITS, \
identityTypes[] = {}
identityTypes[]=
if !(isNil "_x") then {\
if !(isNil "BIS_SkirmishVehX") then
if (!([] call BIS_fnc_isCampaign)) then {
if (!(CONDITION)) exitWith {};
if (!Bis_DebugMode) then
if (call BIS_fnc_isCampaign) then {
if (isNil var || {!((missionNamespace getVariable var) isEqualType {})}) then { \
incommingMisslieDetectionSystem = 8+16;
init = "if (local (_this select 0)) then {[(_this select 0), [], []] call BIS_fnc_unitHeadgear;}";\
INIT_DISPLAY(GVARMAIN(RscDisplayAttributesModuleAIGear),ADDON)
initAngleX=0; minAngleX=-30; maxAngleX=+30;
initCargoAngleY=+185; // cargo viewing limitations
initFov = 0.7/1.5;
intensity = .8;
INVENTORY_PICTURE(rhs_6b23)
items[] = {"FirstAidKit","ACE_Clacker","ACE_DefusalKit"}; \
landingAoa            		= 5*3.1415/180; // landing angle of attack in radians, for AI
limitsArrayTop[] = {{45, -85}, , {35, -17},{15, -15}};
localize "str_channel_global", \
maxAngle	= -360-180;
maxAngleX=+30;
maximumLoad = LOADEMPTY;									\
maxTurn=+360;
Mike26Compositions = BIS_inf execVM "Composition.sqf";
minValue = rad 0;//
model				= \rhsafrf\addons\rhs_a2port_air\data\rhs_pylon_g_upk23_ka52;\
model				= \rhsafrf\addons\rhs_airweapons\rhs_m_ptab25m;
model	= "\rhsusf\addons\rhsusf_a2port_air\a10\A10.p3d";
model 				= \A3\Weapons_F\DynamicLoadout\PylonPod_Twin_Cannon_20mm.p3d;
model 				= rhsgref\addons\rhsgref_airweapons\zt3\rhs_pylon_m_zt3_launcher.p3d;
MOI							= .796;
name = $STR_A3_CfgGroups_East0;
name = $STR_A3_CfgGroups_West_BLU_F_Infantry_BUS_InfSquad0; // Rifle Squad
name = QUOTE(x);\
name = Rifle Squad (CAMO_NAME);
osdLocation = $STR_A3_campmaxwell0;
PACKAGE_VEH(m242,autocannon)
PACKAGE(ak74,rifle_small,PREFIX)
PACKAGE(K98,rifle2)
PACKAGE(M240,MMG1)
PACKAGE(m590,MMG1)
PACKAGE(M70,MMG1)
PACKAGE(M84,MMG1)
PACKAGE(MG42_delayed,MMG1_delayed)
PACKAGE(MP7,pistol1)
PACKAGE(pk,rifle_med,PREFIX)
PACKAGE(pp2000,pistol1)
PACKAGE(pp2000,rifle_small,PREFIX)
PACKAGE(tokarev,Pistol1)
PACKAGELAYER(m24,rifle2,PREFIX)
PACKAGELAYER(m240,rifle_med,PREFIX)
PACKAGELAYER(m249,rifle_small,PREFIX)
PACKAGELAYER(mp7,rifle_small,PREFIX)
params[] = {0.15, .4, 1.57, 0.5, 0.5, 1};
params[] = {0.15, 0f.4, 1.57, 0.5, 0.5, 1};
parentTitle = $STR_A3_MP_GroundSupport_Task_Transport_Parent_Title;
picture = \CUP\Creatures\People\Military\CUP_Creatures_People_Military_CDF\data\ui\icon_h_cdf_helmet_mnt_ca.paa ;
picture = \CUP\Creatures\People\Military\CUP_Creatures_People_Military_USArmy\data\ui\icon_CUP_V_B_Ciras_Olive.paa;
PIP_BB(1)
player createDiaryRecord 
player createDiaryRecord [
PREP_FOLDER(radio);
PREP_FOLDER(vic3);
PREP_FOLDER(vrc64);
PREP_W_INTERCEPT(getSurfaceTexturePath,getSurfaceTexturePathNative,"surfaceTexture");
private _group = grpNull; \
private _position = [GVAR(mousePos), 2] call EFUNC(common,getPosFromScreen); \
private _return = [];\
private _z = random 2 - 1;\
private var = getText (cfg); \
private["_blocks","_block","_blockCount","_blockNr","_blockArray","_blockText","_blockTextF","_blockTextF_","_blockFormat","_formats","_inputData","_processedTextF","_char","_cursorBlinks","_cursorInvis"];
private["_variant"];
property = QEGVAR(fastroping,equipFRIES); \
QUOTE([ARR_2('ACE3 Common',QQEGVAR(interact_menu,selfInteractKey))] call FUNC(getKeybindName)), \
respawnWeapons[] = {"rhs_weap_m21a", STD_WEAP_GRUNT};\
RHA(100, 1000)
RHA(30,200)
rhs_30Rnd_545x39_AK,\
RHS_DLC
RHS_FUEL_RANGE(350)
rhs_mag_20Rnd_556x45_M193_Stanag,\
rhs_player_01 createDiaryRecord
RHS_SOUNDSET_RIFLE_SD(AK74)
RHS_SOUNDSHADER_RIFLE(AK74,3000)
RHS_TAILSETCONFIG_SD(Rifle1_SD) 
RHS_TAILSHADERCONFIG(Rifle1,3000)
RHS_TAILSOUNDSET_DEFAULT(pistol1,0.8)
RHS_TAILSOUNDSHADERS_DEFAULT(pistol1,1,150,2700,small,1.35,200)
RHS_WEAPONSOUNDSET_DEFAULT(AK74,BASEVOLRIFLE,BASEVOLCURVE,BASE3D,BASEFILTER)
RHS_WEAPONSOUNDSHADERS_DEFAULT(AK74,AK74,0.87,35,LMG545,1.23,AK74,0.54,120,rifle1,1,150,150,800,3000)
RHSGREF_TAILSOUNDSET_DEFAULT(pistol1,0.8)
RHSGREF_TAILSOUNDSHADERS_DEFAULT(pistol1,1,150,2700,small,1.35,200)
RHSGREF_WEAPONSOUNDSET_DEFAULT(M70,BASEVOLRIFLE,BASEVOLCURVE,BASE3D,BASEFILTER)
RHSGREF_WEAPONSOUNDSHADERS_DEFAULT(M70,AK74,0.925,35,LMG762rus,1.28,dp,0.54,120,mmg1,1,150,150,800,3000)
RHSSAF_TAILSOUNDSET_DEFAULT(pistol1,0.8)
RHSSAF_TAILSOUNDSHADERS_DEFAULT(pistol1,1,150,2700,small,1.35,200)
RHSSAF_WEAPONSOUNDSET_DEFAULT(M70,BASEVOLRIFLE,BASEVOLCURVE,BASE3D,BASEFILTER)
RHSSAF_WEAPONSOUNDSHADERS_DEFAULT(M70,AK74,0.925,35,LMG762rus,1.28,dp,0.54,120,mmg1,1,150,150,800,3000)
RHSUSF_SOUNDSET_RIFLE_SD(M4)
RHSUSF_SOUNDSHADER_RIFLE(MP7,2750)
RHSUSF_TAILSETCONFIG(pistol1) 
RHSUSF_TAILSHADERCONFIG(Rifle1,3000)
RHSUSF_TAILSOUNDSET_DEFAULT(pistol1,0.8)
RHSUSF_TAILSOUNDSHADERS_DEFAULT(pistol1,1,150,2700,small,1.35,200)
RHSUSF_WEAPONSOUNDSET_DEFAULT(M240,BASEVOLRIFLE,BASEVOLCURVE,BASE3D,BASEFILTER)
RHSUSF_WEAPONSOUNDSHADERS_DEFAULT(M240,M240,0.95,45,LMG762,1.24,M240,0.69,150,mmg1,1,150,150,800,3300)
scale		= 0.15 * (SafeZoneW Min SafeZoneH);
scope 				= 2;
scope = 0;
scope = 1;\
scope = 2;
scope = 2;\
scope=1;
scope=2;
scopeCurator = 2;\
SCRIPT_MACRO(2s1_ejection)
SCRIPT_MACRO(2s3_init)
SCRIPT_MACRO(AGM114_helper,rhs_fnc_agm114_helper)
SCRIPT_MACRO(t14_deleted)
SCRIPT_MACRO(t15_deleted)
SENS_RANGES_WVR(5000,5000)
side	= TWest;\
side = 0;
side = 0;\
side = 1;
side = 2;
side=0;\
side=1;\
side=2;\
side=sideN; \
SizeEx = 1.5*0.02300;
Smolko[]	= { ##VOICE_TYPE##\020_Names\Smolko.ogg }; \
sound[] = {"rhsafrf\addons\rhs_vehiclesounds\sounds\air\Tu95\tu95_ext_high",db+10.5,1.2};
SOUNDSET_DEF(VEH_NAME,MOD_TAG,CAMPOS,RPM0)
statement = QUOTE([ARR_2(_target,0)] call FUNC(displayPatientInformation));
style = 0x00;
style = 0x01;
style = 0x02;
style = 32 + 16; // LB_MULTI + ST_MULTI
style = 48 + 2048;
style = 48 + 2048; // picture + keep aspect ratio
style = ST_MULTI + ST_CENTER;
style=0x02; // Center
TAILSOUNDSET_DEFAULT(rifle_small,1.55,PREFIX)
TAILSOUNDSHADERS_DEFAULT(rifle_small,rifle_small,1.15,1.15,250,3000,FILEPATH,PREFIX)
TAILSOUNDSHADERS_DEFAULT(rifle_small,rifle_small,1.15,1.15,250,3000,FILEPATH,PREFIX)    
tankTurnForceAngMinSpd	= __EVAL(45*(PI/180)));	// in rad/s, speed where tankTurnForce starts fading to 0 @ tankTurnForceAngSpd
targets append _groups;\
TEXT_ROW_LARGE(21, ROW_LARGE_X, ROW_LARGE_Y, 0, ROW_LARGE_W, ROW_LARGE_H, ROW_LARGE_OFFX, ROW_LARGE_OFFY, ROW_LARGE_FONT_SIZE)
TEXT_SIZEPOS(0.05,-0.1,FONT_SIZE,FONT_SIZE_Y)
TEXT_SIZEPOS(0.75,0.060,FONT_SIZE,FONT_SIZE)
textureChecked           = checkedTexture; \
textures[]=
title = $STR_A3_C_in1_notifBriefingNotification_title;
title = $STR_A3_category_welcome;
title = $STR_A3_CfgTaskDescriptions_Ping;
title = $STR_A3_POI_Abandoned_Battlefield_01_Task_name;
title = $STR_A3_VR_heli_basics_title;
title = $STR_A3_VR_WH1_title;
titles[] = {$STR_A3_c_in1_20_broadcast_SPE_0};
TRACE_1("Starting preload",class);\
type = 6 * WeaponSlotItem;
UNITTYPE_UNARMED;
url = "http://www.rhsmods.org/";
VCLASS
vehicleClass = "rhs_vehclass_infantry_emr";\
vehicleClass = "rhs_vehclass_infantry_flora";\
vehicleClass = "rhs_vehclass_infantry_recon";\
vehicleClass = "rhs_vehclass_infantry_ucp_arb";\
vehicleClass = "rhs_vehclass_lar";\
vehicleClass = "rhssaf_vehclass_cars";\
vehicleClass="CUP_B_Men_USMC_FRDST"; \
vehicleClass="CUP_B_Men_USMC_FROGWDL"; \
volume = rain*(1-camPos);
volume = VOL_IDLE * 0.25;
w = (6/10) * TMF_ADMINMENU_RESP_W_COL1;
w = (ATTRIBUTE_TITLE_W + ATTRIBUTE_CONTENT_W) * GRID_W;
w = POS_W(13.8) - pixelW;
w = POS_W(26) - pixelW;
w = safeZoneXAbs - safeZoneX;
waitUntil{!(isNil "BIS_fnc_camp_setTimeWeather__dateWeatherSet") && {BIS_fnc_camp_setTimeWeather__dateWeatherSet}};
weapon = #WEAPON; \
weaponLockSystem		= CM_Lock_IR + CM_Missile;
weapons[] = {"CUP_arifle_XM8_Carbine","CUP_hgun_glock17_flashlight","Throw","Put"}; \
weapons[] = {"rhs_weap_m21a", STD_WEAP_GRUNT};\
weaponsGroup1 = WEAPONGROUP_CANNONS + WEAPONGROUP_MGUNS;
WEAPONSOUNDSET_DEFAULT(m240,BASEVOLRIFLE,BASEVOLCURVE,BASE3D,BASEFILTER,PREFIX)
WEAPONSOUNDSET_DEFAULT(pk,BASEVOLRIFLE,BASEVOLCURVE,BASE3D,BASEFILTER,PREFIX)
WEAPONSOUNDSHADERS_DEFAULT(m240,762mm_mmg,rifle,1.0,1,17,130,750,3300,rifle2,1,150,1200,FILEPATH,PREFIX)
WEAPONSOUNDSHADERS_DEFAULT(pk,762mm_mmg,rifle,1.0,1,17,130,750,3300,rifle_small,1,150,1200,FILEPATH,PREFIX)
weight				= 0.385*230;
wheelPointRejectAngle       = Pi/4.0;
x			= 11.0 *   (0.01875 * Safezoneh);
x			= 3.7 *   (0.01875 * Safezoneh);
x			= 3.75 *   (0.01875 * Safezoneh);
x			= 8.89 *   (0.01875 * Safezoneh);
x	= 0.45 * safezoneW + safezoneX;
x = -0.5 * GUI_GRID_W + GUI_GRID_X;
x = (0.50-0.5/2) * safezoneW + safezoneX;
x = (MSGBOX_X);
x = 0 * safezoneW + safezoneX;
x = 0 * TMF_ADMINMENU_STD_WIDTH;
x = 0.182689 * safezoneW + safezoneX;
x = 24.5 *   (0.01875 * Safezoneh);
x = 7 * GUI_GRID_W + GUI_GRID_CENTER_X;
x = 7 * GUI_GRID_W + GUI_GRID_X;
x = CENTER_X - (PICTURE_W + GRID_W(5)) / 2;
x = CENTER_X - 25 * GRID_W;
x = QUOTE(profileNamespace getVariable [ARR_2('TRIPLES(IGUI,GVAR(volumeControl),X)',VOLUME_CONTROL_DEFAULT_X)]); \
x = safeZoneWAbs / 2 - POS_W(13.5);
x = safeZoneX + POS_EDGE(12.5,11) * GUI_GRID_W;
x = safezoneX-border_offsetX; w = column_weight*3 + border_offsetX*4;
x =-3.54 *   (0.01875 * Safezoneh);
x =13.4 *   (0.01875 * Safezoneh);
x =14.95 *   (0.01875 * Safezoneh);
x =16.98 *   (0.01875 * Safezoneh);
x =17.65 *   (0.01875 * Safezoneh);
x =17.95 *   (0.01875 * Safezoneh);
x =18.05 *   (0.01875 * Safezoneh);
x =18.65 *   (0.01875 * Safezoneh);
x =25.85 *   (0.01875 * Safezoneh);
x =26.4 *   (0.01875 * Safezoneh);
x =3.65 *   (0.01875 * Safezoneh);
x =4.41 *   (0.01875 * Safezoneh);
x =8.89 *   (0.01875 * Safezoneh);
x=__EVAL(##X##); \
y = 0 * SIZE_M * GRID_H;
y = safeZoneY + POS_H(3.3);