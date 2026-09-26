
uint __thiscall FUN_005d1240(void *this,uint param_1,int param_2,int param_3)

{
  int iVar1;
  uint uVar2;
  ushort uVar3;
  void *this_00;
  undefined4 *puVar5;
  uint uVar6;
  int *piVar7;
  int iVar8;
  int *local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  uint uVar4;
  
  uVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654556;
  local_c = ExceptionList;
  piVar7 = (int *)0x0;
  local_1c = (int *)0x0;
  ExceptionList = &local_c;
  FUN_005f5060(param_1);
  uVar3 = FUN_005f50e0(param_1);
  uVar4 = (uint)uVar3;
  iVar8 = 1;
  if (0 < param_3) {
    do {
      if (iVar8 * iVar8 - uVar4 != 0 && (int)uVar4 <= iVar8 * iVar8) break;
      iVar8 = iVar8 + 1;
    } while (iVar8 <= param_3);
  }
  if (uVar4 < 2) {
    param_1 = 1;
  }
  else {
    param_1 = iVar8 - 1;
  }
  uVar6 = (param_3 - param_2) + 1;
  if ((int)uVar6 <= (int)param_1) {
    param_1 = uVar6;
  }
  param_3 = 0;
  param_2 = 0;
  for (iVar8 = FUN_005f5060(uVar2); iVar8 != 0; iVar8 = *(int *)(iVar8 + 0x10)) {
    iVar1 = *(int *)(iVar8 + 0x1c);
    if ((param_3 % (int)((longlong)(ulonglong)uVar4 / (longlong)(int)param_1) == 0) &&
       (param_2 < (int)param_1)) {
      param_2 = param_2 + 1;
      this_00 = (void *)FUN_00618b70(0xbc);
      piVar7 = (int *)0x0;
      local_4 = 0;
      if (this_00 != (void *)0x0) {
        piVar7 = FUN_005cbf50(this_00,*(uint *)(iVar1 + 0xec));
      }
      local_4 = 0xffffffff;
      if (local_1c == (int *)0x0) {
        local_1c = piVar7;
      }
      if (DAT_006bc4a8 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)FUN_00618b70(0x274);
        local_4 = 1;
        if (puVar5 == (undefined4 *)0x0) {
          DAT_006bc4a8 = (undefined4 *)0x0;
        }
        else {
          DAT_006bc4a8 = FUN_005af5f0(puVar5);
        }
      }
      local_4 = 0xffffffff;
      FUN_005afde0(piVar7);
      FUN_005ca4c0((void *)((int)this + 0x70),(int)piVar7);
      (**(code **)(*piVar7 + 8))(*(undefined4 *)(iVar1 + 0xec));
      *(int *)(iVar1 + 0x174) = piVar7[6];
    }
    else {
      (**(code **)(*piVar7 + 0x28))(*(undefined4 *)(iVar1 + 0xec));
    }
    param_3 = param_3 + 1;
  }
  ExceptionList = local_c;
  return (param_3 < 1) - 1 & param_1;
}

