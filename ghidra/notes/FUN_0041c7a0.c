
undefined4 __thiscall FUN_0041c7a0(void *this,uint *param_1)

{
  ushort uVar1;
  int iVar2;
  bool bVar3;
  undefined3 extraout_var;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  int iVar9;
  int local_10;
  int local_c;
  
  if ((param_1 != (uint *)0x0) &&
     (bVar3 = FUN_005fc040((int *)param_1), CONCAT31(extraout_var,bVar3) != 0)) {
    uVar1 = *(ushort *)((int)this + 0x1c);
    uVar4 = FUN_005fc0e0((int *)param_1);
    if (uVar4 != uVar1) {
      return 0;
    }
    uVar1 = *(ushort *)((int)this + 0x1e);
    uVar4 = FUN_005fc0f0((int *)param_1);
    if (uVar4 != uVar1) {
      return 0;
    }
    uVar4 = FUN_005fc100(param_1);
    if (uVar4 != 8) {
      return 0;
    }
  }
  iVar5 = FUN_005fc110((int *)param_1);
  if (iVar5 != 0) {
    return 0;
  }
  iVar5 = FUN_00583c40((int)param_1);
  uVar4 = FUN_005fc9c0(param_1);
  FUN_005fc0e0((int *)param_1);
  FUN_005fc0f0((int *)param_1);
  local_c = 0;
  if (0 < *(short *)((int)this + 0x1e)) {
    do {
      bVar3 = true;
      iVar2 = *(int *)(*(int *)((int)this + 0x30) + local_c * 4);
      iVar6 = FUN_005fc9e0(param_1,local_c);
      iVar9 = 0;
      iVar6 = iVar6 * uVar4 + iVar5;
      local_10 = 0;
      if (0 < *(short *)((int)this + 0x1c)) {
        do {
          uVar8 = (uint)*(byte *)(iVar9 + iVar2);
          iVar9 = iVar9 + 1;
          if (bVar3) {
            bVar3 = false;
          }
          else {
            iVar7 = 0;
            if (uVar8 != 0) {
              do {
                *(char *)(iVar7 + iVar6) =
                     *(char *)(iVar7 + iVar6) + *(char *)(iVar7 + iVar9 + iVar2);
                iVar7 = iVar7 + 1;
              } while (iVar7 < (int)uVar8);
            }
            iVar9 = iVar9 + uVar8;
            bVar3 = true;
          }
          iVar6 = iVar6 + uVar8;
          local_10 = local_10 + uVar8;
        } while (local_10 < *(short *)((int)this + 0x1c));
      }
      local_c = local_c + 1;
    } while (local_c < *(short *)((int)this + 0x1e));
  }
  return 1;
}

