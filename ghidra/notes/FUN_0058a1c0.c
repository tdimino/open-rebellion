
bool __thiscall FUN_0058a1c0(void *this,undefined4 param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  void *this_00;
  int aiStack_1c [2];
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064fc30;
  local_c = ExceptionList;
  bVar1 = true;
  if (*(int *)((int)this + 0x4c) != 0) {
    ExceptionList = &local_c;
    iVar2 = FUN_00520b90(*(int *)((int)this + 0x14));
    if (iVar2 != 0) {
      bVar1 = FUN_00520ad0(*(int *)((int)this + 0x14));
      iVar2 = (**(code **)(**(int **)((int)this + 0x14) + 0x1dc))
                        ((CONCAT31(extraout_var,bVar1) != 0) + '\x03',param_1);
      bVar1 = iVar2 != 0;
    }
    this_00 = (void *)((int)this + 8);
    FUN_00589310(local_14,this_00);
    iStack_4 = 0;
    iVar2 = FUN_00587f80(this_00,local_14,param_1);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00589190(aiStack_1c,this_00);
    iStack_4._0_1_ = 1;
    iVar2 = FUN_00587b70(this_00,aiStack_1c,param_1);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_005891d0(aiStack_1c);
    iStack_4 = 0xffffffff;
    FUN_00589350(local_14);
  }
  ExceptionList = local_c;
  return bVar1;
}

