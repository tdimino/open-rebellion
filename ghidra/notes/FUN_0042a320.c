
void __thiscall FUN_0042a320(void *this,int *param_1)

{
  int iVar1;
  void *this_00;
  int *this_01;
  int aiStack_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c523;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  PostMessageA(*(HWND *)((int)this + 0x18),0x409,0,0);
  FUN_00526730(aiStack_1c);
  iStack_4 = 0;
  FUN_004f5380(param_1,aiStack_1c);
  iVar1 = thunk_FUN_005f5060((int)aiStack_1c);
  if (iVar1 == 0) {
    FUN_0041ce20(param_1,0);
  }
  else {
    this_00 = (void *)FUN_00618b70(0x174);
    iStack_4._0_1_ = 1;
    if (this_00 == (void *)0x0) {
      this_01 = (int *)0x0;
    }
    else {
      this_01 = (int *)FUN_0046a750(this_00,*(undefined4 *)((int)this + 0x1c),(int)this,0xc,param_1)
      ;
    }
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    if (this_01 != (int *)0x0) {
      FUN_005ffce0(this_01,0);
      FUN_005f4f10((void *)((int)this + 0x6c),(int)this_01);
      (**(code **)(*this_01 + 0x2c))(5);
    }
  }
  iStack_4 = 0xffffffff;
  FUN_00526770(aiStack_1c);
  ExceptionList = pvStack_c;
  return;
}

