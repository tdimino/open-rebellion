
int __cdecl FUN_00516360(uint *param_1,undefined4 param_2,int param_3,undefined4 *param_4)

{
  int *piVar1;
  uint uVar2;
  int iVar3;
  uint *puVar4;
  int iVar5;
  bool bVar6;
  bool bVar7;
  int local_18;
  uint *local_14;
  uint local_10;
  uint local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  iVar5 = param_3;
  puVar4 = param_1;
  bVar6 = param_3 == 1;
  bVar7 = param_3 == 3;
  local_18 = 0;
  local_10 = (uint)bVar6;
  local_c = (uint)bVar7;
  piVar1 = (int *)FUN_00505b20(param_1);
  if (((param_3 < 1) || (4 < param_3)) || (param_3 = 1, piVar1 == (int *)0x0)) {
    param_3 = 0;
  }
  iVar3 = param_3;
  if (((piVar1 != (int *)0x0) && (0 < iVar5)) && ((iVar5 < 5 && ((bVar6 != 0 || (bVar7 != 0)))))) {
    param_1 = (uint *)0x0;
    local_14 = (uint *)0x0;
    if (((byte)piVar1[9] & 0xc0) != 0xc0) {
      uVar2 = (uint)piVar1[9] >> 6 & 3;
      if (uVar2 == 1) {
        iVar5 = 1;
      }
      else if (uVar2 == 2) {
        iVar5 = 2;
      }
      else {
        iVar5 = 0;
      }
      param_1 = (uint *)FUN_00506f30(iVar5);
      local_14 = (uint *)FUN_00506f30(2 - (uint)(iVar5 != 1));
    }
    if ((param_1 != (uint *)0x0) && (local_14 != (uint *)0x0)) {
      local_8 = 0x2c;
      local_4 = 0x2d;
      if ((*puVar4 >> 0x18 < 0x2c) || (0x2c < *puVar4 >> 0x18)) {
        bVar6 = false;
      }
      else {
        bVar6 = true;
      }
      FUN_00619730();
      if (bVar6) {
        if (local_10 != 0) {
          iVar5 = FUN_00530650(piVar1,param_4);
          if ((iVar5 == 0) || (param_3 == 0)) {
            param_3 = 0;
          }
          else {
            param_3 = 1;
          }
        }
        if (local_c != 0) {
          uVar2 = FUN_005166a0(piVar1,&local_18);
          if ((uVar2 == 0) || (param_3 == 0)) {
            bVar6 = false;
          }
          else {
            bVar6 = true;
          }
          puVar4 = local_14;
          if (local_18 == 0) {
            puVar4 = param_1;
          }
          iVar5 = FUN_00530670(puVar4,piVar1,param_4);
LAB_005165d1:
          if ((iVar5 != 0) && (bVar6)) {
            return 1;
          }
          return 0;
        }
      }
      else {
        local_8 = 0x2d;
        local_4 = 0x2e;
        if ((*puVar4 >> 0x18 < 0x2d) || (0x2d < *puVar4 >> 0x18)) {
          bVar6 = false;
        }
        else {
          bVar6 = true;
        }
        FUN_00619730();
        if (bVar6) {
          if (local_10 != 0) {
            uVar2 = FUN_005306b0(param_1,piVar1,param_4);
            if ((uVar2 == 0) || (param_3 == 0)) {
              param_3 = 0;
            }
            else {
              param_3 = 1;
            }
          }
          if (local_c != 0) {
            uVar2 = FUN_005166a0(piVar1,&local_18);
            if ((uVar2 == 0) || (param_3 == 0)) {
              bVar6 = false;
            }
            else {
              bVar6 = true;
            }
            puVar4 = local_14;
            if (local_18 == 0) {
              puVar4 = param_1;
            }
            iVar5 = FUN_005307e0(puVar4,piVar1,param_4);
            goto LAB_005165d1;
          }
        }
        else {
          local_8 = 0x28;
          local_4 = 0x2c;
          if ((*puVar4 >> 0x18 < 0x28) || (0x2b < *puVar4 >> 0x18)) {
            bVar6 = false;
          }
          else {
            bVar6 = true;
          }
          FUN_00619730();
          if (bVar6) {
            if ((local_10 != 0) &&
               ((uVar2 = FUN_00530820(param_1,piVar1,param_4), uVar2 == 0 ||
                (param_3 = 1, iVar3 == 0)))) {
              param_3 = 0;
            }
            iVar5 = param_3;
            if ((local_c != 0) &&
               ((iVar3 = FUN_00530950(piVar1,param_4), iVar3 == 0 || (param_3 = 1, iVar5 == 0)))) {
              param_3 = 0;
            }
          }
        }
      }
    }
  }
  return param_3;
}

