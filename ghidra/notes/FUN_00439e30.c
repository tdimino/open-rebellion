
undefined4 __thiscall FUN_00439e30(void *this,int param_1)

{
  int iVar1;
  undefined4 uVar2;
  uint uVar3;
  
  iVar1 = 0;
  uVar2 = 0;
  if (*(int *)((int)this + 0x10) != 0) {
    if (param_1 == 0x115) {
      uVar3 = 0x15;
    }
    else {
      if (param_1 != 0x116) goto LAB_00439e59;
      uVar3 = 0x14;
    }
    iVar1 = FUN_005f5500((void *)((int)this + 0x18),uVar3);
  }
LAB_00439e59:
  if (iVar1 != 0) {
    iVar1 = FUN_005f4960((undefined4 *)(iVar1 + 0x1c));
    if (iVar1 != 2) {
      uVar2 = 1;
    }
  }
  return uVar2;
}

