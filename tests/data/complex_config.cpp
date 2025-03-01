////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Fri Feb 28 16:15:23 2025 : 'file' last modified on Wed Sep 04 21:24:41 2024
////////////////////////////////////////////////////////////////////

#define _ARMA_

//(23 Enums)
enum {
    _fnc_sizeex = 0,
    _fnc_colorhextorgba_4 = 0,
    _axiscolor1 = 0,
    destructengine = 2,
    _axiscolor2 = 0,
    destructdefault = 6,
    destructwreck = 7,
    _fnc_colorrgbatohex = 0,
    _fnc_colorhextorgba_6 = 0,
    _axiscolor3 = 0,
    destructtree = 3,
    destructtent = 4,
    stabilizedinaxisx = 1,
    stabilizedinaxesxyz = 4,
    stabilizedinaxisy = 2,
    _fnc_colorhextorgba = 0,
    stabilizedinaxesboth = 3,
    destructno = 0,
    stabilizedinaxesnone = 0,
    _fnc_colorhextorgba_0 = 0,
    destructman = 5,
    _fnc_colorhextorgba_2 = 0,
    destructbuilding = 1
};

class CfgPatches
{
    class A3_3DEN
    {
        author = "$STR_A3_Bohemia_Interactive";
        name = "Arma 3 Eden Update - Eden Editor";
        url = "https://www.arma3.com";
        requiredAddons[] = {"A3_Data_F_Exp_B"};
        requiredVersion = 0.1;
        units[] = {"Sphere_3DEN","SphereNoGround_3DEN"};
        weapons[] = {};
    };
};
class CfgAddons
{
    class PreloadAddons
    {
        class 3DEN
        {
            list[] = {"A3_3DEN","A3_3DEN_Language","3DEN"};
        };
    };
};
class RscText;
class RscTitle;
class RscListbox;
class RscControlsGroupNoScrollbars;
class RscPicture;
class RscButtonMenu;
class RscButtonMenuOK;
class RscButtonMenuCancel;
class RscStandardDisplay;
class RscVignette;
class RscMap;
class ctrlDefault
{
    access = 0;
    idc = -1;
    style = 0;
    default = 0;
    show = 1;
    fade = 0;
    blinkingPeriod = 0;
    deletable = 0;
    x = 0;
    y = 0;
    w = 0;
    h = 0;
    tooltip = "";
    tooltipMaxWidth = 0.5;
    tooltipColorShade[] = {0,0,0,1};
    tooltipColorText[] = {1,1,1,1};
    tooltipColorBox[] = {0,0,0,0};
    class ScrollBar
    {
        width = 0;
        height = 0;
        scrollSpeed = 0.06;
        arrowEmpty = "\a3\3DEN\Data\Controls\ctrlDefault\arrowEmpty_ca.paa";
        arrowFull = "\a3\3DEN\Data\Controls\ctrlDefault\arrowFull_ca.paa";
        border = "\a3\3DEN\Data\Controls\ctrlDefault\border_ca.paa";
        thumb = "\a3\3DEN\Data\Controls\ctrlDefault\thumb_ca.paa";
        color[] = {1,1,1,1};
    };
};