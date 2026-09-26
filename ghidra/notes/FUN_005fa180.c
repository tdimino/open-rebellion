
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int __thiscall FUN_005fa180(void *this,int param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  undefined4 local_50 [20];
  
  iVar5 = 3;
  if ((*(int *)((int)this + 0x108) != 0) && (param_1 != 0)) {
    uVar1 = *(undefined4 *)(param_1 + 0x1c);
    uVar2 = *(undefined4 *)(param_1 + 0x20);
    uVar3 = *(undefined4 *)(param_1 + 0x24);
    uVar4 = *(undefined4 *)(param_1 + 0x28);
    iVar5 = FUN_00583c40(param_1 + 0x2c);
    FUN_005fa2b0(this,local_50,uVar1,uVar2,uVar3,uVar4,DAT_006be3b4,iVar5);
    iVar5 = (**(code **)(**(int **)((int)this + 0x108) + 0x60))
                      (*(int **)((int)this + 0x108),local_50,1);
    if (iVar5 != 0) {
      FUN_005f8600(this);
      FUN_005ae1e0();
      return (iVar5 == -0x7788fee8) + 7;
    }
    _DAT_006be380 = *(undefined4 *)(param_1 + 0x1c);
    _DAT_006be384 = *(undefined4 *)(param_1 + 0x20);
    _DAT_006be388 = *(undefined4 *)(param_1 + 0x24);
    _DAT_006be38c = *(undefined4 *)(param_1 + 0x28);
    iVar5 = FUN_005fa270(this);
  }
  return iVar5;
}

