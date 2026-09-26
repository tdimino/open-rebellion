
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b1970(void *param_1,undefined4 param_2)

{
  undefined4 uVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  uint uVar5;
  float10 fVar6;
  longlong lVar7;
  
  *(int *)((int)param_1 + 0x158) = *(int *)((int)param_1 + 0x158) + 1;
  switch(param_2) {
  case 0:
    uVar2 = *(int *)((int)param_1 + 0x370) + 1;
    if (3 < uVar2) {
      uVar2 = 4;
    }
    *(uint *)((int)param_1 + 0x370) = uVar2;
    break;
  case 1:
    uVar2 = *(int *)((int)param_1 + 0x37c) + 1;
    if (3 < uVar2) {
      uVar2 = 4;
    }
    *(uint *)((int)param_1 + 0x37c) = uVar2;
    return;
  case 2:
    uVar2 = *(int *)((int)param_1 + 0x378) + 1;
    if (3 < uVar2) {
      uVar2 = 4;
    }
    *(uint *)((int)param_1 + 0x378) = uVar2;
    iVar3 = FUN_00597530();
    if ((((*(int *)(iVar3 + 0x668) == 0) && (*(int *)((int)param_1 + 0x124) == 0)) &&
        (*(int *)((int)param_1 + 0x380) != 0)) &&
       (fVar6 = FUN_005b1790(param_1), fVar6 == (float10)_DAT_0066c4f0)) {
      uVar1 = *(undefined4 *)((int)param_1 + 0xec);
      piVar4 = (int *)FUN_005aaf90();
      (**(code **)(*piVar4 + 0x28))(0x29,uVar1,*(undefined4 *)((int)param_1 + 0x380),1);
      *(undefined4 *)((int)param_1 + 0x3ac) = 1;
      return;
    }
    break;
  case 3:
    uVar2 = *(int *)((int)param_1 + 0x36c) + 1;
    if (3 < uVar2) {
      uVar2 = 4;
    }
    *(uint *)((int)param_1 + 0x36c) = uVar2;
    return;
  case 4:
    uVar2 = *(int *)((int)param_1 + 0x374) + 1;
    lVar7 = __ftol();
    uVar5 = (uint)lVar7;
    if (uVar2 < (uint)lVar7) {
      uVar5 = uVar2;
    }
    *(uint *)((int)param_1 + 0x374) = uVar5;
    return;
  }
  return;
}

