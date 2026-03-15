
void __thiscall FUN_0042a640(void *this,undefined4 param_1)

{
  int iVar1;
  void *this_00;
  int *this_01;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c57b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  PostMessageA(*(HWND *)((int)this + 0x18),0x409,0,0);
  iVar1 = FUN_00604500((void *)((int)this + 0x6c),7);
  if (iVar1 != 0) {
    FUN_00600f90(this,*(uint *)(iVar1 + 0x24));
  }
  this_00 = (void *)FUN_00618b70(400);
  uStack_4 = 0;
  if (this_00 == (void *)0x0) {
    this_01 = (int *)0x0;
  }
  else {
    this_01 = (int *)FUN_0046f140(this_00,*(undefined4 *)((int)this + 0x1c),(int)this,8,param_1);
  }
  uStack_4 = 0xffffffff;
  FUN_005ffce0(this_01,0);
  (**(code **)(*this_01 + 0x2c))(5);
  FUN_005f4f10((void *)((int)this + 0x6c),(int)this_01);
  SetFocus((HWND)this_01[6]);
  ExceptionList = this_00;
  return;
}

