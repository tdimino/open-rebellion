
uint __thiscall FUN_005d1080(void *this,uint param_1,uint param_2,uint param_3)

{
  int iVar1;
  uint uVar2;
  ushort uVar3;
  int iVar5;
  void *this_00;
  undefined4 *puVar6;
  int *piVar7;
  uint uVar8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  uint uVar4;
  
  uVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654536;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5060(param_1);
  uVar3 = FUN_005f50e0(param_1);
  uVar4 = (uint)uVar3;
  uVar8 = 1;
  if (param_3 != 0) {
    do {
      if (uVar4 <= uVar8 * uVar8 && uVar8 * uVar8 - uVar4 != 0) break;
      uVar8 = uVar8 + 1;
    } while (uVar8 <= param_3);
  }
  if (uVar4 < 2) {
    param_1 = 1;
  }
  else {
    param_1 = uVar8 - 1;
  }
  uVar8 = (param_3 - param_2) + 1;
  if (uVar8 <= param_1) {
    param_1 = uVar8;
  }
  piVar7 = (int *)0x0;
  param_3 = 0;
  param_2 = 0;
  for (iVar5 = FUN_005f5060(uVar2); iVar5 != 0; iVar5 = *(int *)(iVar5 + 0x10)) {
    iVar1 = *(int *)(iVar5 + 0x1c);
    if (((int)param_3 % (int)(uVar4 / param_1) == 0) && (param_2 < param_1)) {
      param_2 = param_2 + 1;
      this_00 = (void *)FUN_00618b70(0xbc);
      piVar7 = (int *)0x0;
      local_4 = 0;
      if (this_00 != (void *)0x0) {
        piVar7 = FUN_005cbf50(this_00,*(uint *)(iVar1 + 0xec));
      }
      local_4 = 0xffffffff;
      if (DAT_006bc4a8 == (undefined4 *)0x0) {
        puVar6 = (undefined4 *)FUN_00618b70(0x274);
        local_4 = 1;
        if (puVar6 == (undefined4 *)0x0) {
          DAT_006bc4a8 = (undefined4 *)0x0;
        }
        else {
          DAT_006bc4a8 = FUN_005af5f0(puVar6);
        }
      }
      local_4 = 0xffffffff;
      FUN_005afde0(piVar7);
      *(int *)(iVar1 + 0x174) = piVar7[6];
      FUN_005ca4c0((void *)((int)this + 0x70),(int)piVar7);
      (**(code **)(*piVar7 + 8))(*(undefined4 *)(iVar1 + 0xec));
    }
    else {
      (**(code **)(*piVar7 + 0x28))(*(undefined4 *)(iVar1 + 0xec));
    }
    param_3 = param_3 + 1;
  }
  ExceptionList = local_c;
  return ((int)param_3 < 1) - 1 & param_1;
}

