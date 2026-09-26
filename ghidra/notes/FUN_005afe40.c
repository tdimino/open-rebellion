
int __thiscall FUN_005afe40(void *this,int param_1)

{
  void *this_00;
  int *piVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  uint uVar5;
  int local_4;
  
  local_4 = 0;
  if (*(int *)(param_1 + 0x170) != 2) {
    return 0;
  }
  uVar5 = *(uint *)(param_1 + 0x164);
  if (uVar5 != 0) {
    this_00 = (void *)FUN_00596bd0();
    piVar1 = FUN_005a6360(this_00,uVar5);
    if (piVar1 != (int *)0x0) {
      FUN_00596f30();
      iVar2 = FUN_00596bd0();
      iVar2 = FUN_005a66a0(iVar2);
      if (iVar2 == 0) {
        piVar4 = (int *)0x0;
      }
      else {
        piVar4 = (int *)(iVar2 + 0x1c);
      }
      if (piVar4 == piVar1) {
        iVar2 = FUN_005ba420(iVar2);
        if (iVar2 != 1) goto LAB_005afee6;
        iVar2 = FUN_005b84a0(piVar1,param_1);
      }
      else {
        iVar2 = FUN_005afc80(this,(int)(piVar1 + -0x106));
        if (iVar2 != 1) goto LAB_005afee6;
        iVar2 = FUN_005b84a0(piVar1,param_1);
      }
      if (iVar2 == 1) {
        local_4 = 1;
      }
    }
  }
LAB_005afee6:
  if (local_4 == 0) {
    for (piVar1 = (int *)(**(code **)(*(int *)(*(int *)((int)this +
                                                       *(int *)(param_1 + 0x168) * 4 + 0x24c) + 0x58
                                              ) + 0x10))(); piVar1 != (int *)0x0;
        piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      iVar3 = FUN_005afc80(this,iVar2);
      if ((iVar3 == 1) && (iVar2 = FUN_005b84a0((void *)(iVar2 + 0x418),param_1), iVar2 == 1)) {
        local_4 = 1;
        break;
      }
    }
    if ((local_4 == 0) && (*(int *)(param_1 + 0x168) == 1)) {
      FUN_00596f30();
      iVar2 = FUN_00596bd0();
      iVar2 = FUN_005a66a0(iVar2);
      if ((iVar2 != 0) && (iVar3 = FUN_005ba420(iVar2), iVar3 != 0)) {
        local_4 = FUN_005b84a0((void *)(iVar2 + 0x1c),param_1);
      }
    }
  }
  if (local_4 == 1) {
    *(undefined4 *)(param_1 + 0x170) = 5;
    FUN_005c60d0((void *)(*(int *)((int)this + *(int *)(param_1 + 0x168) * 4 + 0x24c) + 100),
                 *(uint *)(param_1 + 0xec));
  }
  return local_4;
}

