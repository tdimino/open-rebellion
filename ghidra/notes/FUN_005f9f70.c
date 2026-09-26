
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int __thiscall FUN_005f9f70(void *this,int param_1)

{
  int iVar1;
  undefined4 local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  
  iVar1 = 3;
  if (*(int *)((int)this + 0x108) != 0) {
    iVar1 = FUN_005fa2b0(this,&local_50,local_60,local_5c,local_58,local_54,DAT_006be3b4,param_1);
    if (iVar1 == 0) {
      iVar1 = (**(code **)(**(int **)((int)this + 0x108) + 0x60))
                        (*(int **)((int)this + 0x108),&local_50,2);
      if (iVar1 != 0) {
        FUN_005f8600(this);
        FUN_005ae1e0();
        return (iVar1 == -0x7788fee8) + 7;
      }
      _DAT_006be380 = local_54;
      _DAT_006be384 = local_50;
      _DAT_006be388 = uStack_4c;
      _DAT_006be38c = uStack_48;
      iVar1 = FUN_005fa270(this);
      if (iVar1 == 0) {
        *(undefined4 *)((int)this + 0x24) = 1;
      }
    }
  }
  return iVar1;
}

