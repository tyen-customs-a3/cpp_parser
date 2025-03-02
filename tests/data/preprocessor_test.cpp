class CfgPatches {
#if __has_include("\z\ace\addons\compat_gm\script_component.hpp")
    
#else
  class ace_compat_gm {
    ammo[] = {};
    magazines[] = {};
    units[] = {};
    weapons[] = {};
    requiredVersion = 0.1;
    author = "ACE Team";
    name = "Ace gm not compat";
    requiredAddons[] = {"ace_common", "gm_core"};
    skipWhenMissingDependencies = 1;
    version = 3.16;
    versionStr = "3.16.3.79";
    versionAr[] = {3,16,3,79};
  };
#endif
}; 