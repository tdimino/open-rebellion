
void __fastcall FUN_0042a4d0(int param_1)

{
  int iVar1;
  void *this;
  int *this_00;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c55b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  PostMessageA(*(HWND *)(param_1 + 0x18),0x409,0,0);
  iVar1 = FUN_00604500((void *)(param_1 + 0x6c),0x16);
  if (iVar1 == 0) {
    this = (void *)FUN_00618b70(0x180);
    uStack_4 = 0;
    if (this == (void *)0x0) {
      this_00 = (int *)0x0;
    }
    else {
      this_00 = FUN_0046cbc0(this,*(undefined4 *)(param_1 + 0x1c),0,0,0x1d6,0x14a,param_1,0x16);
    }
    uStack_4 = 0xffffffff;
    if (this_00 != (int *)0x0) {
      this_00[0x14] = this_00[0x14] & 0xefffffff;
      FUN_005ffce0(this_00,0);
      FUN_005f4f10((void *)(param_1 + 0x6c),(int)this_00);
      (**(code **)(*this_00 + 0x2c))(5);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

